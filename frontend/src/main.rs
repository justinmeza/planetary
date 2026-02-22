mod content;

use rpc::{client, Request};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const LISTEN_ADDR: &str = "127.0.0.1:8080";

// Service addresses
const DISCOVERY_ADDR: &str = "127.0.0.1:10200";
const CONFIGURATION_ADDR: &str = "127.0.0.1:10500";
const STORAGE_ADDR: &str = "127.0.0.1:10600";
const CACHING_ADDR: &str = "127.0.0.1:10700";
const MONITORING_ADDR: &str = "127.0.0.1:10800";
const SCHEDULING_ADDR: &str = "127.0.0.1:10900";
const RELEASE_ADDR: &str = "127.0.0.1:11000";
const SECURITY_ADDR: &str = "127.0.0.1:11100";
const LOADBALANCER_ADDR: &str = "127.0.0.1:8080";

static USER_COUNTER: AtomicU64 = AtomicU64::new(1);

// ── Base64 encode/decode (inline, no external crate) ────────────────────────

const B64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn base64_encode(input: &[u8]) -> String {
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(B64_CHARS[((triple >> 18) & 0x3F) as usize] as char);
        out.push(B64_CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            out.push(B64_CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(B64_CHARS[(triple & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

fn base64_decode(input: &str) -> Vec<u8> {
    fn val(c: u8) -> u32 {
        match c {
            b'A'..=b'Z' => (c - b'A') as u32,
            b'a'..=b'z' => (c - b'a' + 26) as u32,
            b'0'..=b'9' => (c - b'0' + 52) as u32,
            b'+' => 62,
            b'/' => 63,
            _ => 0,
        }
    }
    let bytes: Vec<u8> = input.bytes().filter(|&b| b != b'=' && b != b'\n' && b != b'\r').collect();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    for chunk in bytes.chunks(4) {
        if chunk.len() < 2 { break; }
        let a = val(chunk[0]);
        let b = val(chunk[1]);
        let c = if chunk.len() > 2 { val(chunk[2]) } else { 0 };
        let d = if chunk.len() > 3 { val(chunk[3]) } else { 0 };
        let triple = (a << 18) | (b << 12) | (c << 6) | d;
        out.push(((triple >> 16) & 0xFF) as u8);
        if chunk.len() > 2 { out.push(((triple >> 8) & 0xFF) as u8); }
        if chunk.len() > 3 { out.push((triple & 0xFF) as u8); }
    }
    out
}

// ── RPC helpers ─────────────────────────────────────────────────────────────

async fn send(addr: &str, procedure_id: i32, payload: String) -> String {
    let request = Request {
        procedure_id,
        payload,
    };
    match client::send_request(addr, request).await {
        Ok(response) => response.payload,
        Err(e) => format!("ERROR: {}", e),
    }
}

// ── HTTP parsing helpers ────────────────────────────────────────────────────

fn parse_form(body: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in body.split('&') {
        if let Some(idx) = pair.find('=') {
            let key = url_decode(&pair[..idx]);
            let value = url_decode(&pair[idx + 1..]);
            map.insert(key, value);
        }
    }
    map
}

fn url_decode(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        match c {
            '+' => result.push(' '),
            '%' => {
                let hex: String = chars.by_ref().take(2).collect();
                if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                    result.push(byte as char);
                }
            }
            _ => result.push(c),
        }
    }
    result
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn parse_query_string(path: &str) -> (&str, HashMap<String, String>) {
    if let Some(idx) = path.find('?') {
        let base = &path[..idx];
        let qs = &path[idx + 1..];
        (base, parse_form(qs))
    } else {
        (path, HashMap::new())
    }
}

fn parse_cookie(headers: &str, name: &str) -> Option<String> {
    for line in headers.lines() {
        let lower = line.to_lowercase();
        if lower.starts_with("cookie:") {
            let val = &line[7..];
            for part in val.split(';') {
                let part = part.trim();
                if let Some(idx) = part.find('=') {
                    if &part[..idx] == name {
                        return Some(part[idx + 1..].to_string());
                    }
                }
            }
        }
    }
    None
}

fn get_or_create_user_id(headers: &str) -> (String, bool) {
    if let Some(uid) = parse_cookie(headers, "user_id") {
        if !uid.is_empty() {
            return (uid, false);
        }
    }
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let counter = USER_COUNTER.fetch_add(1, Ordering::Relaxed);
    (format!("u_{}_{}", ts, counter), true)
}

// ── Spectral color map ──────────────────────────────────────────────────────

struct NavItem {
    href: &'static str,
    label: &'static str,
    color: &'static str,
}

enum NavEntry {
    Part(&'static str),
    Chapter(NavItem),
    Separator,
}

fn book_nav_entries() -> Vec<NavEntry> {
    vec![
        NavEntry::Chapter(NavItem { href: "/foreword", label: "Foreword", color: "#888" }),
        NavEntry::Chapter(NavItem { href: "/preface", label: "Preface", color: "#888" }),
        NavEntry::Part("Part I: Fundamentals"),
        NavEntry::Chapter(NavItem { href: "/chapter/systems", label: "1. Systems", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/design", label: "2. Design", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/consensus", label: "3. Consensus", color: "#06D6A0" }),
        NavEntry::Chapter(NavItem { href: "/chapter/configuration", label: "4. Configuration", color: "#3A86FF" }),
        NavEntry::Chapter(NavItem { href: "/chapter/discovery", label: "5. Discovery", color: "#F7B731" }),
        NavEntry::Chapter(NavItem { href: "/chapter/routing", label: "6. Routing", color: "#2A9D8F" }),
        NavEntry::Chapter(NavItem { href: "/chapter/caching", label: "7. Caching", color: "#7209B7" }),
        NavEntry::Chapter(NavItem { href: "/chapter/storage", label: "8. Storage", color: "#5E60CE" }),
        NavEntry::Chapter(NavItem { href: "/chapter/implementation", label: "9. Implementation", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/operation", label: "10. Operation", color: "#555" }),
        NavEntry::Part("Part II: Management"),
        NavEntry::Chapter(NavItem { href: "/chapter/scheduling", label: "11. Scheduling", color: "#FF6B35" }),
        NavEntry::Chapter(NavItem { href: "/chapter/release", label: "12. Release", color: "#4CC9F0" }),
        NavEntry::Chapter(NavItem { href: "/chapter/security", label: "13. Security", color: "#D62828" }),
        NavEntry::Chapter(NavItem { href: "/chapter/monitoring", label: "14. Monitoring", color: "#B5179E" }),
        NavEntry::Chapter(NavItem { href: "/chapter/capacity", label: "15. Capacity", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/utilization", label: "16. Utilization", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/efficiency", label: "17. Efficiency", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/load-testing", label: "18. Load Testing", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/planning", label: "19. Planning", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/degradation", label: "20. Degradation", color: "#555" }),
        NavEntry::Part("Part III: Distribution"),
        NavEntry::Chapter(NavItem { href: "/chapter/load-balancing", label: "21. Load Balancing", color: "#1B998B" }),
        NavEntry::Chapter(NavItem { href: "/chapter/consistency", label: "22. Consistency", color: "#A855F7" }),
        NavEntry::Chapter(NavItem { href: "/chapter/placement", label: "23. Placement", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/global-distribution", label: "24. Global Distribution", color: "#10B981" }),
        NavEntry::Chapter(NavItem { href: "/chapter/traffic", label: "25. Traffic", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/faults", label: "26. Faults", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/outages", label: "27. Outages", color: "#555" }),
        NavEntry::Part("Part IV: Infrastructure"),
        NavEntry::Chapter(NavItem { href: "/chapter/resources", label: "28. Resources", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/servers", label: "29. Servers", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/buildings", label: "30. Buildings", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/network", label: "31. Network", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/power", label: "32. Power", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/infra-management", label: "33. Management", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/maintenance", label: "34. Maintenance", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/edges", label: "35. Edges", color: "#555" }),
        NavEntry::Part("Part V: Incident Management"),
        NavEntry::Chapter(NavItem { href: "/chapter/site-events", label: "36. Site Events", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/detection", label: "37. Detection", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/escalation", label: "38. Escalation", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/root-causes", label: "39. Root Causes", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/remediation", label: "40. Remediation", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/prevention", label: "41. Prevention", color: "#555" }),
        NavEntry::Chapter(NavItem { href: "/chapter/communication", label: "42. Communication", color: "#555" }),
        NavEntry::Separator,
        NavEntry::Chapter(NavItem { href: "/afterword", label: "Afterword", color: "#888" }),
        NavEntry::Chapter(NavItem { href: "/colophon", label: "Colophon", color: "#888" }),
    ]
}

// ── Page templates ──────────────────────────────────────────────────────────

fn book_page(title: &str, slug: &str, active_href: &str, chapter_content: &str) -> String {
    let nav_html: String = book_nav_entries()
        .iter()
        .map(|entry| match entry {
            NavEntry::Part(label) => format!(
                r#"<div class="nav-part">{}</div>"#,
                label
            ),
            NavEntry::Separator => r#"<hr class="nav-sep">"#.to_string(),
            NavEntry::Chapter(item) => {
                let active = if item.href == active_href { " class=\"nav-active\"" } else { "" };
                format!(
                    r#"<a href="{}" {}><span class="nav-dot" style="background:{};"></span>{}</a>"#,
                    item.href, active, item.color, item.label
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n            ");

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{title} - The Planetary Scale Computer</title>
    <meta name="description" content="{title} — The Planetary Scale Computer">
    <link rel="canonical" href="https://p.jjm.net{active_href}">
    <meta property="og:title" content="{title} - The Planetary Scale Computer">
    <meta property="og:description" content="{title} — The Planetary Scale Computer">
    <meta property="og:type" content="article">
    <meta property="og:url" content="https://p.jjm.net{active_href}">
    <meta property="og:site_name" content="The Planetary Scale Computer">
    <meta name="twitter:card" content="summary">
    <meta name="twitter:title" content="{title} - The Planetary Scale Computer">
    <style>
        :root {{
            --bg: #fffff8;
            --text: #111;
            --sidebar-bg: #fafaf4;
            --sidebar-border: #e0dfd6;
            --sidebar-title: #555;
            --sidebar-link: #444;
            --sidebar-hover: #f0efe8;
            --sidebar-active: #eae9e0;
            --nav-part: #999;
            --nav-sep: #e8e7de;
            --dashboard-link: #888;
            --code-bg: #f4f3ee;
            --pre-bg: #f8f8f2;
            --pre-border: #ccc;
            --sidenote: #666;
            --sidenote-bg: #fafaf4;
            --sidenote-border: #ddd;
            --toolbar-bg: #2c3e50;
        }}
        @media (prefers-color-scheme: dark) {{
            :root {{
                --bg: #141412;
                --text: #ddd8cc;
                --sidebar-bg: #1c1c19;
                --sidebar-border: #2e2e28;
                --sidebar-title: #888;
                --sidebar-link: #bbb8b0;
                --sidebar-hover: #26261f;
                --sidebar-active: #2e2e26;
                --nav-part: #666;
                --nav-sep: #2a2a24;
                --dashboard-link: #666;
                --code-bg: #252520;
                --pre-bg: #1e1e1a;
                --pre-border: #44443a;
                --sidenote: #999;
                --sidenote-bg: #1c1c19;
                --sidenote-border: #33332c;
                --toolbar-bg: #0d1117;
            }}
        }}

        * {{ margin: 0; padding: 0; box-sizing: border-box; }}

        body {{
            font-family: Georgia, "Times New Roman", serif;
            background: var(--bg);
            color: var(--text);
            line-height: 1.7;
            font-size: 17px;
        }}

        /* ── Sidebar nav ────────────────────── */
        .sidebar {{
            position: fixed;
            top: 0; left: 0;
            width: 240px;
            height: 100vh;
            background: var(--sidebar-bg);
            border-right: 1px solid var(--sidebar-border);
            padding: 24px 16px;
            overflow-y: auto;
            z-index: 10;
        }}
        .sidebar .book-title {{
            font-size: 14px;
            font-variant: small-caps;
            letter-spacing: 0.05em;
            color: var(--sidebar-title);
            margin-bottom: 20px;
            padding-bottom: 12px;
            border-bottom: 1px solid var(--sidebar-border);
        }}
        .sidebar .book-title a {{
            color: inherit;
            text-decoration: none;
        }}
        .sidebar a {{
            display: flex;
            align-items: center;
            gap: 10px;
            padding: 6px 8px;
            margin: 2px 0;
            font-size: 13px;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            color: var(--sidebar-link);
            text-decoration: none;
            border-radius: 4px;
        }}
        .sidebar a:hover {{ background: var(--sidebar-hover); }}
        .sidebar a.nav-active {{ background: var(--sidebar-active); font-weight: 600; }}
        .nav-dot {{
            display: inline-block;
            width: 10px; height: 10px;
            border-radius: 50%;
            flex-shrink: 0;
        }}
        .sidebar .nav-part {{
            font-size: 11px;
            font-variant: small-caps;
            letter-spacing: 0.06em;
            color: var(--nav-part);
            margin: 16px 8px 4px;
            padding-top: 12px;
            border-top: 1px solid var(--nav-sep);
        }}
        .sidebar .nav-part:first-of-type {{
            border-top: none;
            margin-top: 8px;
        }}
        .sidebar hr.nav-sep {{
            border: none;
            border-top: 1px solid var(--sidebar-border);
            margin: 12px 8px;
        }}
        .sidebar .dashboard-link {{
            margin-top: 24px;
            padding-top: 12px;
            border-top: 1px solid var(--sidebar-border);
        }}
        .sidebar .dashboard-link a {{
            color: var(--dashboard-link);
            font-size: 12px;
        }}

        /* ── Main content ───────────────────── */
        .main {{
            margin-left: 240px;
            max-width: 740px;
            padding: 48px 32px 120px 48px;
        }}
        .chapter-content h1 {{
            font-size: 2em;
            margin: 0 0 24px 0;
            font-weight: 400;
            line-height: 1.2;
        }}
        .chapter-content h2 {{
            font-size: 1.4em;
            margin: 40px 0 16px 0;
            font-weight: 400;
            font-style: italic;
        }}
        .chapter-content h3 {{
            font-size: 1.1em;
            margin: 32px 0 12px 0;
            font-weight: 600;
        }}
        .chapter-content p {{
            margin-bottom: 16px;
        }}
        .chapter-content code {{
            font-family: "SF Mono", "Fira Code", "Fira Mono", Menlo, monospace;
            font-size: 0.85em;
            background: var(--code-bg);
            padding: 1px 5px;
            border-radius: 3px;
        }}
        .chapter-content pre {{
            margin: 20px 0;
            padding: 16px 20px;
            background: var(--pre-bg);
            border-radius: 4px;
            overflow-x: auto;
            line-height: 1.5;
            font-size: 0.82em;
            border-left: 4px solid var(--pre-border);
        }}
        .chapter-content pre code {{
            background: none;
            padding: 0;
            font-size: inherit;
        }}

        /* ── Spectral code-block borders ────── */
        pre.code-consensus {{ border-left-color: #06D6A0; }}
        pre.code-normalization {{ border-left-color: #E63946; }}
        pre.code-rpc {{ border-left-color: #F4845F; }}
        pre.code-discovery {{ border-left-color: #F7B731; }}
        pre.code-routing {{ border-left-color: #2A9D8F; }}
        pre.code-echo {{ border-left-color: #00B4D8; }}
        pre.code-configuration {{ border-left-color: #3A86FF; }}
        pre.code-storage {{ border-left-color: #5E60CE; }}
        pre.code-caching {{ border-left-color: #7209B7; }}
        pre.code-monitoring {{ border-left-color: #B5179E; }}
        pre.code-scheduling {{ border-left-color: #FF6B35; }}
        pre.code-release {{ border-left-color: #4CC9F0; }}
        pre.code-security {{ border-left-color: #D62828; }}
        pre.code-loadbalancer {{ border-left-color: #1B998B; }}
        pre.code-consistency {{ border-left-color: #A855F7; }}

        /* ── Tufte elements ─────────────────── */
        .newthought {{
            font-variant: small-caps;
            font-size: 1.1em;
            letter-spacing: 0.05em;
        }}
        .sys {{
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            font-size: 0.9em;
            text-decoration: none;
            white-space: nowrap;
        }}
        .sys::before {{
            content: '\25CF';
            font-size: 0.6em;
            margin-right: 3px;
            vertical-align: 1px;
        }}
        .sys:hover {{
            text-decoration: underline;
        }}
        .sidenote {{
            float: right;
            clear: right;
            margin-right: -240px;
            width: 200px;
            font-size: 0.8em;
            line-height: 1.4;
            color: var(--sidenote);
            padding-left: 12px;
        }}
        .attribution {{
            text-align: right;
            margin-top: 24px;
        }}

        /* ── Highlight toolbar ──────────────── */
        #highlight-toolbar {{
            display: none;
            position: absolute;
            z-index: 100;
            background: var(--toolbar-bg);
            border-radius: 24px;
            padding: 6px 10px;
            gap: 6px;
            box-shadow: 0 4px 16px rgba(0,0,0,0.3);
        }}
        #highlight-toolbar .hl-dot {{
            display: inline-block;
            width: 22px; height: 22px;
            border-radius: 50%;
            cursor: pointer;
            border: 2px solid transparent;
            transition: transform 0.1s;
        }}
        #highlight-toolbar .hl-dot:hover {{
            transform: scale(1.2);
            border-color: #fff;
        }}

        mark[data-hl] {{
            cursor: pointer;
            background: var(--hl-color);
            padding: 0 1px;
            border-radius: 2px;
        }}

        /* ── Responsive ─────────────────────── */
        @media (max-width: 900px) {{
            .sidebar {{ display: none; }}
            .main {{ margin-left: 0; padding: 24px 16px 80px; }}
            .sidenote {{
                float: none;
                margin: 8px 0 16px;
                width: auto;
                display: block;
                padding: 8px 12px;
                background: var(--sidenote-bg);
                border-left: 3px solid var(--sidenote-border);
                font-size: 0.85em;
            }}
        }}
    </style>
</head>
<body>
    <nav class="sidebar">
        <div class="book-title"><a href="/">The Planetary Scale Computer</a></div>
        {nav_html}
        <div class="dashboard-link">
            <a href="/dashboard">System Dashboard</a>
        </div>
    </nav>
    <div class="main">
        <article class="chapter-content" id="chapter-content">
            {chapter_content}
        </article>
    </div>

    <div id="highlight-toolbar">
        <span class="hl-dot" style="background:#E63946" data-color="#E63946"></span>
        <span class="hl-dot" style="background:#F4845F" data-color="#F4845F"></span>
        <span class="hl-dot" style="background:#F7B731" data-color="#F7B731"></span>
        <span class="hl-dot" style="background:#2A9D8F" data-color="#2A9D8F"></span>
        <span class="hl-dot" style="background:#00B4D8" data-color="#00B4D8"></span>
        <span class="hl-dot" style="background:#3A86FF" data-color="#3A86FF"></span>
        <span class="hl-dot" style="background:#5E60CE" data-color="#5E60CE"></span>
        <span class="hl-dot" style="background:#7209B7" data-color="#7209B7"></span>
        <span class="hl-dot" style="background:#B5179E" data-color="#B5179E"></span>
    </div>

    <script>
    (function() {{
        var PAGE_SLUG = "{slug}";
        var toolbar = document.getElementById('highlight-toolbar');
        var content = document.getElementById('chapter-content');
        var highlights = [];
        var hlCounter = 0;

        // ── Path serialization for Range API ──
        function nodeToPath(node, root) {{
            var path = [];
            var cur = node;
            while (cur && cur !== root) {{
                var parent = cur.parentNode;
                if (!parent) break;
                var idx = 0;
                for (var i = 0; i < parent.childNodes.length; i++) {{
                    if (parent.childNodes[i] === cur) {{ idx = i; break; }}
                }}
                path.unshift(idx);
                cur = parent;
            }}
            return path.join('/');
        }}

        function pathToNode(pathStr, root) {{
            if (!pathStr) return null;
            var parts = pathStr.split('/');
            var cur = root;
            for (var i = 0; i < parts.length; i++) {{
                var idx = parseInt(parts[i], 10);
                if (!cur.childNodes[idx]) return null;
                cur = cur.childNodes[idx];
            }}
            return cur;
        }}

        // ── Restore highlights from saved data ──
        function restoreHighlights(list) {{
            list.forEach(function(h) {{
                try {{
                    var startNode = pathToNode(h.startPath, content);
                    var endNode = pathToNode(h.endPath, content);
                    if (!startNode || !endNode) return;
                    var range = document.createRange();
                    range.setStart(startNode, h.startOffset);
                    range.setEnd(endNode, h.endOffset);
                    wrapRange(range, h.color, h.id);
                }} catch(e) {{ /* skip invalid highlights */ }}
            }});
        }}

        function wrapRange(range, color, id) {{
            var mark = document.createElement('mark');
            mark.setAttribute('data-hl', id);
            mark.style.setProperty('--hl-color', color + '44');
            mark.addEventListener('click', function() {{
                removeHighlight(id);
            }});
            try {{
                range.surroundContents(mark);
            }} catch(e) {{
                /* cross-element selections can't be wrapped simply */
            }}
        }}

        function removeHighlight(id) {{
            highlights = highlights.filter(function(h) {{ return h.id !== id; }});
            var mark = document.querySelector('mark[data-hl="' + id + '"]');
            if (mark) {{
                var parent = mark.parentNode;
                while (mark.firstChild) {{
                    parent.insertBefore(mark.firstChild, mark);
                }}
                parent.removeChild(mark);
                parent.normalize();
            }}
            saveHighlights();
        }}

        // ── Toolbar positioning on text selection ──
        document.addEventListener('mouseup', function(e) {{
            var sel = window.getSelection();
            if (!sel || sel.isCollapsed || !content.contains(sel.anchorNode)) {{
                toolbar.style.display = 'none';
                return;
            }}
            var range = sel.getRangeAt(0);
            var rect = range.getBoundingClientRect();
            toolbar.style.display = 'flex';
            toolbar.style.left = (rect.left + rect.width / 2 - 110 + window.scrollX) + 'px';
            toolbar.style.top = (rect.top - 40 + window.scrollY) + 'px';
        }});

        // ── Color dot click: create highlight ──
        toolbar.querySelectorAll('.hl-dot').forEach(function(dot) {{
            dot.addEventListener('mousedown', function(e) {{
                e.preventDefault();
                e.stopPropagation();
                var sel = window.getSelection();
                if (!sel || sel.isCollapsed) return;
                var range = sel.getRangeAt(0);
                var color = dot.getAttribute('data-color');
                var id = 'h_' + Date.now() + '_' + (++hlCounter);
                var h = {{
                    id: id,
                    startPath: nodeToPath(range.startContainer, content),
                    startOffset: range.startOffset,
                    endPath: nodeToPath(range.endContainer, content),
                    endOffset: range.endOffset,
                    color: color,
                    text: sel.toString().substring(0, 80)
                }};
                highlights.push(h);
                wrapRange(range, color, id);
                sel.removeAllRanges();
                toolbar.style.display = 'none';
                saveHighlights();
            }});
        }});

        // ── API communication ──
        function loadHighlights() {{
            fetch('/api/highlights?page=' + encodeURIComponent(PAGE_SLUG))
                .then(function(r) {{ return r.json(); }})
                .then(function(list) {{
                    if (Array.isArray(list) && list.length > 0) {{
                        highlights = list;
                        hlCounter = list.length;
                        restoreHighlights(list);
                    }}
                }})
                .catch(function() {{}});
        }}

        function saveHighlights() {{
            fetch('/api/highlights', {{
                method: 'POST',
                headers: {{ 'Content-Type': 'application/json' }},
                body: JSON.stringify({{ page: PAGE_SLUG, highlights: highlights }})
            }}).catch(function() {{}});
        }}

        loadHighlights();
    }})();
    </script>
</body>
</html>"##,
        title = title,
        nav_html = nav_html,
        chapter_content = chapter_content,
        slug = slug,
    )
}

fn landing_page() -> String {
    let items: String = book_nav_entries()
        .iter()
        .map(|entry| match entry {
            NavEntry::Part(label) => format!(
                r#"<div class="toc-part">{}</div>"#,
                label
            ),
            NavEntry::Separator => r#"<hr class="toc-sep">"#.to_string(),
            NavEntry::Chapter(item) => format!(
                r#"<a href="{}" class="toc-item">
                    <span class="toc-dot" style="background:{}"></span>
                    <span class="toc-label">{}</span>
                </a>"#,
                item.href, item.color, item.label
            ),
        })
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>The Planetary Scale Computer</title>
    <meta name="description" content="A self-describing planetary scale computer. The book is served by the very systems it describes — microservices handling discovery, routing, configuration, caching, storage, and monitoring, all built from scratch in Rust.">
    <link rel="canonical" href="https://p.jjm.net/">
    <meta property="og:title" content="The Planetary Scale Computer">
    <meta property="og:description" content="A self-describing planetary scale computer. The book is served by the very systems it describes — microservices handling discovery, routing, configuration, caching, storage, and monitoring, all built from scratch in Rust.">
    <meta property="og:type" content="website">
    <meta property="og:url" content="https://p.jjm.net/">
    <meta property="og:site_name" content="The Planetary Scale Computer">
    <meta name="twitter:card" content="summary">
    <meta name="twitter:title" content="The Planetary Scale Computer">
    <meta name="twitter:description" content="A self-describing planetary scale computer. The book is served by the very systems it describes.">
    <style>
        :root {{
            --bg: #fffff8;
            --text: #111;
            --subtitle: #666;
            --description: #555;
            --link: #2A9D8F;
            --toc-item: #333;
            --toc-hover: #f4f3ee;
            --toc-part: #999;
            --toc-sep: #e8e7de;
            --toc-border: #e0dfd6;
            --tab-label: #999;
            --si-chapters: #666;
            --si-link: #555;
            --si-link-border: #ddd;
        }}
        @media (prefers-color-scheme: dark) {{
            :root {{
                --bg: #141412;
                --text: #ddd8cc;
                --subtitle: #888;
                --description: #aaa;
                --link: #2ec9b8;
                --toc-item: #ccc;
                --toc-hover: #1e1e1a;
                --toc-part: #666;
                --toc-sep: #2a2a24;
                --toc-border: #2e2e28;
                --tab-label: #666;
                --si-chapters: #777;
                --si-link: #999;
                --si-link-border: #3a3a34;
            }}
        }}

        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{
            font-family: Georgia, "Times New Roman", serif;
            background: var(--bg);
            color: var(--text);
            display: flex;
            justify-content: center;
            padding: 80px 24px;
        }}
        .landing {{ max-width: 600px; width: 100%; }}
        .landing h1 {{
            font-size: 2.4em;
            font-weight: 400;
            line-height: 1.2;
            margin-bottom: 8px;
        }}
        .landing .subtitle {{
            font-style: italic;
            color: var(--subtitle);
            margin-bottom: 24px;
            font-size: 1.1em;
        }}
        .landing .description {{
            color: var(--description);
            font-size: 0.92em;
            line-height: 1.65;
            margin-bottom: 40px;
        }}
        .landing .description a {{
            color: var(--link);
            text-decoration: none;
        }}
        .landing .description a:hover {{
            text-decoration: underline;
        }}
        .landing .author {{
            font-variant: small-caps;
            color: var(--description);
            margin-bottom: 48px;
            font-size: 1.05em;
        }}
        .toc-header {{
            font-size: 0.9em;
            font-variant: small-caps;
            letter-spacing: 0.08em;
            color: var(--tab-label);
            margin-bottom: 16px;
        }}
        .toc-item {{
            display: flex;
            align-items: center;
            gap: 14px;
            padding: 12px 16px;
            margin: 4px 0;
            text-decoration: none;
            color: var(--toc-item);
            border-radius: 6px;
            font-size: 1.05em;
            transition: background 0.15s;
        }}
        .toc-part {{
            font-size: 0.8em;
            font-variant: small-caps;
            letter-spacing: 0.08em;
            color: var(--toc-part);
            margin: 24px 0 8px;
            padding-top: 16px;
            border-top: 1px solid var(--toc-sep);
        }}
        .toc-part:first-of-type {{
            border-top: none;
            margin-top: 8px;
        }}
        hr.toc-sep {{
            border: none;
            border-top: 1px solid var(--toc-sep);
            margin: 16px 0;
        }}
        .toc-item:hover {{ background: var(--toc-hover); }}
        .toc-dot {{
            width: 12px; height: 12px;
            border-radius: 50%;
            flex-shrink: 0;
        }}
        .dashboard-link {{
            margin-top: 48px;
            padding-top: 24px;
            border-top: 1px solid var(--toc-border);
        }}
        .dashboard-link a {{
            color: var(--tab-label);
            font-size: 0.85em;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            text-decoration: none;
        }}
        .dashboard-link a:hover {{ color: var(--description); }}
        .tab-radio {{ display: none; }}
        .tab-bar {{ display: flex; gap: 24px; margin-bottom: 16px; }}
        .tab-label {{
            font-size: 0.9em;
            font-variant: small-caps;
            letter-spacing: 0.08em;
            color: var(--tab-label);
            cursor: pointer;
            padding-bottom: 4px;
            border-bottom: 2px solid transparent;
        }}
        .tab-panel {{ display: none; }}
        #tab-index:checked ~ .tab-bar .tab-label[for="tab-index"],
        #tab-reverse:checked ~ .tab-bar .tab-label[for="tab-reverse"] {{
            color: var(--toc-item);
            border-bottom-color: var(--toc-item);
        }}
        #tab-index:checked ~ .tab-panel-index,
        #tab-reverse:checked ~ .tab-panel-reverse {{
            display: block;
        }}
        .si-entry {{ margin-bottom: 14px; }}
        .si-name {{
            display: inline-flex;
            align-items: center;
            gap: 8px;
            font-weight: bold;
            font-size: 0.95em;
        }}
        .si-dot {{
            width: 10px; height: 10px;
            border-radius: 50%;
            display: inline-block;
        }}
        .si-chapters {{
            padding-left: 18px;
            font-size: 0.85em;
            color: var(--si-chapters);
            margin-top: 2px;
        }}
        .si-chapters a {{
            color: var(--si-link);
            text-decoration: none;
            border-bottom: 1px solid var(--si-link-border);
        }}
        .si-chapters a:hover {{
            color: var(--toc-item);
            border-bottom-color: var(--si-link);
        }}
    </style>
</head>
<body>
    <div class="landing">
        <p class="author">Justin J. Meza</p>
        <h1>The Planetary Scale Computer</h1>
        <p class="subtitle">First Edition</p>
        <p class="description">
            This site is a self-describing planetary scale computer.  The book
            you are reading is served by the very systems it describes &mdash;
            a constellation of microservices handling
            <a href="/chapter/discovery">discovery</a>,
            <a href="/chapter/routing">routing</a>,
            <a href="/chapter/configuration">configuration</a>,
            <a href="/chapter/caching">caching</a>,
            <a href="/chapter/storage">storage</a>, and
            <a href="/chapter/monitoring">monitoring</a>,
            all built from scratch and running right now.
            Read the chapters to understand how they work, then visit the
            <a href="/dashboard">system dashboard</a> to see them in action.
            The <a href="https://github.com/justinmeza/planetary">source code</a>
            is freely available &mdash; readers are encouraged to run the system
            themselves and explore.
        </p>
        <input type="radio" name="idx" id="tab-index" checked class="tab-radio">
        <input type="radio" name="idx" id="tab-reverse" class="tab-radio">
        <div class="tab-bar">
            <label for="tab-index" class="tab-label">Index</label>
            <label for="tab-reverse" class="tab-label">Reverse Index</label>
        </div>
        <div class="tab-panel tab-panel-index">
            {items}
        </div>
        <div class="tab-panel tab-panel-reverse">
            <div class="si-entry">
                <div class="si-name"><span class="si-dot" style="background:#7209B7"></span> caching</div>
                <div class="si-chapters"><a href="/chapter/caching">7. Caching</a> &middot; <a href="/chapter/storage">8. Storage</a> &middot; <a href="/chapter/design">2. Design</a> &middot; <a href="/chapter/degradation">20. Degradation</a></div>
            </div>
            <div class="si-entry">
                <div class="si-name"><span class="si-dot" style="background:#3A86FF"></span> configuration</div>
                <div class="si-chapters"><a href="/chapter/configuration">4. Configuration</a> &middot; <a href="/chapter/design">2. Design</a> &middot; <a href="/chapter/operation">10. Operation</a></div>
            </div>
            <div class="si-entry">
                <div class="si-name"><span class="si-dot" style="background:#06D6A0"></span> consensus</div>
                <div class="si-chapters"><a href="/chapter/consensus">3. Consensus</a> &middot; <a href="/chapter/consistency">22. Consistency</a></div>
            </div>
            <div class="si-entry">
                <div class="si-name"><span class="si-dot" style="background:#F7B731"></span> discovery</div>
                <div class="si-chapters"><a href="/chapter/discovery">5. Discovery</a> &middot; <a href="/chapter/routing">6. Routing</a> &middot; <a href="/chapter/operation">10. Operation</a> &middot; <a href="/chapter/consensus">3. Consensus</a></div>
            </div>
            <div class="si-entry">
                <div class="si-name"><span class="si-dot" style="background:#00B4D8"></span> echo</div>
                <div class="si-chapters"><a href="/chapter/systems">1. Systems</a> &middot; <a href="/chapter/discovery">5. Discovery</a></div>
            </div>
            <div class="si-entry">
                <div class="si-name"><span class="si-dot" style="background:#1B998B"></span> loadbalancer</div>
                <div class="si-chapters"><a href="/chapter/load-balancing">21. Load Balancing</a> &middot; <a href="/chapter/routing">6. Routing</a> &middot; <a href="/chapter/traffic">25. Traffic</a></div>
            </div>
            <div class="si-entry">
                <div class="si-name"><span class="si-dot" style="background:#B5179E"></span> monitoring</div>
                <div class="si-chapters"><a href="/chapter/monitoring">14. Monitoring</a> &middot; <a href="/chapter/operation">10. Operation</a> &middot; <a href="/chapter/utilization">16. Utilization</a> &middot; <a href="/chapter/detection">36. Detection</a></div>
            </div>
            <div class="si-entry">
                <div class="si-name"><span class="si-dot" style="background:#E63946"></span> normalization</div>
                <div class="si-chapters"><a href="/chapter/systems">1. Systems</a></div>
            </div>
            <div class="si-entry">
                <div class="si-name"><span class="si-dot" style="background:#4CC9F0"></span> release</div>
                <div class="si-chapters"><a href="/chapter/release">12. Release</a> &middot; <a href="/chapter/operation">10. Operation</a></div>
            </div>
            <div class="si-entry">
                <div class="si-name"><span class="si-dot" style="background:#2A9D8F"></span> routing</div>
                <div class="si-chapters"><a href="/chapter/routing">6. Routing</a> &middot; <a href="/chapter/discovery">5. Discovery</a> &middot; <a href="/chapter/load-balancing">21. Load Balancing</a> &middot; <a href="/chapter/monitoring">14. Monitoring</a></div>
            </div>
            <div class="si-entry">
                <div class="si-name"><span class="si-dot" style="background:#F4845F"></span> rpc</div>
                <div class="si-chapters"><a href="/chapter/systems">1. Systems</a> &middot; <a href="/chapter/routing">6. Routing</a></div>
            </div>
            <div class="si-entry">
                <div class="si-name"><span class="si-dot" style="background:#FF6B35"></span> scheduling</div>
                <div class="si-chapters"><a href="/chapter/scheduling">11. Scheduling</a> &middot; <a href="/chapter/operation">10. Operation</a> &middot; <a href="/chapter/planning">19. Planning</a></div>
            </div>
            <div class="si-entry">
                <div class="si-name"><span class="si-dot" style="background:#D62828"></span> security</div>
                <div class="si-chapters"><a href="/chapter/security">13. Security</a> &middot; <a href="/chapter/operation">10. Operation</a></div>
            </div>
            <div class="si-entry">
                <div class="si-name"><span class="si-dot" style="background:#5E60CE"></span> storage</div>
                <div class="si-chapters"><a href="/chapter/storage">8. Storage</a> &middot; <a href="/chapter/caching">7. Caching</a> &middot; <a href="/chapter/design">2. Design</a> &middot; <a href="/chapter/consistency">22. Consistency</a></div>
            </div>
        </div>
        <div class="dashboard-link">
            <a href="/dashboard">System Dashboard &rarr;</a>
        </div>
    </div>
</body>
</html>"##,
        items = items
    )
}

// ── Dashboard pages (existing functionality, moved under /dashboard) ────────

fn wrap_dashboard(title: &str, nav_active: &str, body: &str) -> String {
    let nav_items = vec![
        ("/dashboard", "Dashboard"),
        ("/dashboard/config", "Configuration"),
        ("/dashboard/storage", "Storage"),
        ("/dashboard/cache", "Cache"),
        ("/dashboard/health", "Health"),
        ("/dashboard/scheduling", "Scheduling"),
        ("/dashboard/release", "Release"),
        ("/dashboard/security", "Security"),
        ("/dashboard/loadbalancer", "Load Balancer"),
        ("/dashboard/consistency", "Consistency"),
        ("/dashboard/regions", "Regions"),
    ];

    let nav_html: String = nav_items
        .iter()
        .map(|(href, label)| {
            let active = if *label == nav_active { " class=\"active\"" } else { "" };
            format!("<a href=\"{}\"{}>{}</a>", href, active, label)
        })
        .collect::<Vec<_>>()
        .join("\n        ");

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>{title} - Planetary Computer Dashboard</title>
    <meta name="robots" content="noindex, nofollow">
    <style>
        :root {{
            --bg: #f5f5f5;
            --text: #333;
            --nav-bg: #2c3e50;
            --nav-border: #34495e;
            --nav-link: #ecf0f1;
            --nav-active: #3498db;
            --heading: #2c3e50;
            --card-bg: white;
            --card-shadow: rgba(0,0,0,0.1);
            --table-border: #eee;
            --th-bg: #f8f9fa;
            --th-text: #555;
            --label: #666;
            --input-border: #ddd;
            --input-bg: white;
            --btn-primary: #3498db;
            --btn-primary-hover: #2980b9;
            --btn-danger: #e74c3c;
            --btn-danger-hover: #c0392b;
            --status-healthy: #27ae60;
            --status-unhealthy: #e74c3c;
            --status-unknown: #f39c12;
            --msg-success-bg: #d5f5e3;
            --msg-success: #27ae60;
            --msg-error-bg: #fadbd8;
            --msg-error: #e74c3c;
            --stat-bg: #f0f0f0;
            --stat-label: #888;
            --stat-value: #2c3e50;
            --empty: #999;
        }}
        @media (prefers-color-scheme: dark) {{
            :root {{
                --bg: #111;
                --text: #ccc;
                --nav-bg: #161b22;
                --nav-border: #21262d;
                --nav-link: #c9d1d9;
                --nav-active: #1f6feb;
                --heading: #e6edf3;
                --card-bg: #161b22;
                --card-shadow: rgba(0,0,0,0.4);
                --table-border: #21262d;
                --th-bg: #1c2128;
                --th-text: #8b949e;
                --label: #8b949e;
                --input-border: #30363d;
                --input-bg: #0d1117;
                --btn-primary: #1f6feb;
                --btn-primary-hover: #388bfd;
                --btn-danger: #da3633;
                --btn-danger-hover: #b91c1c;
                --status-healthy: #3fb950;
                --status-unhealthy: #f85149;
                --status-unknown: #e3b341;
                --msg-success-bg: #0d2b1a;
                --msg-success: #3fb950;
                --msg-error-bg: #2d0f0e;
                --msg-error: #f85149;
                --stat-bg: #1c2128;
                --stat-label: #8b949e;
                --stat-value: #e6edf3;
                --empty: #555;
            }}
        }}

        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; background: var(--bg); color: var(--text); }}
        nav {{ background: var(--nav-bg); padding: 0 20px; display: flex; align-items: center; gap: 0; }}
        nav .back {{ color: var(--nav-link); text-decoration: none; padding: 16px 20px; font-size: 14px; border-right: 1px solid var(--nav-border); }}
        nav a {{ color: var(--nav-link); text-decoration: none; padding: 16px 20px; font-size: 14px; }}
        nav a:hover {{ background: var(--nav-border); }}
        nav a.active {{ background: var(--nav-active); }}
        .container {{ max-width: 960px; margin: 20px auto; padding: 0 20px; }}
        h1 {{ margin-bottom: 20px; color: var(--heading); }}
        .card {{ background: var(--card-bg); border-radius: 6px; padding: 20px; margin-bottom: 16px; box-shadow: 0 1px 3px var(--card-shadow); }}
        .card h2 {{ margin-bottom: 12px; font-size: 18px; color: var(--heading); }}
        table {{ width: 100%; border-collapse: collapse; }}
        th, td {{ text-align: left; padding: 10px 12px; border-bottom: 1px solid var(--table-border); }}
        th {{ background: var(--th-bg); font-weight: 600; color: var(--th-text); }}
        .status-healthy {{ color: var(--status-healthy); font-weight: 600; }}
        .status-unhealthy {{ color: var(--status-unhealthy); font-weight: 600; }}
        .status-unknown {{ color: var(--status-unknown); font-weight: 600; }}
        form {{ display: flex; gap: 10px; align-items: end; flex-wrap: wrap; }}
        label {{ font-size: 13px; color: var(--label); display: block; margin-bottom: 4px; }}
        input[type="text"] {{ padding: 8px 12px; border: 1px solid var(--input-border); border-radius: 4px; font-size: 14px; background: var(--input-bg); color: var(--text); }}
        button {{ padding: 8px 20px; border: none; border-radius: 4px; font-size: 14px; cursor: pointer; color: white; }}
        .btn-primary {{ background: var(--btn-primary); }}
        .btn-primary:hover {{ background: var(--btn-primary-hover); }}
        .btn-danger {{ background: var(--btn-danger); }}
        .btn-danger:hover {{ background: var(--btn-danger-hover); }}
        .message {{ padding: 12px; border-radius: 4px; margin-bottom: 16px; }}
        .message-success {{ background: var(--msg-success-bg); color: var(--msg-success); }}
        .message-error {{ background: var(--msg-error-bg); color: var(--msg-error); }}
        .empty {{ color: var(--empty); font-style: italic; padding: 20px; text-align: center; }}
        .stat {{ display: inline-block; background: var(--stat-bg); padding: 12px 20px; border-radius: 6px; margin-right: 12px; margin-bottom: 8px; }}
        .stat .label {{ font-size: 12px; color: var(--stat-label); text-transform: uppercase; }}
        .stat .value {{ font-size: 24px; font-weight: 700; color: var(--stat-value); }}
    </style>
</head>
<body>
    <nav>
        <a class="back" href="/">&larr; Book</a>
        {nav_html}
    </nav>
    <div class="container">
        <h1>{title}</h1>
        {body}
    </div>
</body>
</html>"#,
        title = title,
        nav_html = nav_html,
        body = body,
    )
}

async fn page_dashboard() -> String {
    let mut services_html = String::new();

    let service_names = vec![
        ("discovery", DISCOVERY_ADDR),
        ("echo", "127.0.0.1:10100"),
        ("configuration", CONFIGURATION_ADDR),
        ("storage", STORAGE_ADDR),
        ("caching", CACHING_ADDR),
        ("monitoring", MONITORING_ADDR),
        ("routing", "127.0.0.1:10300"),
        ("scheduling", SCHEDULING_ADDR),
        ("release", RELEASE_ADDR),
        ("security", SECURITY_ADDR),
    ];

    let health_args = monitoring::HealthArgs { placeholder: 0 };
    let health_resp = send(
        MONITORING_ADDR,
        monitoring::HEALTH_PROCEDURE,
        health_args.serialize(),
    )
    .await;
    let health_map: HashMap<String, String> =
        match monitoring::HealthResult::deserialize(&health_resp) {
            Ok(result) if !result.services.is_empty() => result
                .services
                .split(';')
                .filter_map(|entry| {
                    let parts: Vec<&str> = entry.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        Some((parts[0].to_string(), parts[1].to_string()))
                    } else {
                        None
                    }
                })
                .collect(),
            _ => HashMap::new(),
        };

    for (name, addr) in &service_names {
        let reachable = tokio::net::TcpStream::connect(*addr).await.is_ok();
        let status = if reachable {
            health_map
                .get(*name)
                .cloned()
                .unwrap_or_else(|| "running".to_string())
        } else {
            "unreachable".to_string()
        };

        let status_class = if reachable {
            "status-healthy"
        } else {
            "status-unhealthy"
        };

        services_html.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td class=\"{}\">{}</td></tr>\n",
            html_escape(name),
            html_escape(addr),
            status_class,
            html_escape(&status),
        ));
    }

    let body = format!(
        r#"<div class="card">
    <h2>Service Overview</h2>
    <table>
        <tr><th>Service</th><th>Address</th><th>Status</th></tr>
        {}
    </table>
</div>"#,
        services_html
    );

    wrap_dashboard("Dashboard", "Dashboard", &body)
}

async fn page_config(body: &str) -> String {
    let mut message = String::new();

    if !body.is_empty() {
        let form = parse_form(body);
        if let (Some(key), Some(value)) = (form.get("key"), form.get("value")) {
            if !key.is_empty() {
                let args = configuration::SetArgs {
                    key: key.clone(),
                    value: value.clone(),
                };
                let resp = send(
                    CONFIGURATION_ADDR,
                    configuration::SET_PROCEDURE,
                    args.serialize(),
                )
                .await;
                if resp == "OK" {
                    message = format!(
                        "<div class=\"message message-success\">Set key '{}' successfully.</div>",
                        html_escape(key)
                    );
                } else {
                    message = format!(
                        "<div class=\"message message-error\">Error: {}</div>",
                        html_escape(&resp)
                    );
                }
            }
        }
    }

    let list_args = configuration::ListArgs {
        prefix: String::new(),
    };
    let resp = send(
        CONFIGURATION_ADDR,
        configuration::LIST_PROCEDURE,
        list_args.serialize(),
    )
    .await;

    let mut rows = String::new();
    if let Ok(result) = configuration::ListResult::deserialize(&resp) {
        if !result.keys.is_empty() {
            for key in result.keys.split(',') {
                if key.is_empty() {
                    continue;
                }
                let get_args = configuration::GetArgs {
                    key: key.to_string(),
                };
                let val_resp = send(
                    CONFIGURATION_ADDR,
                    configuration::GET_PROCEDURE,
                    get_args.serialize(),
                )
                .await;
                let value = configuration::GetResult::deserialize(&val_resp)
                    .map(|r| r.value)
                    .unwrap_or_default();

                rows.push_str(&format!(
                    r#"<tr>
    <td>{}</td>
    <td>{}</td>
    <td>
        <form method="POST" action="/dashboard/config/delete" style="margin:0">
            <input type="hidden" name="key" value="{}">
            <button type="submit" class="btn-danger">Delete</button>
        </form>
    </td>
</tr>"#,
                    html_escape(key),
                    html_escape(&value),
                    html_escape(key),
                ));
            }
        }
    }

    let table = if rows.is_empty() {
        "<div class=\"empty\">No configuration keys found.</div>".to_string()
    } else {
        format!(
            "<table><tr><th>Key</th><th>Value</th><th>Actions</th></tr>{}</table>",
            rows
        )
    };

    let body_html = format!(
        r#"{}
<div class="card">
    <h2>Set Configuration</h2>
    <form method="POST" action="/dashboard/config">
        <div><label>Key</label><input type="text" name="key" placeholder="app.setting"></div>
        <div><label>Value</label><input type="text" name="value" placeholder="value"></div>
        <button type="submit" class="btn-primary">Set</button>
    </form>
</div>
<div class="card">
    <h2>Configuration Entries</h2>
    {}
</div>"#,
        message, table
    );

    wrap_dashboard("Configuration", "Configuration", &body_html)
}

async fn page_config_delete(body: &str) -> String {
    let form = parse_form(body);
    if let Some(key) = form.get("key") {
        if !key.is_empty() {
            let args = configuration::DeleteArgs {
                key: key.clone(),
            };
            send(
                CONFIGURATION_ADDR,
                configuration::DELETE_PROCEDURE,
                args.serialize(),
            )
            .await;
        }
    }
    page_config("").await
}

async fn page_storage(body: &str) -> String {
    let mut message = String::new();

    if !body.is_empty() {
        let form = parse_form(body);
        if let (Some(key), Some(value)) = (form.get("key"), form.get("value")) {
            if !key.is_empty() {
                let args = storage::PutArgs {
                    key: key.clone(),
                    value: value.clone(),
                };
                let resp =
                    send(STORAGE_ADDR, storage::PUT_PROCEDURE, args.serialize()).await;
                if resp == "OK" {
                    message = format!(
                        "<div class=\"message message-success\">Stored key '{}' successfully.</div>",
                        html_escape(key)
                    );
                } else {
                    message = format!(
                        "<div class=\"message message-error\">Error: {}</div>",
                        html_escape(&resp)
                    );
                }
            }
        }
    }

    let scan_args = storage::ScanArgs {
        prefix: String::new(),
        limit: 100,
    };
    let resp = send(STORAGE_ADDR, storage::SCAN_PROCEDURE, scan_args.serialize()).await;

    let mut rows = String::new();
    if let Ok(result) = storage::ScanResult::deserialize(&resp) {
        if !result.entries.is_empty() {
            for entry in result.entries.split(';') {
                if entry.is_empty() {
                    continue;
                }
                let parts: Vec<&str> = entry.splitn(2, '=').collect();
                if parts.len() == 2 {
                    rows.push_str(&format!(
                        r#"<tr>
    <td>{}</td>
    <td>{}</td>
    <td>
        <form method="POST" action="/dashboard/storage/delete" style="margin:0">
            <input type="hidden" name="key" value="{}">
            <button type="submit" class="btn-danger">Delete</button>
        </form>
    </td>
</tr>"#,
                        html_escape(parts[0]),
                        html_escape(parts[1]),
                        html_escape(parts[0]),
                    ));
                }
            }
        }
    }

    let table = if rows.is_empty() {
        "<div class=\"empty\">No storage entries found.</div>".to_string()
    } else {
        format!(
            "<table><tr><th>Key</th><th>Value</th><th>Actions</th></tr>{}</table>",
            rows
        )
    };

    let body_html = format!(
        r#"{}
<div class="card">
    <h2>Store Key-Value</h2>
    <form method="POST" action="/dashboard/storage">
        <div><label>Key</label><input type="text" name="key" placeholder="my.key"></div>
        <div><label>Value</label><input type="text" name="value" placeholder="value"></div>
        <button type="submit" class="btn-primary">Put</button>
    </form>
</div>
<div class="card">
    <h2>Storage Entries</h2>
    {}
</div>"#,
        message, table
    );

    wrap_dashboard("Storage", "Storage", &body_html)
}

async fn page_storage_delete(body: &str) -> String {
    let form = parse_form(body);
    if let Some(key) = form.get("key") {
        if !key.is_empty() {
            let args = storage::DeleteArgs {
                key: key.clone(),
            };
            send(STORAGE_ADDR, storage::DELETE_PROCEDURE, args.serialize()).await;
        }
    }
    page_storage("").await
}

async fn page_cache() -> String {
    let stats_args = caching::StatsArgs { placeholder: 0 };
    let resp = send(
        CACHING_ADDR,
        caching::STATS_PROCEDURE,
        stats_args.serialize(),
    )
    .await;

    let stats_html = match caching::StatsResult::deserialize(&resp) {
        Ok(stats) => format!(
            r#"<div>
    <div class="stat"><div class="label">Cache Hits</div><div class="value">{}</div></div>
    <div class="stat"><div class="label">Cache Misses</div><div class="value">{}</div></div>
    <div class="stat"><div class="label">Entries</div><div class="value">{}</div></div>
    <div class="stat"><div class="label">Hit Rate</div><div class="value">{:.1}%</div></div>
</div>"#,
            stats.hits,
            stats.misses,
            stats.size,
            if stats.hits + stats.misses > 0 {
                (stats.hits as f64 / (stats.hits + stats.misses) as f64) * 100.0
            } else {
                0.0
            },
        ),
        Err(_) => "<div class=\"message message-error\">Failed to fetch cache stats.</div>"
            .to_string(),
    };

    let body = format!(
        r#"<div class="card">
    <h2>Cache Statistics</h2>
    {}
</div>"#,
        stats_html
    );

    wrap_dashboard("Cache", "Cache", &body)
}

async fn page_health() -> String {
    let health_args = monitoring::HealthArgs { placeholder: 0 };
    let resp = send(
        MONITORING_ADDR,
        monitoring::HEALTH_PROCEDURE,
        health_args.serialize(),
    )
    .await;

    let mut rows = String::new();
    if let Ok(result) = monitoring::HealthResult::deserialize(&resp) {
        if !result.services.is_empty() {
            for entry in result.services.split(';') {
                if entry.is_empty() {
                    continue;
                }
                let parts: Vec<&str> = entry.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let status_class = match parts[1] {
                        "healthy" => "status-healthy",
                        "unhealthy" => "status-unhealthy",
                        _ => "status-unknown",
                    };
                    rows.push_str(&format!(
                        "<tr><td>{}</td><td class=\"{}\">{}</td></tr>\n",
                        html_escape(parts[0]),
                        status_class,
                        html_escape(parts[1]),
                    ));
                }
            }
        }
    }

    let table = if rows.is_empty() {
        "<div class=\"empty\">No health data available.</div>".to_string()
    } else {
        format!(
            "<table><tr><th>Service</th><th>Status</th></tr>{}</table>",
            rows
        )
    };

    let services = vec![
        ("discovery", DISCOVERY_ADDR),
        ("configuration", CONFIGURATION_ADDR),
        ("storage", STORAGE_ADDR),
        ("caching", CACHING_ADDR),
        ("monitoring", MONITORING_ADDR),
        ("routing", "127.0.0.1:10300"),
        ("echo", "127.0.0.1:10100"),
        ("scheduling", SCHEDULING_ADDR),
        ("release", RELEASE_ADDR),
        ("security", SECURITY_ADDR),
    ];

    let mut connectivity_rows = String::new();
    for (name, addr) in &services {
        let reachable = tokio::net::TcpStream::connect(*addr).await.is_ok();
        let (status_class, status_text) = if reachable {
            ("status-healthy", "reachable")
        } else {
            ("status-unhealthy", "unreachable")
        };
        connectivity_rows.push_str(&format!(
            "<tr><td>{}</td><td>{}</td><td class=\"{}\">{}</td></tr>\n",
            html_escape(name),
            addr,
            status_class,
            status_text,
        ));
    }

    let body = format!(
        r#"<div class="card">
    <h2>Service Health (from Monitoring)</h2>
    {}
</div>
<div class="card">
    <h2>Connectivity Check</h2>
    <table>
        <tr><th>Service</th><th>Address</th><th>Status</th></tr>
        {}
    </table>
</div>"#,
        table, connectivity_rows
    );

    wrap_dashboard("System Health", "Health", &body)
}

// ── Highlight API handlers ──────────────────────────────────────────────────

async fn api_get_highlights(user_id: &str, page: &str) -> String {
    let cache_key = format!("hl:{}:{}", user_id, page);

    // Try cache first
    let cache_args = caching::GetArgs {
        key: cache_key.clone(),
    };
    let cache_resp = send(
        CACHING_ADDR,
        caching::GET_PROCEDURE,
        cache_args.serialize(),
    )
    .await;

    if let Ok(result) = caching::GetResult::deserialize(&cache_resp) {
        if result.hit == 1 && !result.value.is_empty() {
            // Decode base64 and return
            let decoded = base64_decode(&result.value);
            if let Ok(json) = String::from_utf8(decoded) {
                return json;
            }
        }
    }

    // Fall back to storage
    let storage_args = storage::GetArgs {
        key: cache_key.clone(),
    };
    let storage_resp = send(
        STORAGE_ADDR,
        storage::GET_PROCEDURE,
        storage_args.serialize(),
    )
    .await;

    if let Ok(result) = storage::GetResult::deserialize(&storage_resp) {
        if result.found == 1 && !result.value.is_empty() {
            // Put in cache for next time
            let set_args = caching::SetArgs {
                key: cache_key,
                value: result.value.clone(),
                ttl_secs: 3600,
            };
            let _ = send(CACHING_ADDR, caching::SET_PROCEDURE, set_args.serialize()).await;

            let decoded = base64_decode(&result.value);
            if let Ok(json) = String::from_utf8(decoded) {
                return json;
            }
        }
    }

    "[]".to_string()
}

async fn api_post_highlights(user_id: &str, body: &str) -> String {
    // Parse the JSON body to extract page and highlights
    // Simple JSON parsing: find "page":"..." and "highlights":[...]
    let page = extract_json_string(body, "page").unwrap_or_default();
    if page.is_empty() {
        return r#"{"error":"missing page"}"#.to_string();
    }

    // Extract the highlights array (everything between "highlights": and the closing)
    let highlights_json = if let Some(idx) = body.find("\"highlights\"") {
        let rest = &body[idx..];
        if let Some(arr_start) = rest.find('[') {
            // Find matching bracket
            let arr_bytes = rest[arr_start..].as_bytes();
            let mut depth = 0;
            let mut end = arr_bytes.len();
            for (i, &b) in arr_bytes.iter().enumerate() {
                match b {
                    b'[' => depth += 1,
                    b']' => {
                        depth -= 1;
                        if depth == 0 {
                            end = i + 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            rest[arr_start..arr_start + end].to_string()
        } else {
            "[]".to_string()
        }
    } else {
        "[]".to_string()
    };

    let cache_key = format!("hl:{}:{}", user_id, page);
    let encoded = base64_encode(highlights_json.as_bytes());

    // Write to storage
    let put_args = storage::PutArgs {
        key: cache_key.clone(),
        value: encoded.clone(),
    };
    let _ = send(STORAGE_ADDR, storage::PUT_PROCEDURE, put_args.serialize()).await;

    // Write to cache
    let set_args = caching::SetArgs {
        key: cache_key,
        value: encoded,
        ttl_secs: 3600,
    };
    let _ = send(CACHING_ADDR, caching::SET_PROCEDURE, set_args.serialize()).await;

    r#"{"ok":true}"#.to_string()
}

fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\"", key);
    let idx = json.find(&pattern)?;
    let rest = &json[idx + pattern.len()..];
    // Skip whitespace and colon
    let rest = rest.trim_start();
    let rest = rest.strip_prefix(':')?;
    let rest = rest.trim_start();
    let rest = rest.strip_prefix('"')?;
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

fn extract_json_value(json: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\"", key);
    let idx = json.find(&pattern)?;
    let rest = &json[idx + pattern.len()..];
    let rest = rest.trim_start();
    let rest = rest.strip_prefix(':')?;
    let rest = rest.trim_start();
    // Find end: comma, }, or ]
    let end = rest.find(|c: char| c == ',' || c == '}' || c == ']').unwrap_or(rest.len());
    let val = rest[..end].trim();
    if val.is_empty() { None } else { Some(val.to_string()) }
}

fn extract_json_array_strings(json: &str, key: &str) -> Vec<String> {
    let pattern = format!("\"{}\"", key);
    let idx = match json.find(&pattern) {
        Some(i) => i,
        None => return Vec::new(),
    };
    let rest = &json[idx + pattern.len()..];
    let rest = rest.trim_start();
    let rest = match rest.strip_prefix(':') {
        Some(r) => r.trim_start(),
        None => return Vec::new(),
    };
    let rest = match rest.strip_prefix('[') {
        Some(r) => r,
        None => return Vec::new(),
    };
    let end = match rest.find(']') {
        Some(i) => i,
        None => return Vec::new(),
    };
    let inner = &rest[..end];
    inner
        .split(',')
        .filter_map(|s| {
            let s = s.trim().trim_matches('"');
            if s.is_empty() { None } else { Some(s.to_string()) }
        })
        .collect()
}

// ── Security middleware ──────────────────────────────────────────────────────

async fn require_admin(headers: &str) -> bool {
    if let Some(token) = parse_cookie(headers, "auth_token") {
        if !token.is_empty() {
            let result = security::validate_token(SECURITY_ADDR, token).await;
            return result.valid == 1;
        }
    }
    false
}

fn forbidden_page() -> String {
    wrap_dashboard(
        "Access Denied",
        "",
        r#"<div class="card">
    <h2>Authentication Required</h2>
    <p>You must have a valid auth token to perform this action.</p>
    <p>Visit the <a href="/dashboard/security">Security dashboard</a> to create a token, then set it as your <code>auth_token</code> cookie.</p>
</div>"#,
    )
}

// ── Scheduling dashboard ────────────────────────────────────────────────────

async fn page_scheduling() -> String {
    let instances_result = scheduling::list_instances(SCHEDULING_ADDR).await;

    let mut rows = String::new();
    if !instances_result.instances.is_empty() {
        for entry in instances_result.instances.split(';') {
            if entry.is_empty() {
                continue;
            }
            // Format: id:service_name:address:port:pid:status
            let parts: Vec<&str> = entry.splitn(6, ':').collect();
            if parts.len() >= 6 {
                let status_class = match parts[5] {
                    "healthy" => "status-healthy",
                    "unhealthy" => "status-unhealthy",
                    _ => "status-unknown",
                };
                rows.push_str(&format!(
                    "<tr><td>{}</td><td>{}</td><td>{}:{}</td><td>{}</td><td class=\"{}\">{}</td><td>\
                    <form method=\"POST\" action=\"/dashboard/scheduling/stop\" style=\"margin:0\">\
                    <input type=\"hidden\" name=\"instance_id\" value=\"{}\">\
                    <button type=\"submit\" class=\"btn-danger\">Stop</button></form></td></tr>\n",
                    html_escape(parts[0]),
                    html_escape(parts[1]),
                    html_escape(parts[2]),
                    html_escape(parts[3]),
                    html_escape(parts[4]),
                    status_class,
                    html_escape(parts[5]),
                    html_escape(parts[0]),
                ));
            }
        }
    }

    let table = if rows.is_empty() {
        "<div class=\"empty\">No instances running.</div>".to_string()
    } else {
        format!(
            "<table><tr><th>ID</th><th>Service</th><th>Address</th><th>PID</th><th>Status</th><th>Actions</th></tr>{}</table>",
            rows
        )
    };

    let body = format!(
        r#"<div class="card">
    <h2>Schedule Service</h2>
    <form method="POST" action="/dashboard/scheduling">
        <div><label>Service Name</label><input type="text" name="name" placeholder="echo"></div>
        <div><label>Manifest Path</label><input type="text" name="manifest_path" placeholder="echo/Cargo.toml"></div>
        <div><label>Binary Name (optional)</label><input type="text" name="bin_name" placeholder="server_v1"></div>
        <div><label>Replicas</label><input type="text" name="replicas" placeholder="3"></div>
        <button type="submit" class="btn-primary">Schedule</button>
    </form>
</div>
<div class="card">
    <h2>Running Instances</h2>
    {}
</div>"#,
        table
    );

    wrap_dashboard("Scheduling", "Scheduling", &body)
}

async fn page_scheduling_post(post_body: &str) -> String {
    let form = parse_form(post_body);
    if let Some(name) = form.get("name") {
        if !name.is_empty() {
            let manifest_path = form.get("manifest_path").cloned().unwrap_or_default();
            let bin_name = form.get("bin_name").cloned().unwrap_or_default();
            let replicas: i32 = form
                .get("replicas")
                .and_then(|r| r.parse().ok())
                .unwrap_or(1);
            let _ = scheduling::schedule_service(
                SCHEDULING_ADDR,
                name.clone(),
                manifest_path,
                bin_name,
                replicas,
            )
            .await;
        }
    }
    page_scheduling().await
}

async fn page_scheduling_stop(post_body: &str) -> String {
    let form = parse_form(post_body);
    if let Some(instance_id) = form.get("instance_id") {
        if !instance_id.is_empty() {
            let _ =
                scheduling::stop_instance(SCHEDULING_ADDR, instance_id.clone()).await;
        }
    }
    page_scheduling().await
}

// ── Release dashboard ───────────────────────────────────────────────────────

async fn page_release() -> String {
    let releases_result = release::list_releases(RELEASE_ADDR).await;

    let mut rows = String::new();
    if !releases_result.releases.is_empty() {
        for entry in releases_result.releases.split(';') {
            if entry.is_empty() {
                continue;
            }
            // Format: id:service:version:status:batch_progress
            let parts: Vec<&str> = entry.splitn(5, ':').collect();
            if parts.len() >= 5 {
                let status_class = match parts[3] {
                    "deployed" => "status-healthy",
                    "deploying" => "status-unknown",
                    "rolled_back" => "status-unhealthy",
                    _ => "",
                };
                let actions = if parts[3] == "created" || parts[3] == "deploying" {
                    format!(
                        r#"<form method="POST" action="/dashboard/release/advance" style="margin:0;display:inline">
                            <input type="hidden" name="release_id" value="{}">
                            <button type="submit" class="btn-primary">Advance</button>
                        </form>
                        <form method="POST" action="/dashboard/release/rollback" style="margin:0;display:inline">
                            <input type="hidden" name="service" value="{}">
                            <button type="submit" class="btn-danger">Rollback</button>
                        </form>"#,
                        html_escape(parts[0]),
                        html_escape(parts[1]),
                    )
                } else {
                    String::new()
                };

                rows.push_str(&format!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td><td class=\"{}\">{}</td><td>{}</td><td>{}</td></tr>\n",
                    html_escape(parts[0]),
                    html_escape(parts[1]),
                    html_escape(parts[2]),
                    status_class,
                    html_escape(parts[3]),
                    html_escape(parts[4]),
                    actions,
                ));
            }
        }
    }

    let table = if rows.is_empty() {
        "<div class=\"empty\">No releases found.</div>".to_string()
    } else {
        format!(
            "<table><tr><th>ID</th><th>Service</th><th>Version</th><th>Status</th><th>Progress</th><th>Actions</th></tr>{}</table>",
            rows
        )
    };

    let body = format!(
        r#"<div class="card">
    <h2>Create Release</h2>
    <form method="POST" action="/dashboard/release">
        <div><label>Service</label><input type="text" name="service" placeholder="echo"></div>
        <div><label>Version</label><input type="text" name="version" placeholder="v2.0"></div>
        <div><label>Description</label><input type="text" name="description" placeholder="New feature release"></div>
        <button type="submit" class="btn-primary">Create Release</button>
    </form>
</div>
<div class="card">
    <h2>Releases</h2>
    {}
</div>"#,
        table
    );

    wrap_dashboard("Release", "Release", &body)
}

async fn page_release_post(post_body: &str) -> String {
    let form = parse_form(post_body);
    if let (Some(service), Some(version)) = (form.get("service"), form.get("version")) {
        if !service.is_empty() {
            let description = form.get("description").cloned().unwrap_or_default();
            let _ = release::create_release(
                RELEASE_ADDR,
                service.clone(),
                version.clone(),
                description,
            )
            .await;
        }
    }
    page_release().await
}

async fn page_release_advance(post_body: &str) -> String {
    let form = parse_form(post_body);
    if let Some(release_id) = form.get("release_id") {
        if !release_id.is_empty() {
            let _ = release::advance_release(RELEASE_ADDR, release_id.clone()).await;
        }
    }
    page_release().await
}

async fn page_release_rollback(post_body: &str) -> String {
    let form = parse_form(post_body);
    if let Some(service) = form.get("service") {
        if !service.is_empty() {
            let _ = release::rollback(RELEASE_ADDR, service.clone()).await;
        }
    }
    page_release().await
}

// ── Security dashboard ──────────────────────────────────────────────────────

async fn page_security() -> String {
    let tokens_result = security::list_tokens(SECURITY_ADDR).await;

    let mut rows = String::new();
    if !tokens_result.tokens.is_empty() {
        for entry in tokens_result.tokens.split(';') {
            if entry.is_empty() {
                continue;
            }
            // Format: name:token:permissions:created_at
            let parts: Vec<&str> = entry.splitn(4, ':').collect();
            if parts.len() >= 4 {
                let token_display = if parts[1].len() > 8 {
                    format!("{}...", &parts[1][..8])
                } else {
                    parts[1].to_string()
                };
                rows.push_str(&format!(
                    "<tr><td>{}</td><td><code>{}</code></td><td>{}</td><td>{}</td><td>\
                    <form method=\"POST\" action=\"/dashboard/security/revoke\" style=\"margin:0\">\
                    <input type=\"hidden\" name=\"token\" value=\"{}\">\
                    <button type=\"submit\" class=\"btn-danger\">Revoke</button></form></td></tr>\n",
                    html_escape(parts[0]),
                    html_escape(&token_display),
                    html_escape(parts[2]),
                    html_escape(parts[3]),
                    html_escape(parts[1]),
                ));
            }
        }
    }

    let table = if rows.is_empty() {
        "<div class=\"empty\">No tokens found.</div>".to_string()
    } else {
        format!(
            "<table><tr><th>Name</th><th>Token</th><th>Permissions</th><th>Created</th><th>Actions</th></tr>{}</table>",
            rows
        )
    };

    let body = format!(
        r#"<div class="card">
    <h2>Create Token</h2>
    <form method="POST" action="/dashboard/security">
        <div><label>Name</label><input type="text" name="name" placeholder="admin"></div>
        <div><label>Permissions</label><input type="text" name="permissions" placeholder="admin,read,write"></div>
        <button type="submit" class="btn-primary">Create Token</button>
    </form>
</div>
<div class="card">
    <h2>Tokens</h2>
    {}
</div>
<div class="card">
    <h2>Usage</h2>
    <p>After creating a token, set it as a cookie to authenticate dashboard actions:</p>
    <pre><code>document.cookie = "auth_token=YOUR_TOKEN_HERE; path=/";</code></pre>
</div>"#,
        table
    );

    wrap_dashboard("Security", "Security", &body)
}

async fn page_security_post(post_body: &str) -> (String, Option<String>) {
    let form = parse_form(post_body);
    if let Some(name) = form.get("name") {
        if !name.is_empty() {
            let permissions = form.get("permissions").cloned().unwrap_or_default();
            let result =
                security::create_token(SECURITY_ADDR, name.clone(), permissions).await;
            // Set the token as a cookie for the user
            let page = page_security().await;
            return (
                page,
                Some(format!(
                    "auth_token={}; Path=/; Max-Age=31536000",
                    result.token
                )),
            );
        }
    }
    (page_security().await, None)
}

async fn page_security_revoke(post_body: &str) -> String {
    let form = parse_form(post_body);
    if let Some(token) = form.get("token") {
        if !token.is_empty() {
            let _ = security::revoke_token(SECURITY_ADDR, token.clone()).await;
        }
    }
    page_security().await
}

// ── Load Balancer dashboard ──────────────────────────────────────────────────

async fn page_loadbalancer() -> String {
    // Fetch lb status via HTTP
    let status_json = fetch_lb_status().await;

    // Parse the JSON manually
    let strategy = extract_json_string(&status_json, "strategy").unwrap_or_else(|| "unknown".to_string());
    let backend_count = extract_json_value(&status_json, "backend_count").unwrap_or_else(|| "0".to_string());
    let own_region = extract_json_string(&status_json, "own_region").unwrap_or_else(|| "unknown".to_string());
    let shedding = extract_json_value(&status_json, "shedding").unwrap_or_else(|| "false".to_string()) == "true";
    let local_util = extract_json_value(&status_json, "local_utilization").unwrap_or_else(|| "0".to_string());
    let local_util_pct: f64 = local_util.parse::<f64>().unwrap_or(0.0) * 100.0;
    let shed_threshold = extract_json_value(&status_json, "shed_threshold").unwrap_or_else(|| "0.8".to_string());
    let shed_threshold_pct: f64 = shed_threshold.parse::<f64>().unwrap_or(0.8) * 100.0;
    let drained_regions = extract_json_array_strings(&status_json, "drained_regions");

    // Parse backends array
    let mut backends_html = String::new();
    // Find the backends array (last [ in the JSON)
    if let Some(arr_start) = status_json.find("\"backends\":[") {
        let arr_offset = arr_start + "\"backends\":[".len();
        if let Some(arr_end) = status_json[arr_offset..].rfind(']') {
            let arr = &status_json[arr_offset..arr_offset + arr_end];
            for entry in arr.split("},{") {
                let clean = entry.trim_start_matches('{').trim_end_matches('}');
                let address = extract_json_string(clean, "address").unwrap_or_default();
                let healthy = clean.contains("\"healthy\":true");
                let is_local = clean.contains("\"local\":true");
                let active = extract_json_value(clean, "active_connections").unwrap_or_else(|| "0".to_string());

                let status_class = if healthy { "status-healthy" } else { "status-unhealthy" };
                let status_text = if healthy { "healthy" } else { "unhealthy" };

                let locality = if is_local {
                    "local".to_string()
                } else {
                    // Determine remote region from address
                    let remote_region = if address.starts_with("10.0.0.1") {
                        "sfo"
                    } else if address.starts_with("10.0.0.2") {
                        "nyc"
                    } else if address.starts_with("10.0.0.3") {
                        "ams"
                    } else {
                        "remote"
                    };
                    format!("remote ({})", remote_region)
                };

                backends_html.push_str(&format!(
                    "<tr><td>{}</td><td class=\"{}\">{}</td><td>{}</td><td>{}</td></tr>\n",
                    html_escape(&address),
                    status_class,
                    status_text,
                    html_escape(&active),
                    html_escape(&locality),
                ));
            }
        }
    }

    let table = if backends_html.is_empty() {
        "<div class=\"empty\">No backends registered yet. The load balancer refreshes from discovery every 5 seconds.</div>".to_string()
    } else {
        format!(
            "<table><tr><th>Address</th><th>Health</th><th>Active Connections</th><th>Locality</th></tr>{}</table>",
            backends_html
        )
    };

    // Edge load balancing card
    let shedding_indicator = if shedding {
        "<span style=\"color:#e74c3c;font-weight:600;\">active</span>"
    } else {
        "<span style=\"color:#27ae60;\">inactive</span>"
    };

    let util_bar_color = if local_util_pct >= shed_threshold_pct { "#e74c3c" } else { "#27ae60" };

    // Region drain controls
    let all_regions = &[("sfo", "SFO"), ("nyc", "NYC"), ("ams", "AMS")];
    let mut drain_rows = String::new();
    for &(region_id, region_label) in all_regions {
        let is_drained = drained_regions.iter().any(|r| r == region_id);
        let is_own = region_id == own_region;
        let own_label = if is_own { " (this region)" } else { "" };
        let (status_class, status_text) = if is_drained {
            ("status-unhealthy", "drained")
        } else {
            ("status-healthy", "active")
        };
        let action = if is_drained {
            format!(
                r#"<form method="POST" action="/dashboard/loadbalancer/undrain" style="display:inline;margin:0;"><input type="hidden" name="region" value="{}"><button type="submit" class="btn-sm" style="background:#27ae60;color:#fff;border:none;padding:4px 12px;border-radius:4px;cursor:pointer;">Undrain</button></form>"#,
                region_id
            )
        } else {
            format!(
                r#"<form method="POST" action="/dashboard/loadbalancer/drain" style="display:inline;margin:0;"><input type="hidden" name="region" value="{}"><button type="submit" class="btn-sm" style="background:#e74c3c;color:#fff;border:none;padding:4px 12px;border-radius:4px;cursor:pointer;">Drain</button></form>"#,
                region_id
            )
        };
        drain_rows.push_str(&format!(
            "<tr><td>{}{}</td><td class=\"{}\">{}</td><td>{}</td></tr>\n",
            region_label, own_label, status_class, status_text, action
        ));
    }

    let body = format!(
        r#"<div class="card">
    <h2>Load Balancer Status</h2>
    <div>
        <div class="stat"><div class="label">Strategy</div><div class="value">{strategy}</div></div>
        <div class="stat"><div class="label">Backends</div><div class="value">{backend_count}</div></div>
    </div>
</div>
<div class="card">
    <h2>Change Strategy</h2>
    <form method="POST" action="/dashboard/loadbalancer">
        <div><label>Strategy</label>
            <select name="strategy" style="padding:8px 12px;border:1px solid #ddd;border-radius:4px;font-size:14px;">
                <option value="round-robin" {rr}>Round Robin</option>
                <option value="least-connections" {lc}>Least Connections</option>
                <option value="random" {rnd}>Random</option>
                <option value="pick-2" {p2}>Pick-2 (Power of Two)</option>
            </select>
        </div>
        <button type="submit" class="btn-primary">Update Strategy</button>
    </form>
</div>
<div class="card">
    <h2>Edge Load Balancing</h2>
    <div>
        <div class="stat"><div class="label">Region</div><div class="value">{own_region_upper}</div></div>
        <div class="stat"><div class="label">Local Utilization</div><div class="value">{local_util_pct:.1}%</div></div>
        <div class="stat"><div class="label">Shedding</div><div class="value">{shedding_indicator}</div></div>
        <div class="stat"><div class="label">Shed Threshold</div><div class="value">{shed_threshold_pct:.0}%</div></div>
    </div>
    <div style="margin-top:12px;">
        <div style="background:#eee;border-radius:4px;height:20px;overflow:hidden;">
            <div style="background:{util_bar_color};height:100%;width:{util_bar_width}%;transition:width 0.3s;"></div>
        </div>
    </div>
</div>
<div class="card">
    <h2>Region Drain Controls</h2>
    <table>
        <tr><th>Region</th><th>Status</th><th>Action</th></tr>
        {drain_rows}
    </table>
</div>
<div class="card">
    <h2>Backend Instances</h2>
    {table}
</div>"#,
        strategy = html_escape(&strategy),
        backend_count = html_escape(&backend_count),
        rr = if strategy == "round-robin" { "selected" } else { "" },
        lc = if strategy == "least-connections" { "selected" } else { "" },
        rnd = if strategy == "random" { "selected" } else { "" },
        p2 = if strategy == "pick-2" { "selected" } else { "" },
        own_region_upper = html_escape(&own_region.to_uppercase()),
        local_util_pct = local_util_pct,
        shedding_indicator = shedding_indicator,
        shed_threshold_pct = shed_threshold_pct,
        util_bar_color = util_bar_color,
        util_bar_width = local_util_pct.min(100.0),
        drain_rows = drain_rows,
        table = table,
    );

    wrap_dashboard("Load Balancer", "Load Balancer", &body)
}

async fn page_loadbalancer_post(post_body: &str) -> String {
    let form = parse_form(post_body);
    if let Some(strategy) = form.get("strategy") {
        if !strategy.is_empty() {
            // POST to load balancer's /__lb_strategy endpoint
            let _ = post_lb_strategy(strategy).await;
        }
    }
    page_loadbalancer().await
}

async fn fetch_lb_status() -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    match TcpStream::connect(LOADBALANCER_ADDR).await {
        Ok(mut stream) => {
            let request = "GET /__lb_status HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nConnection: close\r\n\r\n";
            if stream.write_all(request.as_bytes()).await.is_err() {
                return "{}".to_string();
            }
            let mut buf = vec![0u8; 16384];
            let mut total = 0;
            loop {
                match stream.read(&mut buf[total..]).await {
                    Ok(0) => break,
                    Ok(n) => total += n,
                    Err(_) => break,
                }
            }
            let response = String::from_utf8_lossy(&buf[..total]).to_string();
            // Extract body after \r\n\r\n
            if let Some(idx) = response.find("\r\n\r\n") {
                response[idx + 4..].to_string()
            } else {
                "{}".to_string()
            }
        }
        Err(_) => "{}".to_string(),
    }
}

async fn post_lb_strategy(strategy: &str) -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    match TcpStream::connect(LOADBALANCER_ADDR).await {
        Ok(mut stream) => {
            let body = format!("strategy={}", strategy);
            let request = format!(
                "POST /__lb_strategy HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            if stream.write_all(request.as_bytes()).await.is_err() {
                return "ERROR".to_string();
            }
            let mut buf = vec![0u8; 4096];
            let n = stream.read(&mut buf).await.unwrap_or(0);
            String::from_utf8_lossy(&buf[..n]).to_string()
        }
        Err(_) => "ERROR".to_string(),
    }
}

async fn post_lb_drain(region: &str) -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    match TcpStream::connect(LOADBALANCER_ADDR).await {
        Ok(mut stream) => {
            let body = format!("region={}", region);
            let request = format!(
                "POST /__lb_drain HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            if stream.write_all(request.as_bytes()).await.is_err() {
                return "ERROR".to_string();
            }
            let mut buf = vec![0u8; 4096];
            let n = stream.read(&mut buf).await.unwrap_or(0);
            String::from_utf8_lossy(&buf[..n]).to_string()
        }
        Err(_) => "ERROR".to_string(),
    }
}

async fn post_lb_undrain(region: &str) -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    match TcpStream::connect(LOADBALANCER_ADDR).await {
        Ok(mut stream) => {
            let body = format!("region={}", region);
            let request = format!(
                "POST /__lb_undrain HTTP/1.1\r\nHost: 127.0.0.1:8080\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                body.len(),
                body
            );
            if stream.write_all(request.as_bytes()).await.is_err() {
                return "ERROR".to_string();
            }
            let mut buf = vec![0u8; 4096];
            let n = stream.read(&mut buf).await.unwrap_or(0);
            String::from_utf8_lossy(&buf[..n]).to_string()
        }
        Err(_) => "ERROR".to_string(),
    }
}

// ── Consistency dashboard ───────────────────────────────────────────────────

async fn page_consistency() -> String {
    // Get storage quorum info
    let peers_result = storage::get_peers(STORAGE_ADDR).await;

    // Get cache consistency mode
    let cache_mode = caching::get_mode(CACHING_ADDR).await;

    let body = format!(
        r#"<div class="card">
    <h2>Storage Quorum</h2>
    <div>
        <div class="stat"><div class="label">Peers (N)</div><div class="value">{}</div></div>
        <div class="stat"><div class="label">Write Quorum (W)</div><div class="value">{}</div></div>
        <div class="stat"><div class="label">Read Quorum (R)</div><div class="value">{}</div></div>
    </div>
    <p style="margin-top:12px;color:#666;font-size:13px;">
        W + R = {} + {} = {} {} N+1 = {} &mdash; {}
    </p>
</div>
<div class="card">
    <h2>Cache Consistency Mode</h2>
    <div>
        <div class="stat"><div class="label">Current Mode</div><div class="value">{}</div></div>
    </div>
    <form method="POST" action="/dashboard/consistency" style="margin-top:16px;">
        <div><label>Change Mode</label>
            <select name="mode" style="padding:8px 12px;border:1px solid #ddd;border-radius:4px;font-size:14px;">
                <option value="eventual" {}>Eventual</option>
                <option value="quorum" {}>Quorum</option>
                <option value="strong" {}>Strong</option>
            </select>
        </div>
        <button type="submit" class="btn-primary">Update Mode</button>
    </form>
</div>
<div class="card">
    <h2>Consistency Models</h2>
    <table>
        <tr><th>Mode</th><th>Write Behavior</th><th>Read Behavior</th><th>Latency</th></tr>
        <tr><td>Eventual</td><td>Local + async replicate</td><td>Local only</td><td>Low</td></tr>
        <tr><td>Quorum</td><td>Local + W-1 peer acks</td><td>Local + R-1 peers</td><td>Medium</td></tr>
        <tr><td>Strong</td><td>Local + ALL peer acks</td><td>Local + ALL peers</td><td>High</td></tr>
    </table>
</div>"#,
        peers_result.peer_count + 1, // N = peers + self
        peers_result.quorum_w,
        peers_result.quorum_r,
        peers_result.quorum_w,
        peers_result.quorum_r,
        peers_result.quorum_w + peers_result.quorum_r,
        if peers_result.quorum_w + peers_result.quorum_r > peers_result.peer_count + 1 { "&gt;" } else { "&le;" },
        peers_result.peer_count + 2,
        if peers_result.quorum_w + peers_result.quorum_r > peers_result.peer_count + 1 { "strong consistency guaranteed" } else { "weak consistency" },
        html_escape(&cache_mode.mode),
        if cache_mode.mode == "eventual" { "selected" } else { "" },
        if cache_mode.mode == "quorum" { "selected" } else { "" },
        if cache_mode.mode == "strong" { "selected" } else { "" },
    );

    wrap_dashboard("Consistency", "Consistency", &body)
}

async fn page_consistency_post(post_body: &str) -> String {
    let form = parse_form(post_body);
    if let Some(mode) = form.get("mode") {
        if !mode.is_empty() {
            let _ = caching::set_mode(CACHING_ADDR, mode.clone()).await;
        }
    }
    page_consistency().await
}

// ── Regions dashboard ────────────────────────────────────────────────────────

async fn page_regions() -> String {
    let region = std::env::var("REGION").unwrap_or_else(|_| "local".to_string());
    let peers_str = std::env::var("DISCOVERY_PEERS").unwrap_or_default();

    // Region definitions
    let regions = vec![
        ("SFO", "10.0.0.1"),
        ("NYC", "10.0.0.2"),
        ("AMS", "10.0.0.3"),
    ];

    // Card 1: Regions table with reachability
    let mut regions_rows = String::new();
    for (name, ip) in &regions {
        let discovery_addr = format!("{}:10200", ip);
        let reachable = match tokio::time::timeout(
            std::time::Duration::from_secs(2),
            tokio::net::TcpStream::connect(&discovery_addr),
        )
        .await
        {
            Ok(Ok(_)) => "<span style=\"color:#10B981\">reachable</span>",
            _ => "<span style=\"color:#EF4444\">unreachable</span>",
        };
        regions_rows.push_str(&format!(
            "<tr><td>{}</td><td><code>{}</code></td><td>{}</td></tr>\n",
            name, ip, reachable
        ));
    }

    // Card 2: Mesh latency
    let pairs = vec![
        ("SFO", "NYC", "10.0.0.1:10200", "10.0.0.2:10200"),
        ("SFO", "AMS", "10.0.0.1:10200", "10.0.0.3:10200"),
        ("NYC", "AMS", "10.0.0.2:10200", "10.0.0.3:10200"),
    ];

    let mut latency_rows = String::new();
    for (a, b, _addr_a, addr_b) in &pairs {
        let start = std::time::Instant::now();
        let rtt = match tokio::time::timeout(
            std::time::Duration::from_secs(2),
            tokio::net::TcpStream::connect(addr_b),
        )
        .await
        {
            Ok(Ok(_)) => format!("{:.0}ms", start.elapsed().as_secs_f64() * 1000.0),
            _ => "timeout".to_string(),
        };
        latency_rows.push_str(&format!(
            "<tr><td>{} &harr; {}</td><td>{}</td></tr>\n",
            a, b, rtt
        ));
    }

    // Card 3: Cross-region replication stats
    let tailer_addr = "127.0.0.1:10400";
    let tailer_stats = match rpc::client::send_request(
        tailer_addr,
        rpc::Request {
            procedure_id: 1,
            payload: "0".to_string(),
        },
    )
    .await
    {
        Ok(resp) => resp.payload,
        Err(_) => "unavailable".to_string(),
    };

    let storage_all = discovery::list("storage".to_string()).await;
    let storage_local = discovery::list_local("storage".to_string()).await;
    let all_count = storage_all.addresses.split(';').filter(|s| !s.is_empty()).count();
    let local_count = storage_local.addresses.split(';').filter(|s| !s.is_empty()).count();
    let remote_count = if all_count > local_count { all_count - local_count } else { 0 };

    let cache_mode = caching::get_mode(CACHING_ADDR).await;

    let discovery_peers: Vec<&str> = peers_str.split(',').filter(|s| !s.is_empty()).collect();

    let body = format!(
        r#"
    <div class="card">
        <h2>Regions</h2>
        <table>
            <thead><tr><th>Region</th><th>WireGuard IP</th><th>Discovery</th></tr></thead>
            <tbody>{regions_rows}</tbody>
        </table>
        <p style="margin-top:8px;color:#888">Current region: <strong>{region}</strong></p>
    </div>

    <div class="card">
        <h2>Mesh Latency</h2>
        <table>
            <thead><tr><th>Link</th><th>TCP Connect RTT</th></tr></thead>
            <tbody>{latency_rows}</tbody>
        </table>
        <p style="margin-top:8px;color:#888">Measured via TCP connect to remote discovery port.</p>
    </div>

    <div class="card">
        <h2>Cross-Region Replication</h2>
        <table>
            <tbody>
                <tr><td>Tailer stats</td><td><code>{tailer_stats}</code></td></tr>
                <tr><td>Local storage instances</td><td>{local_count}</td></tr>
                <tr><td>Remote storage instances</td><td>{remote_count}</td></tr>
                <tr><td>Cache consistency mode</td><td>{cache_mode}</td></tr>
                <tr><td>Discovery peers</td><td>{disc_peers}</td></tr>
            </tbody>
        </table>
    </div>
    "#,
        regions_rows = regions_rows,
        region = region,
        latency_rows = latency_rows,
        tailer_stats = tailer_stats,
        local_count = local_count,
        remote_count = remote_count,
        cache_mode = cache_mode.mode,
        disc_peers = if discovery_peers.is_empty() { "none (local mode)".to_string() } else { discovery_peers.join(", ") },
    );

    wrap_dashboard("Regions", "Regions", &body)
}

// ── SEO helpers ─────────────────────────────────────────────────────────────

fn generate_sitemap() -> String {
    let mut urls = vec!["/".to_string()];
    for entry in book_nav_entries() {
        if let NavEntry::Chapter(item) = entry {
            urls.push(item.href.to_string());
        }
    }
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
"#);
    for url in &urls {
        xml.push_str(&format!(
            "  <url><loc>https://p.jjm.net{}</loc></url>\n",
            url
        ));
    }
    xml.push_str("</urlset>\n");
    xml
}

// ── Request routing ─────────────────────────────────────────────────────────

async fn handle_request(
    method: &str,
    path: &str,
    headers: &str,
    body: &str,
) -> (u16, String, Vec<String>) {
    let (base_path, query) = parse_query_string(path);
    let (user_id, is_new) = get_or_create_user_id(headers);
    let mut cookies: Vec<String> = Vec::new();
    if is_new {
        cookies.push(format!("user_id={}; Path=/; Max-Age=31536000", user_id));
    }

    let (status, html) = match (method, base_path) {
        // ── Book pages ──
        ("GET", "/") => (200, landing_page()),
        ("GET", "/foreword") => (
            200,
            book_page("Foreword", "foreword", "/foreword", content::foreword()),
        ),
        ("GET", "/preface") => (
            200,
            book_page("Preface", "preface", "/preface", content::preface()),
        ),
        ("GET", "/chapter/systems") => (
            200,
            book_page("Systems", "systems", "/chapter/systems", content::chapter_systems()),
        ),
        ("GET", "/chapter/configuration") => (
            200,
            book_page(
                "Configuration",
                "configuration",
                "/chapter/configuration",
                content::chapter_configuration(),
            ),
        ),
        ("GET", "/chapter/caching") => (
            200,
            book_page("Caching", "caching", "/chapter/caching", content::chapter_caching()),
        ),
        ("GET", "/chapter/storage") => (
            200,
            book_page("Storage", "storage", "/chapter/storage", content::chapter_storage()),
        ),
        ("GET", "/chapter/design") => (
            200,
            book_page("Design", "design", "/chapter/design", content::chapter_design()),
        ),
        ("GET", "/chapter/consensus") => (
            200,
            book_page("Consensus", "consensus", "/chapter/consensus", content::chapter_consensus()),
        ),
        ("GET", "/chapter/discovery") => (
            200,
            book_page("Discovery", "discovery", "/chapter/discovery", content::chapter_discovery()),
        ),
        ("GET", "/chapter/routing") => (
            200,
            book_page("Routing", "routing", "/chapter/routing", content::chapter_routing()),
        ),
        ("GET", "/chapter/implementation") => (
            200,
            book_page("Implementation", "implementation", "/chapter/implementation", content::chapter_implementation()),
        ),
        ("GET", "/chapter/operation") => (
            200,
            book_page("Operation", "operation", "/chapter/operation", content::chapter_operation()),
        ),
        ("GET", "/chapter/scheduling") => (
            200,
            book_page("Scheduling", "scheduling", "/chapter/scheduling", content::chapter_scheduling()),
        ),
        ("GET", "/chapter/release") => (
            200,
            book_page("Release", "release", "/chapter/release", content::chapter_release()),
        ),
        ("GET", "/chapter/security") => (
            200,
            book_page("Security", "security", "/chapter/security", content::chapter_security()),
        ),
        ("GET", "/chapter/monitoring") => (
            200,
            book_page(
                "Monitoring",
                "monitoring",
                "/chapter/monitoring",
                content::chapter_monitoring(),
            ),
        ),
        ("GET", "/chapter/capacity") => (
            200,
            book_page("Capacity", "capacity", "/chapter/capacity", content::chapter_capacity()),
        ),
        ("GET", "/chapter/utilization") => (
            200,
            book_page("Utilization", "utilization", "/chapter/utilization", content::chapter_utilization()),
        ),
        ("GET", "/chapter/efficiency") => (
            200,
            book_page("Efficiency", "efficiency", "/chapter/efficiency", content::chapter_efficiency()),
        ),
        ("GET", "/chapter/load-testing") => (
            200,
            book_page("Load Testing", "load-testing", "/chapter/load-testing", content::chapter_load_testing()),
        ),
        ("GET", "/chapter/planning") => (
            200,
            book_page("Planning", "planning", "/chapter/planning", content::chapter_planning()),
        ),
        ("GET", "/chapter/degradation") => (
            200,
            book_page("Degradation", "degradation", "/chapter/degradation", content::chapter_degradation()),
        ),
        ("GET", "/chapter/load-balancing") => (
            200,
            book_page("Load Balancing", "load-balancing", "/chapter/load-balancing", content::chapter_load_balancing()),
        ),
        ("GET", "/chapter/consistency") => (
            200,
            book_page("Consistency", "consistency", "/chapter/consistency", content::chapter_consistency()),
        ),
        ("GET", "/chapter/placement") => (
            200,
            book_page("Placement", "placement", "/chapter/placement", content::chapter_placement()),
        ),
        ("GET", "/chapter/global-distribution") => (
            200,
            book_page("Global Distribution", "global-distribution", "/chapter/global-distribution", content::chapter_global_distribution()),
        ),
        ("GET", "/chapter/traffic") => (
            200,
            book_page("Traffic", "traffic", "/chapter/traffic", content::chapter_traffic()),
        ),
        ("GET", "/chapter/faults") => (
            200,
            book_page("Faults", "faults", "/chapter/faults", content::chapter_faults()),
        ),
        ("GET", "/chapter/outages") => (
            200,
            book_page("Outages", "outages", "/chapter/outages", content::chapter_outages()),
        ),
        ("GET", "/chapter/resources") => (
            200,
            book_page("Resources", "resources", "/chapter/resources", content::chapter_resources()),
        ),
        ("GET", "/chapter/servers") => (
            200,
            book_page("Servers", "servers", "/chapter/servers", content::chapter_servers()),
        ),
        ("GET", "/chapter/buildings") => (
            200,
            book_page("Buildings", "buildings", "/chapter/buildings", content::chapter_buildings()),
        ),
        ("GET", "/chapter/network") => (
            200,
            book_page("Network", "network", "/chapter/network", content::chapter_network()),
        ),
        ("GET", "/chapter/power") => (
            200,
            book_page("Power", "power", "/chapter/power", content::chapter_power()),
        ),
        ("GET", "/chapter/infra-management") => (
            200,
            book_page("Management", "infra-management", "/chapter/infra-management", content::chapter_infra_management()),
        ),
        ("GET", "/chapter/maintenance") => (
            200,
            book_page("Maintenance", "maintenance", "/chapter/maintenance", content::chapter_maintenance()),
        ),
        ("GET", "/chapter/edges") => (
            200,
            book_page("Edges", "edges", "/chapter/edges", content::chapter_edges()),
        ),
        ("GET", "/chapter/site-events") => (
            200,
            book_page("Site Events", "site-events", "/chapter/site-events", content::chapter_site_events()),
        ),
        ("GET", "/chapter/detection") => (
            200,
            book_page("Detection", "detection", "/chapter/detection", content::chapter_detection()),
        ),
        ("GET", "/chapter/escalation") => (
            200,
            book_page("Escalation", "escalation", "/chapter/escalation", content::chapter_escalation()),
        ),
        ("GET", "/chapter/root-causes") => (
            200,
            book_page("Root Causes", "root-causes", "/chapter/root-causes", content::chapter_root_causes()),
        ),
        ("GET", "/chapter/remediation") => (
            200,
            book_page("Remediation", "remediation", "/chapter/remediation", content::chapter_remediation()),
        ),
        ("GET", "/chapter/prevention") => (
            200,
            book_page("Prevention", "prevention", "/chapter/prevention", content::chapter_prevention()),
        ),
        ("GET", "/chapter/communication") => (
            200,
            book_page("Communication", "communication", "/chapter/communication", content::chapter_communication()),
        ),
        ("GET", "/afterword") => (
            200,
            book_page("Afterword", "afterword", "/afterword", content::afterword()),
        ),
        ("GET", "/colophon") => (
            200,
            book_page("Colophon", "colophon", "/colophon", content::colophon()),
        ),

        // ── Highlight API ──
        ("GET", "/api/highlights") => {
            let page = query.get("page").map(|s| s.as_str()).unwrap_or("");
            let json = api_get_highlights(&user_id, page).await;
            (200, json)
        }
        ("POST", "/api/highlights") => {
            let json = api_post_highlights(&user_id, body).await;
            (200, json)
        }

        // ── Dashboard pages ──
        ("GET", "/dashboard") => (200, page_dashboard().await),
        ("GET", "/dashboard/config") => (200, page_config("").await),
        ("POST", "/dashboard/config") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                (200, page_config(body).await)
            }
        }
        ("POST", "/dashboard/config/delete") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                (200, page_config_delete(body).await)
            }
        }
        ("GET", "/dashboard/storage") => (200, page_storage("").await),
        ("POST", "/dashboard/storage") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                (200, page_storage(body).await)
            }
        }
        ("POST", "/dashboard/storage/delete") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                (200, page_storage_delete(body).await)
            }
        }
        ("GET", "/dashboard/cache") => (200, page_cache().await),
        ("GET", "/dashboard/health") => (200, page_health().await),

        // ── New service dashboards ──
        ("GET", "/dashboard/scheduling") => (200, page_scheduling().await),
        ("POST", "/dashboard/scheduling") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                (200, page_scheduling_post(body).await)
            }
        }
        ("POST", "/dashboard/scheduling/stop") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                (200, page_scheduling_stop(body).await)
            }
        }
        ("GET", "/dashboard/release") => (200, page_release().await),
        ("POST", "/dashboard/release") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                (200, page_release_post(body).await)
            }
        }
        ("POST", "/dashboard/release/advance") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                (200, page_release_advance(body).await)
            }
        }
        ("POST", "/dashboard/release/rollback") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                (200, page_release_rollback(body).await)
            }
        }
        ("GET", "/dashboard/security") => (200, page_security().await),
        ("POST", "/dashboard/security") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                let (html, auth_cookie) = page_security_post(body).await;
                if let Some(cookie) = auth_cookie {
                    cookies.push(cookie);
                }
                (200, html)
            }
        }
        ("POST", "/dashboard/security/revoke") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                (200, page_security_revoke(body).await)
            }
        }

        // ── Load Balancer + Consistency dashboards ──
        ("GET", "/dashboard/loadbalancer") => (200, page_loadbalancer().await),
        ("POST", "/dashboard/loadbalancer") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                (200, page_loadbalancer_post(body).await)
            }
        }
        ("POST", "/dashboard/loadbalancer/drain") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                let form = parse_form(body);
                if let Some(region) = form.get("region") {
                    let _ = post_lb_drain(region).await;
                }
                (200, page_loadbalancer().await)
            }
        }
        ("POST", "/dashboard/loadbalancer/undrain") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                let form = parse_form(body);
                if let Some(region) = form.get("region") {
                    let _ = post_lb_undrain(region).await;
                }
                (200, page_loadbalancer().await)
            }
        }
        ("GET", "/dashboard/consistency") => (200, page_consistency().await),
        ("POST", "/dashboard/consistency") => {
            if !require_admin(headers).await {
                (403, forbidden_page())
            } else {
                (200, page_consistency_post(body).await)
            }
        }
        ("GET", "/dashboard/regions") => (200, page_regions().await),

        // ── SEO: robots.txt and sitemap.xml ──
        ("GET", "/robots.txt") => (200, "User-agent: *\nAllow: /\nDisallow: /dashboard\nDisallow: /api/\nSitemap: https://p.jjm.net/sitemap.xml\n".to_string()),
        ("GET", "/sitemap.xml") => (200, generate_sitemap()),

        // ── Legacy redirects (old dashboard paths) ──
        ("GET", "/config") => (301, String::new()),
        ("GET", "/storage") => (301, String::new()),
        ("GET", "/cache") => (301, String::new()),
        ("GET", "/health") => (301, String::new()),

        _ => (
            404,
            book_page(
                "Not Found",
                "",
                "",
                "<h1>Not Found</h1><p>The page you requested does not exist.</p>",
            ),
        ),
    };

    (status, html, cookies)
}

// ── Main server ─────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let listen_addr = std::env::var("PORT")
        .map(|p| format!("127.0.0.1:{}", p))
        .unwrap_or_else(|_| LISTEN_ADDR.to_string());
    discovery::register("frontend".to_string(), listen_addr.clone());
    let listener = TcpListener::bind(&listen_addr)
        .await
        .expect("Failed to bind HTTP server");
    println!("Frontend server running on http://{}", listen_addr);

    loop {
        let (mut socket, addr) = match listener.accept().await {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Accept error: {}", e);
                continue;
            }
        };

        tokio::spawn(async move {
            let mut buf = vec![0u8; 65536];
            let n = match socket.read(&mut buf).await {
                Ok(0) => return,
                Ok(n) => n,
                Err(_) => return,
            };

            let request_str = String::from_utf8_lossy(&buf[..n]).to_string();

            // Parse request line
            let first_line = request_str.lines().next().unwrap_or("");
            let parts: Vec<&str> = first_line.split_whitespace().collect();
            if parts.len() < 2 {
                return;
            }

            let method = parts[0];
            let path = parts[1];

            // Parse body for POST requests
            let body = if method == "POST" {
                let content_length: usize = request_str
                    .lines()
                    .find(|line| line.to_lowercase().starts_with("content-length:"))
                    .and_then(|line| line.split(':').nth(1))
                    .and_then(|val| val.trim().parse().ok())
                    .unwrap_or(0);

                if let Some(body_start) = request_str.find("\r\n\r\n") {
                    let body_data = &request_str[body_start + 4..];
                    if body_data.len() >= content_length {
                        body_data[..content_length].to_string()
                    } else {
                        let mut full_body = body_data.to_string();
                        while full_body.len() < content_length {
                            let mut extra = vec![0u8; content_length - full_body.len()];
                            match socket.read(&mut extra).await {
                                Ok(0) => break,
                                Ok(n) => {
                                    full_body
                                        .push_str(&String::from_utf8_lossy(&extra[..n]));
                                }
                                Err(_) => break,
                            }
                        }
                        full_body
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            println!("{} {} {} from {}", method, path, body.len(), addr);

            let (status, html, cookies) =
                handle_request(method, path, &request_str, &body).await;

            // Handle redirects for legacy paths
            if status == 301 {
                let (base, _) = parse_query_string(path);
                let new_path = format!("/dashboard{}", base);
                let response = format!(
                    "HTTP/1.1 301 Moved Permanently\r\nLocation: {}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                    new_path
                );
                let _ = socket.write_all(response.as_bytes()).await;
                return;
            }

            let status_text = match status {
                200 => "OK",
                403 => "Forbidden",
                404 => "Not Found",
                _ => "Error",
            };

            let content_type = if path == "/robots.txt" {
                "text/plain; charset=utf-8"
            } else if path == "/sitemap.xml" {
                "application/xml; charset=utf-8"
            } else if path.starts_with("/api/") {
                "application/json"
            } else {
                "text/html; charset=utf-8"
            };

            let cache_control = if path.starts_with("/dashboard") || path.starts_with("/api/") {
                "no-store"
            } else {
                "public, max-age=300"
            };

            let mut header = format!(
                "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: {}\r\nConnection: close\r\n",
                status, status_text, content_type, html.len(), cache_control
            );

            for cookie in &cookies {
                header.push_str(&format!("Set-Cookie: {}\r\n", cookie));
            }

            header.push_str("\r\n");

            let mut response_bytes = header.into_bytes();
            response_bytes.extend_from_slice(html.as_bytes());

            let _ = socket.write_all(&response_bytes).await;
        });
    }
}
