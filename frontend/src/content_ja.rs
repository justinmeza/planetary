// frontend/src/content_ja.rs
// Japanese translations of all chapter content
// Code snippets remain in English; prose, headings, sidenotes in Japanese.
// Internal links use /ja/ prefix.

pub fn foreword() -> &'static str {
    r##"
<h1>まえがき</h1>

<p><span class="newthought">インターネットは</span>、私たちがコンピュータと関わる方法を根本的に変えました。21世紀以前、コンピュータを使おうとすれば、そのコンピュータは通常、比較的近くにあるか（タイムシェアリングやターミナルベースのメインフレームの場合）、目の前にあるか（パーソナルコンピュータの場合）のどちらかでした。</p>

<p>しかし、ミレニアムの変わり目に何かが変わりました。高帯域幅で低レイテンシーのインターネット接続が一般的になりました。ラップトップ、タブレット、スマートフォン、スマートウォッチなど、インターネットにアクセスするための端末が、世界中の人々がデジタル世界と好きなだけつながっていられるようにしました。</p>

<p>同時に、プロセッサ、メモリ、ストレージ、ネットワークの進歩により、サーバーが密集した施設が世界中に出現するようになりました。それだけでなく、これらの施設の多くは、一定の料金でハードウェアを貸し出しています。インターネット接続、良いアイデア、そしてわずかな資金があれば、次の数十億ドル規模の産業を立ち上げることができるのです。</p>

<p>残念ながら、最も成功したプラクティス、設計パターン、ベストプラクティスは、最大規模の組織にいるごく一部のエンジニアや科学者にしか知られていないことが多いのです。たとえ同じ課題を再現して探求したいと思っても、そのために必要な膨大な数のマシンをどうやって見つけ、どうやって資金を調達すればよいのでしょうか？</p>

<p>本書は、コンピュータシステムの設計、構築、運用を学びたいすべての人のためのものです。実現したいアイデアがある方も、既存のシステムを最高の状態で稼働させたい方も、あるいは何百万台ものサーバーを持つインフラの裏側で何が起きているのかを知りたい方も&mdash;&mdash;本書にはあなたのための内容があります。</p>

<p>コンピュータシステムの分野は常に進化していますが、何百万マシン年もの運用を通じて鍛え上げられた、時の試練に耐えてきた技術の一部をお伝えしたいと思います。同時に、新たに台頭しつつある分野への好奇心を刺激し、あなた自身のアイデアがこれらの分野を前進させることを願っています。</p>

<p>私たちは、コンピューティングの新しいパラダイム&mdash;&mdash;プラネタリスケールコンピューティング&mdash;&mdash;の入り口に立っていると信じています。メインフレームからパーソナルコンピュータへ、パーソナルコンピュータからウェアハウススケールコンピュータへ移行した際にコンピュータの構築と運用の意味を再考する必要があったように、地球規模に広がるコンピュータについても前提を見直す必要があるのです。</p>

<p>私自身の経験と他の方々の経験に基づいて、何が可能で、どうすればそれを実現できるかの全体像を描くことが私の目標です。あなたの心の中で可能性の領域を広げ、問題に対して誤った解決策で妥協しないようにしたいと思います。この旅に一緒に参加していただけることを嬉しく思います。さあ、前へ！</p>

<p class="attribution">&mdash; <em>Justin J. Meza, San Francisco, 2024</em></p>
"##
}

pub fn preface() -> &'static str {
    r##"
<h1>序文</h1>

<p><span class="newthought">本書は</span>、人類が構想した最大かつ最も複雑なコンピュータ組織の設計、構築、運用の方法を記述したものです。そのため、扱うべき情報は自然と膨大になります！「一から」説明するようにあらゆる努力を払いましたが、読者にはコンピュータ、ネットワーク、オペレーティングシステム、プログラミング言語に関する基礎知識があることを前提としています。</p>

<p>プラネタリスケールコンピューティングはまだ黎明期にあるため、本書で紹介するシステムに対する改善点を読者が見つけ、提案してくれることを期待し、歓迎します。本書はベストプラクティスの生きたダイジェストであり、新しい技術が発見され、議論される中で、分野の発展とともに進化していくことを願っています。最新のベストプラクティスとパターンで更新し続けるよう努めます。</p>

<p>本書のすべてのコードは自由に学習できます。重要な概念を理解するためにコードはできるだけシンプルに保つよう努めましたが、それ以上にシンプルにはしていません。そのようなシンプルなコードの欠点については、該当箇所で注記しています。大規模言語モデルは、これほど大規模で一貫性のあるコード体系を作成する上で貴重な資産でした。興味のある読者には、本書で提示される情報の分析、学習、拡張にこれらのツールを活用することを推奨します。</p>
"##
}

pub fn chapter_systems() -> &'static str {
    r##"
<h1>第1章: システム</h1>

<p><span class="newthought">インターネットを使ったことがあるなら</span>、あなたはコンピュータシステムと対話したことがあります。コンピュータシステムとは、何らかの作業を行うために使用されるソフトウェアとハードウェアの集合体です。単一のハードウェア上で動作するコンピュータプログラムとは異なり、コンピュータシステムは複数のプログラム、デバイス、サーバー、さらには地理的な場所にまたがって分散しています。</p>

<p>コンピュータシステムにはさまざまな形態や規模があります。オペレーティングシステムやファイルストレージシステムなど、単一のコンピュータ上で動作する一般的なシステムにはすでに馴染みがあるかもしれません。システムは、異なるハードウェアとソフトウェアのコンポーネントをつなぎ合わせる接着剤です。コンピュータシステムのこの特性は根本的なものです：コンピュータシステムはコンポーネントの集合を管理します。</p>

<p>システムは、プラネタリスケールコンピュータを構築するための主要なビルディングブロックです。システムの重要な特徴は、標準インターフェースを介して合成可能であることです。適切に設計されたシステムは、ハードウェアとソフトウェアの集合に対して、抽象化されカプセル化された関数がプログラムに対して果たすのと同じ役割を果たします。あらゆるシステムの主要な目的は、何らかのタスクを達成するための再利用可能な方法を提供することです。</p>

<h2>アーキテクチャ</h2>

<p>コンピュータシステムはシステムの機能を公開する明確に定義されたインターフェースを提供するため、人間はあるシステムを別のシステムと関連付けるさまざまな方法を考案してきました。もしシステムが物理世界の中空のブロックであれば、ブロック同士を配置する方法は限られています。いくつかのブロックを他のブロックの<em>中に</em>、<em>横に</em>、あるいは<em>上に</em>置くことができるでしょう。</p>

<h3>モノリシック</h3>

<p>モノリシックシステムアーキテクチャは、他のシステムを内部に含む一つの大きなシステムのようなものです。この大きなシステムは、含まれるシステムに対して必要に応じたインターフェースを提供します。大きなシステムが他のすべてのシステムを&ldquo;つなぎ合わせる&rdquo;のです。すべての機能を異なるHTTPエンドポイントに公開するWebアプリは、モノリシックシステムの一例です。</p>

<p>モノリシックシステムは、必要なすべての機能が同じバイナリにバンドルされているため、作業を行うために他のシステムと通信する必要がないという利点があります。欠点は、システムの小さなコンポーネントを更新するためにシステム全体の新バージョンをビルドしてリリースする必要があることです。</p>

<p>コンポーネント間の通信を避ける必要がある場合、バイナリが単一サーバーのリソース内に収まる場合（ただし多くのサーバーに分散は可能）、そしてビルドとリリースプロセスがモノリシックバイナリへの変更量を処理できる場合に、モノリシックシステムを選択するとよいでしょう。適切なビルドとリリースプロセスがあれば、モノリシックアプローチは数百の機能と1日あたり数千の変更をサポートできます。</p>

<h3>マイクロ</h3>

<p>マイクロシステムは、互いに隣り合う小さなブロックのようなものです。各マイクロシステムは、比較的シンプルなインターフェースを持つ特定のカプセル化されたタスクを実行します。永続ストレージに値を格納しアクセスするシステムはマイクロシステムの一例です。モノリシックシステムがすべての機能が同じアドレス空間を共有する単一のバイナリとして存在するのに対し、マイクロシステムは別々のバイナリとして存在し、別々のプロセスとして実行されます。</p>

<p>機能を別々のバイナリに分離することにはトレードオフがあります。異なるマイクロシステムは異なるプログラミング言語で実装できるため、開発の柔軟性が高まります。マイクロシステムは個別に更新・リリースでき、全体のシステム可用性が向上します。さらに、マイクロシステムは完全に別のデバイスにデプロイすることも可能です。</p>

<p>マイクロシステムでは運用がより複雑になります。マイクロシステムは、自らが利用する機能を持つ他のマイクロシステムと通信する必要があり、依存関係が生じます。マイクロシステムは物理的に近くに配置されていない可能性があるため、レイテンシーが重要な懸念事項になります。マイクロサービス間のインタラクションのテストは、現実世界の特性を再現するよう慎重に行う必要があります。</p>

<h3>ティアード</h3>

<p>ティアードシステムは、モノリシックとマイクロシステムのハイブリッドで、作業が少数のティアに分割され、各ティアは&ldquo;上&rdquo;と&ldquo;下&rdquo;のティアとのみ通信します。少ない作業量と少数のコントリビュータでは、ティアードアーキテクチャはマイクロサービスの利点の一部を、モノリシックサービスに関連する低い運用コストで実現できます。</p>

<p>通常、作業量が増え、コントリビュータ数が増えると、ティアードサービスにもモノリシックサービスの欠点が現れます。最初は、別々の作業を独自のティアに切り出すことで追加のティアを増やせます。しかし、サービスディスカバリ、セキュリティ、プライバシーなどの共通機能はすべてのティアで必要になる場合があり、マイクロサービスアプローチが必要となります。</p>

<h2>通信</h2>

<p>システムはプラネタリスケールコンピュータのビルディングブロックです。単一のシステムも有用ですが、システムの集合体は真に強力です。あるシステムが別のシステムの有用性を活用するためには、システム同士が対話する必要があります。別のシステムに依存するシステムを<em>クライアント</em>、依存されるシステムを<em>サーバー</em>と呼びます。クライアントとサーバー間の通信を処理する一般的な方法がいくつか存在します。</p>

<h3>共有ライブラリ</h3>

<p>共有ライブラリにより、あるシステムを別のシステムと同じプロセスとアドレス空間内で実行できます。共有ライブラリは何らかの機能を実装し、システム設計者はそれがシステム自体の一部であるかのように利用できます。コンパイラは、コンパイル時に共有ライブラリを表すオブジェクトをコンパイルしてリンクするか、ランタイムにオペレーティングシステムが以前にコンパイルされたオブジェクトファイルを動的にリンクします。</p>

<p>共有ライブラリは、システム間のあらゆるインタラクション方法の中で最も低い通信オーバーヘッド&mdash;&mdash;単一の関数呼び出し&mdash;&mdash;を持ちます。共有システムの完全なコピーをそれを共有するシステムに含める必要があるため、共有ライブラリはバイナリの肥大化につながる可能性があります。さらに、新しい共有ライブラリ機能を使用するためにはシステムの再コンパイルと再デプロイが必要であり、システム運用者の負担を増大させる可能性があります。</p>

<p>共有ライブラリを他のシステムインタラクション方法と組み合わせると便利な場合が多いです。例えば、サーバーがキャッシュすべきデータをクライアントに返す場合、すべてのクライアントが独自のキャッシュ方法を実装する代わりに、キャッシュ<em>と</em>サーバー通信を処理する共有ライブラリを提供できます。</p>

<h3>プロセス間通信</h3>

<p>プロセス間通信（IPC）は、システムがオペレーティングシステムを使用して互いに通信することに依存しています。厳密に言えば、プロセス間で情報を渡すあらゆる手段は有効なプロセス間通信の形式です：ファイル、パイプ、共有メモリ、ソケットなどがあります。デバイス上で動作し、プロセス間通信を通じて同じデバイス上の他のシステムに機能を提供するシステムは、時として&ldquo;サイドカー&rdquo;と呼ばれます。</p>

<p>共有ライブラリとは異なり、プロセス間通信では、サイドカープロセスをその機能を利用するプロセスとは独立して更新できます。さらに、プロセス間通信は他のシステムインタラクション形式と比較して比較的低い通信オーバーヘッドを持ちます。欠点は、各サイドカーがデバイス上のコンピュート、メモリ、ストレージ、ネットワーク、その他のリソースを競合することです。</p>

<p>デバイスにデプロイする広く使用されるサイドカーは少数にとどめるのが通常最善です。サイドカーはシステムリソースを制限するため、サイドカーシステムの使用を選択した場合は、リソース使用量を追跡し、どの程度が過剰かを判断することが重要です。リソースニーズが高くなりすぎた場合は、サイドカーを別のインタラクション形式に昇格させる必要が生じる場合もあります。</p>

<h3>ネットワーク</h3>

<p>ネットワークは、プロトコルとリレーデバイスを使用して広大な距離にわたって情報を転送します。ネットワークはプラネタリスケールコンピュータのインターコネクトを提供するため、専用の章を設けて詳しく説明します。ここでは、システムがネットワークを使用して互いにインタラクションする方法を見ていきます。パケットベースのネットワークプロトコルを使用するイーサネットネットワークを想定します。</p>

<p>ネットワークを介して通信するために、サーバーはソケットでリッスンしてクライアントの接続を待ちます。クライアントはサーバーのアドレスを使用して情報のパケットをサーバーに送信します。サーバーがパケットを理解した場合、独自のパケットを構築してクライアントに送り返すことができます。このパターンは、クライアント、サーバー、またはネットワークのいずれかが接続を閉じるまで繰り返されます。</p>

<p>ネットワークは<em>分散</em>システムインタラクションの力を解き放ちます。広域ネットワークやインターネットに接続すると、システムは地球の遠く離れた場所、さらには宇宙空間を超えて通信できます。レイテンシー（地球全体で数十から数百ミリ秒に及ぶ可能性がある）に加えて、ネットワークインタラクションの欠点は、通信の信頼性がネットワークとプロトコルの信頼性によって制限されることです。</p>

<h3>正規化</h3>

<p>クライアントは情報を転送したいサーバーとはまったく異なるオペレーティング環境（ハードウェア、アーキテクチャ、オペレーティングシステムなど）上にある可能性があるため、クライアントはサーバーに送信する情報を正規化する必要があります。例えば、クライアントのデバイス上のメモリにあるビットをそのままサーバーに送信した場合、サーバーがクライアントと同じ方法でビットを解釈する保証はありません。</p>

<p>クライアントが送信しサーバーが受信するビットを正規化するために、クライアントは送信する情報に対して<em>シリアライゼーション</em>というプロセスを実行し、サーバーは情報を再構築するために<em>デシリアライゼーション</em>という逆のプロセスを実行します。このプロセスはクライアントとサーバーの裏側で行われます。</p>

<p><a href="/ja/chapter/systems" class="sys" style="color:#E63946">正規化</a>システムを共有ライブラリとして実装できます。<code>proc-macro</code>、<code>syn</code>、<code>quote</code>クレートを使用してRust構文木のイントロスペクションを容易にし、<code>i32</code>、<code>bool</code>、<code>String</code>要素を持つシンプルな<code>struct</code>のシリアライズとデシリアライズのための手続きマクロを実装します。完全な実装では他のデータ型やエスケープされたデリミタも処理します。</p>

<p><span class="sidenote"><strong><code>normalization/src/lib.rs</code></strong></span>
まず、正規化をサポートする<code>struct</code>にアノテーションを付けるための<code>Serializable</code>トレイトと<code>Deserializable</code>トレイトを定義します。これらのトレイトは、<code>struct</code>を<code>String</code>表現との間で変換する関数を提供します。また、シリアライゼーションとデシリアライゼーション中に発生するエラーの<code>enum</code>も定義します。</p>

<pre class="code-normalization"><code>#[derive(Debug, PartialEq)]
pub enum NormalizationError {
    MissingField,
    InvalidFormat,
    ParseFailure,
}

pub trait Serializable {
    fn serialize(&amp;self) -&gt; String;
}

pub trait Deserializable: Sized {
    fn deserialize(input: &amp;str) -&gt; Result&lt;Self, NormalizationError&gt;;
}</code></pre>

<p><span class="sidenote"><strong><code>normalization/normalization-macros/src/lib.rs</code></strong></span>
次に、シンプルな<code>struct</code>のためのシリアライゼーション関数を実装します。まずRustの<code>struct</code>を受け取り、各フィールドについてフィールドの型に応じて値を文字列表現に変換する関数を定義します。特殊文字をエスケープするために<code>String</code>型は別途処理します。</p>

<pre class="code-normalization"><code>fn generate_serialization_for_type(
    field_name: &amp;Option&lt;syn::Ident&gt;,
    field_type: &amp;TypePath,
) -&gt; proc_macro2::TokenStream {
    if field_type.path.is_ident("i32") || field_type.path.is_ident("bool") {
        quote! { format!("{}: {}", stringify!(#field_name), self.#field_name) }
    } else if field_type.path.is_ident("String") {
        quote! {
            format!(
                "{}: \"{}\"",
                stringify!(#field_name),
                self.#field_name.replace("\\", "\\\\").replace(":", "\\:")
                    .replace("\"", "\\\"").replace(",", "\\,")
            )
        }
    } else {
        panic!("Unsupported type!");
    }
}</code></pre>

<p>最後にシリアライゼーションルーチンの核心に到達します。<code>generate_serialization_for_type</code>関数を使用して構造体のフィールドをシリアライズされた文字列に変換する<code>serialize</code>関数を実装する手続きマクロを定義します。Rustコンパイラは<code>Serializable</code>トレイトから派生した構造体に対してこの関数を呼び出します。</p>

<pre class="code-normalization"><code>#[proc_macro_derive(Serializable)]
pub fn derive_serializable(input: TokenStream) -&gt; TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let name = &amp;input.ident;

    let gen = match &amp;input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) =&gt; {
            let serialization_logic = named.iter().map(|f| {
                let field_name = &amp;f.ident;
                match &amp;f.ty {
                    Path(type_path) =&gt;
                        generate_serialization_for_type(field_name, type_path),
                    _ =&gt; panic!("Unsupported type!"),
                }
            });

            quote! {
                impl #name {
                    pub fn serialize(&amp;self) -&gt; String {
                        let parts = vec![#(#serialization_logic),*];
                        format!("{{{}}}", parts.join(","))
                    }
                }
            }
        }
        _ =&gt; panic!("Only named structs are supported!"),
    };

    gen.into()
}</code></pre>

<p>デシリアライゼーションマクロも同様のパターンに従いますが逆方向です：シリアライズされた<code>String</code>を解析し、キー&ndash;値ペアに分割し（エスケープされたカンマやコロンに注意しながら）、型に応じて<code>struct</code>の各フィールドを再構築します。</p>

<p><span class="sidenote"><strong><code>normalization/src/lib.rs</code></strong></span>
<a href="/ja/chapter/systems" class="sys" style="color:#E63946">正規化</a>ライブラリを使用するには、トレイトを<code>use</code>して<code>struct</code>をそれらから<code>derive</code>として宣言するだけです。</p>

<pre class="code-normalization"><code>pub use normalization_macros::{Serializable, Deserializable};

#[derive(Serializable, Deserializable)]
pub struct Sample {
    pub number: i32,
    pub flag: bool,
    pub text: String,
}</code></pre>

<p><span class="sidenote"><strong><code>normalization/tests/tests.rs</code></strong></span>
後で派生したトレイトを使用したい場合は、<code>struct</code>を<code>String</code>表現との間で正規化するために<code>serialize</code>関数または<code>deserialize</code>関数を呼び出すだけです。</p>

<pre class="code-normalization"><code>use normalization::Sample;

#[test]
fn test_serialization() {
    let sample = Sample {
        number: 5,
        flag: true,
        text: "Hello".to_string(),
    };
    let serialized = sample.serialize();
    assert_eq!(serialized, "{number: 5,flag: true,text: \"Hello\"}");
}

#[test]
fn test_deserialization() {
    let serialized = "{number: 5,flag: true,text: \"Hello\"}";
    let deserialized = Sample::deserialize(serialized);

    match deserialized {
        Ok(sample) =&gt; {
            assert_eq!(sample.number, 5);
            assert_eq!(sample.flag, true);
            assert_eq!(sample.text, "Hello".to_string());
        }
        Err(e) =&gt; panic!("Deserialization failed with error: {:?}", e),
    }
}</code></pre>

<p><code>struct</code>の表現を正規化できるようになったので、システム間で簡単に共有できます。次に、あるシステムが別のシステムを使用して作業を行うための抽象インターフェースについて説明します。</p>

<h3>リモートプロシージャコール</h3>

<p>リモートプロシージャコール（RPC）は、ネットワーク通信のエレガントなラッパーを提供します。RPCの目的は、広大な距離にわたって分散している可能性のあるシステム間に関数のようなインターフェースを提供することです。このインターフェースはネットワーク通信の分散性のメリットと共有ライブラリのプログラマビリティのメリットを実現します。</p>

<p>リモートプロシージャコールは1981年にAndrew Nelsonによって提案され、1984年にBirrellとNelsonによって実装されました。概略として、RPCはシステムが互いに離れた別のデバイス上にある可能性があるという事実を抽象化する方法でシステム間の情報転送方法を規定するインターフェースで構成されています。</p>

<p>RPCインターフェースは、クライアントでのシリアライゼーション、クライアントからサーバーへのリクエストのネットワーク転送、サーバーでのデシリアライゼーション、サーバーでの作業、サーバーでのシリアライゼーション、サーバーからクライアントへのレスポンスのネットワーク転送、クライアントでのデシリアライゼーションを実行するコードに変換されます。</p>

<p>RPCはプラネタリスケールコンピュータの基本的な側面です。ネットワークがプラネタリスケールコンピュータのインターコネクトであるならば、RPCはバスプロトコルです。クライアントとサーバー間のシリアライゼーションとデシリアライゼーションには<a href="/ja/chapter/systems" class="sys" style="color:#E63946">正規化</a>システムを使用できます。ネットワークアクセスの詳細は本書の焦点ではないため、ネットワーク通信には<code>tokio</code>ライブラリを使用します。</p>

<p><span class="sidenote"><strong><code>rpc/src/lib.rs</code></strong></span>
まず、RPCクライアントとサーバー間のインターフェースを定義します。クライアントはサーバーにリクエストを送信し、サーバーに実行してほしいプロシージャを識別する番号とクライアントからのデータを含むペイロードを指定します。サーバーはリクエストを処理し、リクエスト結果を含むペイロードを持つレスポンスをクライアントに返します。</p>

<pre class="code-rpc"><code>pub mod client;
pub mod server;

pub type ProcedureId = i32;
pub type Payload = String;

#[derive(Debug, Clone)]
pub struct Request {
    pub procedure_id: ProcedureId,
    pub payload: Payload,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub payload: Payload,
}</code></pre>

<p>プロシージャ識別子とペイロードについての注意点です。新しいプロシージャを作成したり既存のプロシージャ識別子のペイロード構造を更新する場合、新しいユニークなプロシージャ識別子を選択することが重要です。ペイロードの変更は、新しいインターフェースを使用するクライアントと古いインターフェースを実装するサーバー間で後方互換性が<em>ない</em>ためです。</p>

<p>また、クライアントが古いインターフェースにリクエストを送信しなくなったことが確実になるまで、サーバー上の古いインターフェースの既存の実装を利用可能な状態に保つ必要があります。このため、常に対応する新バージョンのクライアントをデプロイする前に新バージョンのサーバーをデプロイすべきです&mdash;&mdash;古いクライアントのリクエストは新しいサーバーで引き続き処理できますが、新しいクライアントのリクエストは古いサーバーでは処理できません。</p>

<p><span class="sidenote"><strong><code>rpc/src/server.rs</code></strong></span>
RPCサーバーはソケットでリクエストをリッスンし、リクエストを処理するためのスレッドを生成し、リクエストハンドラを呼び出します。リクエストハンドラはサーバーによって実装されます。クライアントがサーバーに実行を求めるプロシージャに応じて、サーバーは異なるコードを実行します。</p>

<pre class="code-rpc"><code>pub async fn start_server(
    addr: &amp;str,
    handler: impl Fn(Request) -&gt; Response + Send + Sync + 'static + Clone,
) -&gt; io::Result&lt;()&gt; {
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        let handler = handler.clone();

        tokio::spawn(async move {
            loop {
                let mut buffer = vec![0u8; 1024];
                let n = socket.read(&amp;mut buffer).await
                    .expect("Failed to read from socket");
                let data = String::from_utf8_lossy(&amp;buffer[..n]);

                let parts: Vec&lt;&amp;str&gt; = data.splitn(2, ':').collect();
                let request = Request {
                    procedure_id: parts[0].parse().unwrap(),
                    payload: parts[1].trim().to_string(),
                };

                let response = handler(request);
                socket.write_all(response.payload.as_bytes()).await
                    .expect("Failed to write to socket");
            }
        });
    }
}</code></pre>

<p>各スレッドがネットワークソケットを開いたまま維持し、追加データを待機する（またはソケットが閉じられるのを待つ）ことに注目してください。これによりクライアントは各リクエストごとにネットワーク接続を確立することなく、サーバーにリクエストのストリームを送信できます。これにより追加リクエストを処理するためのレイテンシーとリソースが削減されますが、リクエストが長時間実行される場合はサーバーのソケットが不足するリスクがあります。</p>

<p><span class="sidenote"><strong><code>rpc/src/client.rs</code></strong></span>
RPCクライアントは特定のアドレスのサーバーに接続し、クライアントが識別するプロシージャをクライアントが提供するペイロードで実行するリクエストを送信します。</p>

<pre class="code-rpc"><code>pub async fn send_request(
    server_addr: &amp;str,
    request: Request,
) -&gt; io::Result&lt;Response&gt; {
    let mut stream = TcpStream::connect(server_addr).await?;
    let serialized = format!("{}:{}\n", request.procedure_id, request.payload);
    stream.write_all(serialized.as_bytes()).await?;

    let mut buffer = vec![0u8; 1024];
    let n = stream.read(&amp;mut buffer).await?;
    let response_data = String::from_utf8_lossy(&amp;buffer[..n]);

    Ok(Response {
        payload: response_data.to_string(),
    })
}</code></pre>

<h3>クライアント&ndash;サーバー</h3>

<p>最も一般的なシステムアーキテクチャは<em>クライアント&ndash;サーバー</em>アーキテクチャです。その名の通り、クライアントデバイスがサーバーデバイスと通信する構造です。このアーキテクチャでは、クライアントがサーバーにリクエストを送信し、サーバーがクライアントにレスポンスを返します。クライアントはコマンドラインインターフェース、スタンドアロンバイナリ、またはリクエストを送信したい別のサーバーである可能性があります。</p>

<p>クライアント&ndash;サーバーアーキテクチャの理解を深め、正規化とRPCをつなぎ合わせるために、シンプルな<a href="/ja/chapter/systems" class="sys" style="color:#00B4D8">echo</a>クライアントとサーバーを見てみましょう。<a href="/ja/chapter/systems" class="sys" style="color:#00B4D8">echo</a>クライアントは<a href="/ja/chapter/systems" class="sys" style="color:#00B4D8">echo</a>サーバーに返してほしい文字列をペイロードとするリクエストを送信します。<a href="/ja/chapter/systems" class="sys" style="color:#00B4D8">echo</a>サーバーはその文字列を検査しクライアントへのレスポンスのペイロードとして返します。</p>

<p><span class="sidenote"><strong><code>echo/src/lib.rs</code></strong></span>
まず<a href="/ja/chapter/systems" class="sys" style="color:#00B4D8">echo</a>システムのコンポーネントを共有ライブラリで指定します。ライブラリはクライアントがリクエストできサーバーが実行できるプロシージャの識別子を提供します。また、リクエストとレスポンスのペイロードの構造も定義します。</p>

<pre class="code-echo"><code>use normalization::{Deserializable, NormalizationError, Serializable};
use rpc::ProcedureId;

pub const SYSTEM_NAME: &amp;str = "echo";
pub const SYSTEM_ADDRESS: &amp;str = "127.0.0.1:10100";
pub const ECHO_PROCEDURE: ProcedureId = 1;

#[derive(Serializable, Deserializable)]
pub struct EchoArgs {
    pub message: String,
}</code></pre>

<p><span class="sidenote"><strong><code>echo/bin/server_v0.rs</code></strong></span>
<a href="/ja/chapter/systems" class="sys" style="color:#00B4D8">echo</a>サーバーではクライアントがリクエストを送信するのを待ちます。サーバーがリクエストを受け取るとハンドラ関数を呼び出してリクエストを処理します。ハンドラはリクエストされたプロシージャがサーバーが認識するものかどうかを確認し、そうであればサーバーはリクエストをデシリアライズしてそのメッセージをレスポンスに格納します。</p>

<pre class="code-echo"><code>fn handle_echo(args: EchoArgs) -&gt; String {
    args.message
}

fn handler(request: Request) -&gt; Response {
    match request.procedure_id {
        ECHO_PROCEDURE =&gt; {
            let args: EchoArgs =
                EchoArgs::deserialize(&amp;request.payload)
                    .expect("Failed to deserialize");
            Response { payload: handle_echo(args) }
        }
        _ =&gt; Response {
            payload: "Unknown procedure".to_string(),
        },
    }
}</code></pre>

<p><span class="sidenote"><strong><code>echo/bin/client_v0.rs</code></strong></span>
<a href="/ja/chapter/systems" class="sys" style="color:#00B4D8">echo</a>クライアントは<a href="/ja/chapter/systems" class="sys" style="color:#00B4D8">echo</a>サーバーに送信するリクエストを構築して送信する役割を担います。クライアントはメッセージをシリアライズし、プロシージャ識別子を指定し、リクエストをサーバーに送信し、サーバーのレスポンスを待ち、レスポンスのペイロードを出力します。</p>

<pre class="code-echo"><code>let args = EchoArgs {
    message: "Hello RPC!".to_string(),
};
let serialized_args = args.serialize();

let request = Request {
    procedure_id: ECHO_PROCEDURE,
    payload: serialized_args,
};

let response = client::send_request(SYSTEM_ADDRESS, request)
    .await
    .expect("Failed to get response");
println!("Response: {}", response.payload);</code></pre>

<p>クライアント&ndash;サーバーアーキテクチャはプラネタリスケールコンピュータを構築するための強力な抽象化です。作業を個別のエンティティに分割して管理性を向上させ、シンプルなインターフェースを公開してモジュール性と再利用性を促進し、柔軟な数のスレッドやデバイスで実行できるようにして分離性とスケーラビリティを提供します。</p>

<h3>状態</h3>

<p>先ほど構築した<a href="/ja/chapter/systems" class="sys" style="color:#00B4D8">echo</a>システムは非常にシンプルでリクエスト間の状態管理を必要としませんでした。しかし多くの実際のシステム（これから見ていくものを含む）は作業を行うために何らかの状態を管理する必要があることが多いです。アトミック参照カウンタを使用してリクエストスレッド間で安全に状態を共有する簡単な方法があります。リクエスト間で状態を共有できる関数で<a href="/ja/chapter/systems" class="sys" style="color:#F4845F">rpc</a>共有ライブラリを改良します。</p>

<p><span class="sidenote"><strong><code>rpc/src/server.rs</code></strong></span>
<code>start_server</code>との主な違いは共有したい状態を表すジェネリック型パラメータ<code>&lt;T&gt;</code>を使用していることです。共有状態を渡すためのパラメータを関数に追加しハンドラの型宣言も共有状態を受け取るように変更します。ハンドラを呼び出す前にハンドラが独自のコピーを持てるように共有状態をクローンします。</p>

<pre class="code-rpc"><code>pub async fn start_server_with_state&lt;T: Send + 'static&gt;(
    addr: &amp;str,
    handler: impl Fn(Request, Arc&lt;Mutex&lt;T&gt;&gt;) -&gt;
        Pin&lt;Box&lt;dyn Future&lt;Output = Response&gt; + Send&gt;&gt;
        + Send + Sync + 'static + Clone,
    shared_state: Arc&lt;Mutex&lt;T&gt;&gt;,
) -&gt; io::Result&lt;()&gt; {
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (mut socket, _) = listener.accept().await?;
        let handler = handler.clone();
        let shared_state = shared_state.clone();

        tokio::spawn(async move {
            let mut buffer = vec![0u8; 1024];
            loop {
                match socket.read(&amp;mut buffer).await {
                    Ok(n) if n == 0 =&gt; break,
                    Ok(n) =&gt; {
                        let data = String::from_utf8_lossy(&amp;buffer[..n]);
                        let parts: Vec&lt;&amp;str&gt; = data.splitn(2, ':').collect();
                        let request = Request {
                            procedure_id: parts[0].parse().unwrap(),
                            payload: parts[1].trim().to_string(),
                        };
                        let response = handler(request, shared_state.clone()).await;
                        if socket.write_all(response.payload.as_bytes())
                            .await.is_err() { break; }
                    }
                    Err(_) =&gt; break,
                }
            }
        });
    }
}</code></pre>

<h2>転送</h2>

<p>あるシステムから別のシステムに転送される情報の旅は魅力的な冒険です。複数の段階を経て潜在的に何千マイルもの距離を移動し、途中で配信の成功を妨げる危険を回避しなければなりません。次にあるシステムから別のシステムへの情報の時系列フローとそれに関わる概念やシステムをたどります。</p>

<h3>ディスカバリ</h3>

<p>クライアントは通信したいサーバーの位置を発見する方法を知る必要があります。しかしサーバーはデバイス間で移動する可能性があり、また多数のデバイスでサーバーのコピーが実行されている可能性もあります&mdash;&mdash;この変動の中でクライアントはどのようにして正しいサーバーを見つけることができるのでしょうか？プラネタリスケールコンピュータの実装において問題に直面した場合、私たちは通常システムで問題を解決しようとします。</p>

<p><a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>システムはクライアントが接続したいシステムを受け取りクライアントが接続するデバイスを選択し選択したデバイスでクライアントに応答する役割を担います。したがってディスカバリシステムはシステムの識別子を入力として受け取りクライアントがサーバーと通信するために接続できるデバイスのアドレスで応答します。</p>

<p><span class="sidenote"><strong><code>discovery/src/lib.rs</code></strong></span>
まず<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>システムが実装するプロシージャの識別子を定義します。サーバーはregisterプロシージャを使用して実装するシステムの名前とリッスンしているアドレスとポートを送信します。queryプロシージャによりクライアントはシステムを名前でリクエストしそのシステムを実装するサーバーのアドレスとポートを含むレスポンスを受け取ることができます。</p>

<pre class="code-discovery"><code>#[derive(Debug, Serializable, Deserializable)]
pub struct RegisterArgs {
    pub name: String,
    pub address: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct QueryArgs {
    pub name: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct QueryResult {
    pub address: String,
}</code></pre>

<p><a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>システムは登録されたシステムを追跡するために共有状態を使用します。レジストリにはシステム名からサーバーアドレスへのマッピングとアドレスが最後に<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>システムに登録した時刻が含まれています。アドレスがリクエストされるとレジストリはランダムに一つを選択して返します。クリーンアップ関数は古い登録アドレスを削除します。</p>

<pre class="code-discovery"><code>#[derive(Default)]
pub struct Registry {
    registry: HashMap&lt;Name, Vec&lt;Address&gt;&gt;,
    last_ping: HashMap&lt;Address, Instant&gt;,
}

impl Registry {
    fn register(&amp;mut self, name: Name, address: Address) {
        if let Some(time) = self.last_ping.get_mut(&amp;address) {
            *time = Instant::now();
        } else {
            self.registry.entry(name)
                .or_insert_with(Vec::new)
                .push(address.clone());
            self.last_ping.insert(address, Instant::now());
        }
    }

    fn get_address(&amp;self, name: &amp;Name) -&gt; Option&lt;&amp;Address&gt; {
        self.registry.get(name)?.choose(&amp;mut rand::thread_rng())
    }

    fn cleanup_stale(&amp;mut self) {
        let now = Instant::now();
        let stale_addresses: HashSet&lt;_&gt; = self.last_ping.iter()
            .filter(|&amp;(_, time)| now.duration_since(*time) &gt; CLEANUP_DURATION)
            .map(|(address, _)| address.clone())
            .collect();

        for address in stale_addresses {
            self.last_ping.remove(&amp;address);
            self.registry.retain(|_, v| {
                v.retain(|a| a != &amp;address);
                !v.is_empty()
            });
        }
    }
}</code></pre>

<p><a href="/ja/chapter/systems" class="sys" style="color:#F4845F">rpc</a>共有ライブラリにより<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>サービスのロジック実装が容易になります。リクエストのプロシージャ識別子に基づくヘルパー関数と共有レジストリ状態を使用したregisterおよびqueryプロシージャの処理関数を定義します。レジストリは複数のスレッド間で共有されるためリクエストハンドラ関数はリクエスト処理前にレジストリへの排他的アクセスを確保します。</p>

<p>サーバーが利用可能になると対応するシステム識別子へのリクエストを処理できるものとして<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>システムにアドレスを登録します。サーバーは追加と削除が行われるためシステムは定期的に<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>サービスに再登録する必要があります。もちろん<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>システム自体も利用不可能になることがあります。その場合サーバーは指数関数的に増加する間隔で最大量まで<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>システムへの接続を再試行します。</p>

<p>しかし重要な問題としてサービスは最初にどのようにして<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>サービスを見つけるのでしょうか？一つの選択肢はクライアントまたはサーバーが常に<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>サービスに到達できるアドレスのセットを維持することです。他にDNSやマルチキャストを使用して<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>サービスを特定する方法もあります。</p>

<h3>ルーティング</h3>

<p>サーバーの可用性は時間とともに変化する可能性がありある時点で発見した利用可能なサーバーが後で利用不可能になることがあります。そのためクライアントのサーバー発見とサーバーへのリクエストルーティングの間に抽象化レイヤーを構築することが有用です。このサービスを<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>サービスと呼びその主な役割はクライアントからサーバーへリクエストを効率的かつ確実に届けることです。</p>

<p><a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>サービス（クライアントは最初に<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>サービスで発見できます）はサービス識別子をそのサービスの利用可能なサーバーに解決します。<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>サービスとは異なり<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>サービスはサービスのサーバーセットのヘルスも確認し利用可能なサーバーにのみリクエストをルーティングします。さらに<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>サービスはコネクションプーリングとロードバランシングを実行します。</p>

<p><span class="sidenote"><strong><code>routing/src/lib.rs</code></strong></span>
<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>システムはコネクションプールの概念に基づいています。各システムには独自のコネクションプールがあり、各コネクションはシステムのサーバーへのアクティブなネットワークソケットです。ソケットの再利用により接続確立のオーバーヘッドが排除されます。</p>

<pre class="code-routing"><code>pub struct ConnectionPool {
    system_name: String,
    pool: Arc&lt;Mutex&lt;VecDeque&lt;TcpStream&gt;&gt;&gt;,
    max_size: usize,
    semaphore: Arc&lt;Semaphore&gt;,
}

impl ConnectionPool {
    pub fn new(system_name: String, max_size: usize) -&gt; Self {
        Self {
            system_name,
            pool: Arc::new(Mutex::new(VecDeque::new())),
            max_size,
            semaphore: Arc::new(Semaphore::new(max_size)),
        }
    }

    pub async fn get(&amp;self) -&gt; Option&lt;TcpStream&gt; {
        self.semaphore.acquire().await
            .expect("Unable to acquire permit").forget();
        let mut pool = self.pool.lock().await;
        let mut conn = pool.pop_front();
        if conn.is_none() {
            let address = discovery::query(self.system_name.clone()).await;
            conn = TcpStream::connect(&amp;address.address).await.ok();
        }
        conn
    }

    pub async fn release(&amp;self, conn: TcpStream) {
        let mut pool = self.pool.lock().await;
        if pool.len() &lt; self.max_size {
            pool.push_back(conn);
        }
        self.semaphore.add_permits(1);
    }
}</code></pre>

<p><a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>システムはプロキシシステムと共有ライブラリの2つのモードで動作できます。プロキシ版はRPCインターフェースを使用し、共有ライブラリ版はクライアント上で全機能を実行します。</p>

<p><span class="sidenote"><strong><code>routing/src/lib.rs</code></strong></span>
共有ライブラリはプロキシと同様の機能をクライアントがインスタンス化する構造体内で実行します。</p>

<pre class="code-routing"><code>pub struct Router {
    pools: Arc&lt;Mutex&lt;HashMap&lt;String, ConnectionPool&gt;&gt;&gt;,
}

impl Router {
    pub fn new() -&gt; Self {
        Self {
            pools: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn send_request(
        &amp;self,
        name: String,
        procedure_id: ProcedureId,
        payload: &amp;str,
    ) -&gt; Response {
        let mut pools = self.pools.lock().await;
        let pool = pools.entry(name.clone())
            .or_insert_with(|| ConnectionPool::new(name.clone(), 10));
        route_request(
            RouteArgs { name, procedure_id, payload: payload.to_string() },
            pool,
        ).await
    }
}</code></pre>

<p><a href="/ja/chapter/systems" class="sys" style="color:#00B4D8">echo</a>システムを更新して<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>と<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>システムを使用し通信プロセスを簡素化できます。</p>

<pre class="code-echo"><code>// echo client using routing
let args = EchoArgs {
    message: "Hello RPC!".to_string(),
};
let serialized_args = args.serialize();

let routing = Router::new();
let response = routing
    .send_request("echo".to_string(), ECHO_PROCEDURE, serialized_args.as_str())
    .await;
println!("Response: {}", response.payload);</code></pre>

<h3>配信</h3>

<p>クライアントがサーバーを発見してリクエストをルーティングした後、リクエストはサーバープロセスに配信される必要があります。サーバーが前のリクエストを処理している間に別のリクエストが到着する可能性があるため、アクティブなリクエストのキューを維持することが有用です。新しいリクエストはキューに入れられ配信されたリクエストはキューから削除されます。</p>

<p>配信キューはリクエストが送信されるクライアント側、またはリクエストが受信されるサーバー側のいずれかで維持できます。サーバーにキューを配置するとフロー制御の管理が不要になりますが、適切なロードバランシングがないとサーバーキューが満杯になる可能性があります。</p>

<h3>並行性</h3>

<p>サーバーはリクエストキューとワーカータスクを使用して複数のリクエストを並行処理できます。サーバーは受信接続を受け入れリクエストを有界チャネルに入れ、自然なバックプレッシャーを提供します。別のワーカータスクがリクエストをデキューして処理します：</p>

<pre class="code-rpc"><code>// Create a channel to hold in-flight requests.
let (tx, mut rx) = mpsc::channel::&lt;InflightRequest&gt;(100);

// Spawn a worker task to process requests.
task::spawn(async move {
    while let Some(request) = rx.recv().await {
        let request_str = String::from_utf8_lossy(&amp;request.data)
            .trim().to_string();
        let req: Request = Request::deserialize(&amp;request_str).unwrap();
        let response = Response { result: req.a * req.b };
        let res_str = response.serialize();
        request.client_socket.write_all(res_str.as_bytes()).await.unwrap();
    }
});</code></pre>

<h3>タイムアウト</h3>

<p>プラネタリスケールコンピュータの障害によりサーバーの応答がクライアントの期待よりも長くなったり、まったく応答しなくなったりする可能性があります。ソフトウェアのバグ、ハードウェア障害、ネットワークや電源の障害などが原因となり得ます。</p>

<p>予測不可能なバグ、障害、停止に対して回復力を持つためにシステムはタイムアウトを使用して作業の試行に費やす時間を制限できます。タイムアウトはリクエストに時間がかかりすぎていることをクライアントに知らせ、貴重なキュースロットとシステムリソースの占有を防ぎます。</p>

<pre class="code-rpc"><code>// Wrap the processing in a timeout.
let process_result = timeout(REQUEST_TIMEOUT, async {
    let request_str = String::from_utf8_lossy(&amp;request.data)
        .trim().to_string();
    let req: Request = Request::deserialize(&amp;request_str).unwrap();
    let response = Response { result: req.a * req.b };
    let res_str = response.serialize();
    request.client_socket.write_all(res_str.as_bytes()).await
}).await;

// Send a fatal response to the client on timeout.
if process_result.is_err() {
    let fatal_msg = "Fatal: Request timed out".as_bytes();
    let _ = request.client_socket.write_all(fatal_msg).await;
}</code></pre>

<h3>リトライ</h3>

<p>リクエストがタイムアウトしたり一時的なエラーが発生した場合クライアントはリクエストを再試行できます。しかしリトライは慎重に実装する必要があります。多くのクライアントが同時にリトライすると、サーバーを圧倒する&ldquo;サンダリングハード&rdquo;が発生する可能性があります。これを緩和するためにリトライの遅延にランダムな<em>ジッター</em>を追加しリトライを時間的に分散させます：</p>

<pre class="code-rpc"><code>const MAX_RETRIES: u32 = 3;
const BASE_RETRY_DELAY: Duration = Duration::from_secs(1);
const JITTER_MS: u64 = 200;

for attempt in 1..=MAX_RETRIES {
    let response = send_request(&amp;request).await;
    match response {
        Ok(response) =&gt; {
            println!("5 * 10 = {}", response.result);
            break;
        },
        Err(e) if e == "Fatal: Request timed out" =&gt; {
            let jitter = rand::thread_rng().gen_range(0..JITTER_MS);
            let delay = BASE_RETRY_DELAY + Duration::from_millis(jitter);
            println!("Attempt {} failed. Retrying in {:?}...", attempt, delay);
            tokio::time::sleep(delay).await;
        },
        Err(e) =&gt; {
            println!("Error: {}", e);
            break;
        }
    }
}</code></pre>

<p>システムが成長するにつれて最終的には複数の地理的リージョンにまたがる必要が出てきます。<a href="/ja/chapter/geo-replication">第24章: ジオレプリケーション</a>ではレイテンシーを低く保ちデータの一貫性を維持しながらフルシステムスタックをリージョン間でレプリケーションする方法を探ります。</p>
"##
}

pub fn chapter_design() -> &'static str {
    r##"
<h1>第2章: 設計</h1>

<p><span class="newthought">コードを1行書く前に</span>、システムエンジニアは根本的な問いに答えなければなりません：<em>どんな問題を解決するのか？</em>設計プロセスは、曖昧さが明確さに変わり、要件がインターフェースに蒸留され、トレードオフが変更コストが高くなる前に明示される場所です。優れた設計は、信頼性の高いシステムが構築される基盤です。</p>

<p>プラネタリスケールコンピューティングの文脈では、設計に追加の次元が加わります。単一のマシンで動作するシステムが何千ものサーバーに分散させると壊滅的に失敗することがあります。10人のユーザーにとって洗練された設計が1000万人では崩壊することがあります。設計プロセスは最初からスケール、障害、進化を見越す必要があります。</p>

<h2>問題定義</h2>

<p>すべてのシステムは問題から始まります。問題定義はシステムが達成すべきこと、誰にサービスするか、どんな制約の下で動作するかを定義します。よく書かれた問題定義は設計の決定を導くのに十分具体的でありながら、実装の詳細への早期のコミットメントを避けるのに十分一般的です。</p>

<p>本書で構築しているシステムを考えてみましょう。<a href="/ja/chapter/systems" class="sys" style="color:#00B4D8">echo</a>システムの問題定義はシンプルです：クライアントからメッセージを受け取り同じメッセージを返す。<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>システムの問題はより微妙です：サーバーが増減する中でシステム名を受け取り、そのシステムを実装する健全なサーバーのアドレスを返す。<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>システムの問題にはさらに別の次元が加わります：プロセスの再起動やハードウェア障害にまたがってデータを永続的に保存する。</p>

<p>各問題定義は暗黙的にシステムの<em>スコープ</em>を定義します。ディスカバリシステムはサーバーを発見しますがリクエストをルーティングはしません。ストレージシステムはデータを保存しますがメモリにキャッシュはしません。スコープを狭く保つことは最も重要な設計原則の一つです。システムが多くのことをやろうとすると理解しにくくテストしにくく運用しにくくなります。</p>

<h2>設計ドキュメント</h2>

<p>設計ドキュメントは問題定義を具体的な計画に変換します。通常4つのセクションを含みます：システムが公開するインターフェース、維持するデータ構造、使用するアルゴリズム、受け入れるトレードオフです。設計ドキュメントは設計者と実装者の間の契約です&mdash;&mdash;たとえ同一人物であっても。</p>

<p>インターフェースセクションは他のシステムがこのシステムとどのようにインタラクションするかを定義します。私たちのシステムではインターフェースは型付き引数と結果を持つRPCプロシージャとして定義されます。<a href="/ja/chapter/configuration" class="sys" style="color:#3A86FF">コンフィグレーション</a>サービスは5つのプロシージャを定義します：get、set、delete、list、watch。各プロシージャには一意の識別子、リクエスト構造体、レスポンス構造体があります。</p>

<p>データ構造セクションはシステムがどんな状態を維持しどのように整理するかを記述します。<a href="/ja/chapter/caching" class="sys" style="color:#7209B7">キャッシング</a>サービスは高速検索のためのハッシュマップとLRU順序のためのデキューを維持します。<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>サービスはインメモリハッシュマップ、ライトアヘッドログ、定期的なスナップショットを維持します。</p>

<p>アルゴリズムセクションはシステムがリクエストをどのように処理するかを記述します。多くのサービスではこれは単純です：リクエストのデシリアライズ、データ構造に対する操作、レスポンスのシリアライズ。<a href="/ja/chapter/consensus" class="sys" style="color:#06D6A0">コンセンサス</a>のようなより複雑なシステムでは選挙プロトコル、ログレプリケーション、状態マシンの適用が記述されます。</p>

<p>トレードオフセクションはおそらく最も重要です。すべての設計決定にはトレードオフが伴い、それを明示することで後の驚きを防ぎます。<a href="/ja/chapter/caching" class="sys" style="color:#7209B7">キャッシング</a>サービスはメモリと速度をトレードします。<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>サービスは書き込みレイテンシーと耐久性をトレードします。<a href="/ja/chapter/configuration" class="sys" style="color:#3A86FF">コンフィグレーション</a>サービスは整合性と可用性をトレードします。</p>

<h2>インターフェースファースト設計</h2>

<p>私たちのシステム全体に現れるパターンは<em>インターフェースファースト設計</em>です。共有ライブラリ（<code>lib.rs</code>）がサーバー（<code>main.rs</code>）が実装する前にインターフェースを定義します。これにはいくつかの利点があります。</p>

<p>第一に、設計者にシステムをクライアントの視点から考えることを強制します。クライアントはどんな操作を必要とするか？どんなデータを送受信するか？この外側からの思考は実装から始めて外側に向かう場合よりもクリーンなインターフェースを生み出します。</p>

<p>第二に、並行開発を可能にします。インターフェースが定義されたらクライアントはスタブやモックを使用してインターフェースに対して記述でき、サーバーの実装と並行して進められます。</p>

<p>第三に、自然なバージョニング境界を提供します。インターフェースが変更されるとプロシージャ識別子が変更され、新旧両方のバージョンが移行中に共存できます。</p>

<h2>リソース</h2>

<p>すべてのシステムはリソースを消費します：CPU、メモリ、ストレージ、ネットワーク帯域幅、ファイルディスクリプタなど。優れた設計はリソース使用を考慮し予算を設定します。<a href="/ja/chapter/caching" class="sys" style="color:#7209B7">キャッシング</a>サービスにはメモリ使用量を制限する<code>MAX_CAPACITY</code>があります。<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスにはメトリックごとのデータポイント数を制限する<code>MAX_METRIC_WINDOW</code>があります。</p>

<p>リソース予算は相互に作用します。メモリを少なく使うシステムはCPUを多く必要とするかもしれません（圧縮のため）。ネットワーク帯域幅を少なく使うシステムはストレージを多く必要とするかもしれません（バッチ処理のため）。これらの相互作用を理解することは設計プロセスの重要な部分です。</p>

<h2>管理</h2>

<p>管理できないシステムはスケールで運用できません。設計には最初から管理の懸念を含める必要があります：システムはどのように設定されるか？ヘルスはどのように監視されるか？どのようにデプロイ・更新されるか？</p>

<p>私たちのシステムは<a href="/ja/chapter/configuration" class="sys" style="color:#3A86FF">コンフィグレーション</a>、<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>、<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>サービスとの統合を通じてこれらの懸念に対処します。各サービスは起動時にディスカバリに登録しモニタリングにメトリックを報告しコンフィグレーションからランタイムパラメータを読み取ります。この管理インフラはビジネスロジック自体と同じくらい重要です。</p>

<p>設計プロセスは反復的です。最初の設計が最終設計になることはめったにありません。実装が予期せぬ課題を明らかにしテストがエッジケースを露出し運用が実世界の動作を表面化させる中で設計は進化します。重要なのは設計を明示的で改訂可能にすることであり最初の試みで完璧にすることではありません。</p>
"##
}

pub fn chapter_consensus() -> &'static str {
    r##"
<h1>第3章: コンセンサス</h1>

<p><span class="newthought">分散システムでは</span>、複数のサーバーがしばしば共有状態について合意する必要があります&mdash;&mdash;誰がリーダーか、ログにどんなエントリがあるか、トランザクションをコミットすべきかどうか。障害が存在する中でこの合意に到達することが<em>コンセンサス</em>の問題です。コンセンサスは分散コンピューティングにおける最も基本的で困難な問題の一つであり、信頼性の高いプラネタリスケールコンピュータを構築するために正しく実装することが不可欠です。</p>

<p>困難さは分散システムの性質に起因します：メッセージは遅延、並べ替え、または消失する可能性があり、サーバーはクラッシュして再起動する可能性があり、グローバルクロックは存在しません。これらの課題にもかかわらず、コンセンサスアルゴリズムによりサーバーのグループ（<em>アンサンブル</em>または<em>クラスタ</em>と呼ばれる）は一部のメンバーが障害を起こしても単一の一貫したユニットとして振る舞うことができます。</p>

<h2>クォーラムベースのコンセンサス</h2>

<p>コンセンサスに対する最も広く使用されるアプローチはクォーラムベースの投票です。重要な洞察は、サーバーの過半数が決定に同意すれば2つの過半数は少なくとも1つのサーバーで重なるということです。この重なりにより一部のサーバーが障害を起こしても決定が失われないことが保証されます。5台のサーバーシステムは2つの障害に耐えられ、3台のシステムは1つの障害に耐えられます。</p>

<p>私たちの実装はDiego OngaroとJohn Ousterhoutによって理解しやすさのために設計されたRaftコンセンサスアルゴリズムに従います。Raftはコンセンサス問題を3つの副問題に分割します：<em>リーダー選挙</em>（単一のリーダーを選ぶ）、<em>ログレプリケーション</em>（リーダーがフォロワーにエントリを配布する）、<em>安全性</em>（コミットされたエントリが失われないことを保証する）。</p>

<h3>ロールと状態</h3>

<p><span class="sidenote"><strong><code>consensus/src/member.rs</code></strong></span>
アンサンブルのすべてのメンバーは任意の時点で3つのロールのいずれかにあります：リーダー、フォロワー、または候補者。リーダーはすべてのクライアントリクエストを処理しログエントリをフォロワーにレプリケートします。フォロワーはリーダーからのエントリを受動的に受け入れます。候補者は新しいリーダーになろうとしているフォロワーです。</p>

<pre class="code-consensus"><code>#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Role {
    Leader,
    Follower,
    Candidate,
}

pub struct Member {
    pub address: Arc&lt;Mutex&lt;String&gt;&gt;,
    pub role: Arc&lt;Mutex&lt;Role&gt;&gt;,
    pub peers: Arc&lt;RwLock&lt;Vec&lt;String&gt;&gt;&gt;,
    pub last_heartbeat: Arc&lt;Mutex&lt;Instant&gt;&gt;,
    pub term: Arc&lt;Mutex&lt;u64&gt;&gt;,
    pub log: Arc&lt;RwLock&lt;Vec&lt;LogEntry&gt;&gt;&gt;,
    pub commit_index: Arc&lt;Mutex&lt;usize&gt;&gt;,
    pub last_applied: Arc&lt;Mutex&lt;usize&gt;&gt;,
    pub state_machine: Arc&lt;Mutex&lt;dyn StateMachine + Send + Sync&gt;&gt;,
}</code></pre>

<p><code>Member</code>構造体にはコンセンサス参加者が必要とするすべての状態が含まれています。<code>term</code>は各選挙で単調増加する論理クロックで古いリーダーの検出を可能にします。<code>log</code>はすべてのメンバーが合意すべきエントリの順序付きシーケンスです。<code>commit_index</code>はログのどこまでが過半数にレプリケートされたかを追跡し、<code>last_applied</code>は状態マシンがどこまで消費したかを追跡します。</p>

<p><span class="sidenote"><strong><code>consensus/src/lib.rs</code></strong></span>
各ログエントリは作成されたtermのレコード、アクション識別子、ペイロードを記録します。<code>StateMachine</code>トレイトはアプリケーションがコミットされたエントリをどのように処理するかを定義します：</p>

<pre class="code-consensus"><code>#[derive(Serializable, Deserializable, Clone, Debug)]
pub struct LogEntry {
    term: u64,
    action: u32,
    payload: String,
}

#[async_trait]
pub trait StateMachine {
    async fn apply(&amp;mut self, action: u32, payload: String);
    async fn handle(&amp;mut self, request: Request) -&gt; Response;
}</code></pre>

<h3>メインループ</h3>

<p>メンバーのライフサイクルは現在のロールに基づいて動作を切り替えるループです。起動時にメンバーは既存のリーダー（<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>サービスで発見）に連絡してアンサンブルに参加するかリーダーが存在しない場合は最初のリーダーとして新しいアンサンブルを初期化します：</p>

<pre class="code-consensus"><code>pub async fn run(&amp;self) {
    self.join_ensemble().await;

    loop {
        let current_role = {
            let role_lock = self.role.lock().await;
            *role_lock
        };

        match current_role {
            Role::Follower =&gt; self.run_as_follower().await,
            Role::Leader =&gt; self.run_as_leader().await,
            Role::Candidate =&gt; self.become_candidate().await,
        }
    }
}</code></pre>

<h3>リーダー選挙</h3>

<p><span class="sidenote"><strong><code>consensus/src/follower.rs</code></strong></span>
フォロワーはリーダーからのハートビートを監視します。ランダム化されたタイムアウト（1500〜3000ミリ秒）内にハートビートが届かない場合フォロワーはリーダーが障害を起こしたと判断し選挙を開始するために候補者ロールに移行します：</p>

<pre class="code-consensus"><code>pub async fn run_as_follower(&amp;self) {
    loop {
        let timeout = rand::thread_rng().gen_range(1500..3000);
        tokio::time::sleep(Duration::from_millis(timeout)).await;

        let last_heartbeat = *self.last_heartbeat.lock().await;
        if last_heartbeat.elapsed() &gt;= Duration::from_millis(timeout as u64) {
            warn!("Too long since last heartbeat from leader");
            self.become_candidate().await;
            break;
        }
    }
}</code></pre>

<p>ランダム化されたタイムアウトは重要です。すべてのフォロワーが同じタイムアウトを使用すると全員が同時に選挙を開始して投票が分散しどの候補者も勝てなくなります。ランダム化により、ほとんどの場合単一のフォロワーが最初にタイムアウトし他のフォロワーが開始する前に勝利します。</p>

<p><span class="sidenote"><strong><code>consensus/src/candidate.rs</code></strong></span>
候補者はtermをインクリメントし自分自身に投票しすべてのピアに投票を要求します。過半数から投票を受け取ればリーダーになります：</p>

<pre class="code-consensus"><code>pub async fn become_candidate(&amp;self) {
    let mut term = self.term.lock().await;
    *term += 1;
    drop(term);

    *self.role.lock().await = Role::Candidate;
    self.start_election().await;
}

async fn start_election(&amp;self) {
    let mut votes = 1; // Start with 1 vote for self
    let term = *self.term.lock().await;
    let needed_votes = (self.peers.read().await.len() / 2) + 1;

    let peers = self.peers.read().await.clone();
    for peer in peers.iter() {
        if self.request_vote(peer, term).await {
            votes += 1;
        }
    }

    if *self.role.lock().await != Role::Candidate {
        return; // Role changed during election
    }

    if votes >= needed_votes {
        *self.role.lock().await = Role::Leader;
    }
}</code></pre>

<p>過半数要件（<code>(peers.len() / 2) + 1</code>）はクォーラムベースコンセンサスの核心です。5メンバーのアンサンブルでは3票が必要です。これにより2つの同時選挙が両方とも成功することは不可能です。</p>

<h2>設計の議論</h2>

<p>Raftのようなコンセンサスアルゴリズムは強い保証を提供します：エントリがコミットされたらサーバーの少数派が障害を起こしても失われません。これらの保証にはコストが伴います。すべての書き込みは確認前に過半数のサーバーにレプリケートされる必要がありレイテンシーが追加されます。読み取り操作も古い読み取りを防ぐためにリニアライズされる必要があります。</p>

<p>タイムアウトの選択はシステム動作にとって重要です。選挙タイムアウトは通常のハートビート遅延が不必要な選挙をトリガーしないほど長く実際のリーダー障害が迅速に検出されるほど短い必要があります。私たちの実装は選挙に1500〜3000ミリ秒、ハートビートに1500ミリ秒を使用しローカルネットワークに適しています。</p>

<p><code>StateMachine</code>トレイトはコンセンサスプロトコルとアプリケーションロジックのクリーンな分離を提供します。状態の変更を<code>(action, payload)</code>ペアのシーケンスとして表現できるすべてのアプリケーションがこのコンセンサス実装上に構築できます。本番環境ではetcd、ZooKeeper、Consulのようなコンセンサスベースのシステムが分散コーディネーションのバックボーンを形成します。</p>
"##
}

pub fn chapter_configuration() -> &'static str {
    r##"
<h1>第4章: コンフィグレーション</h1>

<p><span class="newthought">すべての分散システムは</span>環境に適応しなければなりません。サーバーアドレスの変更、フィーチャーフラグの切り替え、レート制限の調整、運用パラメータの変更&mdash;&mdash;しばしばシステムの実行中に行われます。これらの値をバイナリにハードコードすると、パラメータが変更されるたびに再コンパイルと再デプロイが必要になります。<a href="/ja/chapter/configuration" class="sys" style="color:#3A86FF">コンフィグレーション</a>サービスはランタイムパラメータをそれを使用するコードから分離し、運用者がソースコードに触れたりプロセスを再起動したりすることなくシステムの動作を変更できるようにします。</p>

<p>コアにおいてコンフィグレーションサービスは一つのひねりを加えた分散キーバリューストアです：クライアントは値を読み取るだけでなく変更を<em>監視</em>します。コンフィグレーション値が変更されるとそれに依存するシステムは迅速に変更を知る必要があります。これによりコンフィグレーションは基盤サービスとなり、プラネタリスケールコンピュータのほぼすべてのサービスがそれに依存します。</p>

<h2>インターフェース</h2>

<p><span class="sidenote"><strong><code>configuration/src/lib.rs</code></strong></span>
<a href="/ja/chapter/configuration" class="sys" style="color:#3A86FF">コンフィグレーション</a>サービスはRPCインターフェースを通じて5つのプロシージャを公開します。基本操作&mdash;&mdash;get、set、delete&mdash;&mdash;は標準的なキーバリューセマンティクスを提供します。list操作はプレフィックスベースのキー列挙をサポートし、<code>storage.</code>や<code>caching.</code>のような名前空間下のすべてのコンフィグレーションの発見に有用です。watch操作によりクライアントは特定のキーの変更をポーリングできます。</p>

<pre class="code-configuration"><code>pub const GET_PROCEDURE: ProcedureId = 1;
pub const SET_PROCEDURE: ProcedureId = 2;
pub const DELETE_PROCEDURE: ProcedureId = 3;
pub const LIST_PROCEDURE: ProcedureId = 4;
pub const WATCH_PROCEDURE: ProcedureId = 5;

#[derive(Debug, Serializable, Deserializable)]
pub struct GetArgs {
    pub key: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct GetResult {
    pub value: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct SetArgs {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct DeleteArgs {
    pub key: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ListArgs {
    pub prefix: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ListResult {
    pub keys: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct WatchArgs {
    pub key: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct WatchEvent {
    pub key: String,
    pub value: String,
}</code></pre>

<p>各構造体は<a href="/ja/chapter/systems" class="sys" style="color:#E63946">正規化</a>システムから<code>Serializable</code>と<code>Deserializable</code>を派生し、<a href="/ja/chapter/systems" class="sys" style="color:#F4845F">rpc</a>を介したネットワーク転送を可能にします。<code>ListResult</code>はキーをカンマ区切り文字列として返します&mdash;&mdash;これはリストや配列の正規化を必要としない意図的なシンプルさです。</p>

<h2>実装</h2>

<p><span class="sidenote"><strong><code>configuration/src/main.rs</code></strong></span>
サーバーはインメモリ<code>HashMap</code>と変更をウォッチャーに通知するためのブロードキャストチャネルを含む<code>ConfigStore</code>を維持します。ブロードキャストチャネルはTokioの<code>broadcast::channel</code>を使用し複数のレシーバーと有界バッファリングをサポートします。</p>

<pre class="code-configuration"><code>struct ConfigStore {
    data: HashMap&lt;String, String&gt;,
    watchers: broadcast::Sender&lt;(String, String)&gt;,
}

impl ConfigStore {
    fn new() -&gt; Self {
        let (tx, _) = broadcast::channel(256);
        ConfigStore {
            data: HashMap::new(),
            watchers: tx,
        }
    }
}</code></pre>

<p>setプロシージャのハンドラはキーバリューペアをハッシュマップに挿入し、ウォッチャーチャネルを通じて変更をブロードキャストします。このキーの変更を監視しているシステムはsetが完了するとすぐに通知されます：</p>

<pre class="code-configuration"><code>pub async fn set(payload: &amp;str, store: &amp;mut ConfigStore) -&gt; Response {
    let args = SetArgs::deserialize(payload)
        .expect("Failed to deserialize payload");
    store.data.insert(args.key.clone(), args.value.clone());
    let _ = store.watchers.send((args.key, args.value));
    Response { payload: "OK".to_string() }
}</code></pre>

<p>listハンドラはプレフィックスでキーをフィルタリングします。これはコンフィグレーションの階層的な整理を可能にするパターンです。watchプロシージャはキーの現在の値を返しクライアントが変更をポーリングできるようにします。</p>

<h2>設計の議論</h2>

<p>この実装にはいくつかの設計上のトレードオフがあります。インメモリストアは高速な読み書きを提供しますが再起動に耐えられません。本番のコンフィグレーションサービスはデータを永続的なストレージに保存します&mdash;&mdash;実際には後で見る<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>サービスを使用できるでしょう。</p>

<p>ウォッチャー用のブロードキャストチャネルは実用的な選択です。通知メカニズムをストレージメカニズムから分離し有界バッファが遅いコンシューマーがライターをブロックするのを防ぎます。しかしバッファが一杯になると遅れて到着するウォッチャーはイベントを見逃します&mdash;&mdash;完全性よりも可用性を優先するトレードオフです。</p>

<p>コンフィグレーションは分散システムで最初に起動し最後に停止するサービスであることが多いです。ほぼすべてのサービスがコンフィグレーションに依存するためその可用性は重要です。本番環境ではコンフィグレーションサービスは個々のサーバーが障害を起こしてもコンフィグレーションデータが常に利用可能であることを保証するためにコンセンサスプロトコルを使用して複数のサーバーにレプリケートされます。</p>
"##
}

pub fn chapter_discovery() -> &'static str {
    r##"
<h1>第5章: ディスカバリ</h1>

<p><span class="newthought">プラネタリスケールコンピュータでは</span>、サーバーは一時的なものです。起動、停止、マシン間の移動、負荷に応じたスケールアップとスケールダウンが行われます。依存するサーバーのアドレスをハードコードするクライアントはそのサーバーが移動した瞬間に壊れます。<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>サービスはどのサーバーが利用可能でどこに見つけられるかの動的レジストリを維持することでこの問題を解決します。</p>

<p>第1章でディスカバリを簡単に紹介しました。ここでは実装を詳しく見ていきます：レジストリのデータ構造、最新の状態を保つためのメカニズム、ディスカバリをすべてのサービス通信の基盤として十分に信頼性の高いものにする設計上の決定です。</p>

<h2>インターフェース</h2>

<p><span class="sidenote"><strong><code>discovery/src/lib.rs</code></strong></span>
ディスカバリのインターフェースは意図的にミニマルです：2つのプロシージャ。registerはサーバーが自身をアナウンスしqueryはクライアントがサーバーを見つけることを可能にします。このシンプルさは意図的なもので&mdash;&mdash;複雑なディスカバリサービスは複雑な方法で障害を起こします：</p>

<pre class="code-discovery"><code>pub const REGISTER_PROCEDURE: ProcedureId = 1;
pub const QUERY_PROCEDURE: ProcedureId = 2;

#[derive(Debug, Serializable, Deserializable)]
pub struct RegisterArgs {
    pub name: String,
    pub address: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct QueryArgs {
    pub name: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct QueryResult {
    pub address: String,
}</code></pre>

<h2>レジストリ</h2>

<p><span class="sidenote"><strong><code>discovery/src/main.rs</code></strong></span>
ディスカバリサービスの心臓部は<code>Registry</code>データ構造です。2つのマップを維持します：システム名からサーバーアドレスのリストへのマップと各アドレスが最後にチェックインした時刻を追跡するマップです。このデュアルマップ設計は論理的な関心事（どのサーバーがどのシステムを実装するか）と運用上の関心事（どのサーバーがまだ生きているか）を分離します：</p>

<pre class="code-discovery"><code>#[derive(Default)]
pub struct Registry {
    registry: HashMap&lt;Name, Vec&lt;Address&gt;&gt;,
    last_ping: HashMap&lt;Address, Instant&gt;,
}

impl Registry {
    fn register(&amp;mut self, name: Name, address: Address) {
        if let Some(time) = self.last_ping.get_mut(&amp;address) {
            *time = Instant::now();
        } else {
            self.registry.entry(name)
                .or_insert_with(Vec::new)
                .push(address.clone());
            self.last_ping.insert(address, Instant::now());
        }
    }

    fn get_address(&amp;self, name: &amp;Name) -&gt; Option&lt;&amp;Address&gt; {
        self.registry.get(name)?.choose(&amp;mut rand::thread_rng())
    }
}</code></pre>

<p><code>register</code>メソッドは初回登録と再登録の両方を処理します。既知のアドレスが再登録するとタイムスタンプのみが更新され&mdash;&mdash;レジストリにアドレスが重複することはありません。この冪等性はサーバーが定期的に登録するため重要で、エントリの重複は<code>get_address</code>のランダム選択にバイアスをかけてしまいます。</p>

<p><code>get_address</code>メソッドはランダム選択を使用してシステムを実装するすべてのサーバーに負荷を分散します。3つのサーバーが<a href="/ja/chapter/systems" class="sys" style="color:#00B4D8">echo</a>システムを実装している場合各クエリはほぼ3分の1の確率で各サーバーのアドレスを返します。</p>

<h2>古いエントリのクリーンアップ</h2>

<p>サーバーは登録解除なしに障害を起こす可能性があります&mdash;&mdash;クラッシュ、ネットワーク分断、またはハードウェア障害によりレジストリに古いエントリが残ります。クリーンアップメカニズムは設定可能な期間内にハートビートを送信していないアドレスを削除します。クリーンアップはバックグラウンドタスクとして定期的にレジストリをスキャンします。</p>

<h2>エクスポネンシャルバックオフ付き登録</h2>

<p>ディスカバリのクライアント側も同様に重要です。各サーバーは古いものとしてクリーンアップされないように継続的に再登録する必要があります。ディスカバリサービスが利用不可能な場合、登録はエクスポネンシャルバックオフを使用します。これにより、ディスカバリサービスが障害後に再起動した際の再登録試行のサンダリングハードを防ぎます。</p>

<h2>設計の議論</h2>

<p>重要な問題はサービスがディスカバリサービス自体をどのように見つけるかです。私たちの実装は既知のアドレス（<code>127.0.0.1:10200</code>）を使用します。本番環境ではいくつかの代替手段があります。DNSは既知のホスト名をディスカバリサービスの現在のアドレスにマップできます。マルチキャストまたはブロードキャストプロトコルはローカルネットワーク上でディスカバリサービスをアナウンスできます。</p>

<p><code>get_address</code>のランダム選択は基本的なロードバランシングを提供しますがサーバーのヘルスや負荷を認識しません。<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>サービスはディスカバリの上に構築してヘルス対応のルーティングとコネクションプーリングを追加します。この階層化&mdash;&mdash;ディスカバリはロケーションを処理しルーティングはヘルスと効率を処理する&mdash;&mdash;により各サービスが焦点を絞りシンプルに保てます。</p>

<p>より大規模なデプロイメントではディスカバリサービス自体を可用性のためにレプリケートする必要があります。ディスカバリは本質的にレプリカ間で一貫している必要のあるキーバリューペアのレジストリであるため、前章で見た<a href="/ja/chapter/consensus" class="sys" style="color:#06D6A0">コンセンサス</a>プロトコルの自然な候補です。</p>
"##
}

pub fn chapter_routing() -> &'static str {
    r##"
<h1>第6章: ルーティング</h1>

<p><span class="newthought">サーバーがどこにあるかを知ることは</span>、通信の問題の半分にすぎません。残りの半分はリクエストをそのサーバーに効率的かつ確実に、そしてサーバーを圧倒することなく届けることです。<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>サービスはディスカバリ（サーバーの発見）と通信（データの交換）の間のギャップを埋め、コネクションプーリング、負荷分散、クライアントのためのクリーンな抽象化を追加します。</p>

<p>第1章でルーティングサービスを紹介しました。ここではその実装を詳しく見ていきます：TCPソケットを再利用するコネクションプール、並行性を制御するセマフォ、スタンドアロンプロキシまたは組み込みライブラリとして実行できるデュアルモードアーキテクチャです。</p>

<h2>インターフェース</h2>

<p><span class="sidenote"><strong><code>routing/src/lib.rs</code></strong></span>
ルーティングのインターフェースは単一のプロシージャで構成されます：route。クライアントはターゲットシステムの名前、呼び出すプロシージャ、ペイロードを指定します。ルーティングサービスはサーバーの発見、コネクション管理、リクエストの転送のすべての複雑さを処理します：</p>

<pre class="code-routing"><code>pub const ROUTE_PROCEDURE: ProcedureId = 1;

#[derive(Debug, Serializable, Deserializable)]
pub struct RouteArgs {
    pub name: String,
    pub procedure_id: i32,
    pub payload: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct RouteResult {
    pub payload: String,
}</code></pre>

<h2>コネクションプール</h2>

<p>TCP接続の確立には3ウェイハンドシェイクが必要でリクエストごとにレイテンシーが追加されます。コネクションプーリングは確立済みの接続を複数のリクエスト間で再利用することでこのオーバーヘッドを排除します。セマフォが並行性制御の鍵で、最大数の同時リクエストが飛行中の場合追加のリクエストはブロックされます。</p>

<h2>デュアルモードアーキテクチャ</h2>

<p>ルーティングシステムは2つのモードで動作でき異なるユースケースに異なるトレードオフを提供します。</p>

<p><strong>プロキシモード</strong>はルーティングをスタンドアロンサーバーとして実行します。クライアントはリクエストをルーティングサーバーに送信し、ルーティングサーバーが適切なバックエンドに転送します。</p>

<p><strong>ライブラリモード</strong>はルーティングをクライアントプロセスに直接組み込みます。<code>Router</code>構造体は独自のプールを維持し仲介なしでコネクションを管理します。プロキシを通じた追加のネットワークホップが排除されレイテンシーが削減されますが、分散型のコネクション管理になります。</p>

<h2>設計の議論</h2>

<p>プロキシモードとライブラリモードの選択はデプロイメント環境に依存します。プロキシはクライアントにとってシンプルで（プロキシのアドレスだけ知ればよい）可観測性とポリシー適用の単一ポイントを提供します。ライブラリはより高速で（追加ホップなし）より回復力があります（単一障害点なし）。多くの本番システムは両方を使用します。</p>

<p><a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>とともに<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>サービスはプラネタリスケールコンピュータの通信バックボーンを提供します。ディスカバリはサーバーがどこにあるかを知り、ルーティングはそれらと効率的に通信する方法を知ります。この関心の分離により各サービスは独立して進化しながらすべてのサービス間通信の信頼性の高い基盤を提供できます。</p>
"##
}

pub fn chapter_caching() -> &'static str {
    r##"
<h1>第7章: キャッシング</h1>

<p><span class="newthought">データソースからの</span>アクセス&mdash;&mdash;データベース、リモートサービス、永続ストレージなど&mdash;&mdash;には時間がかかります。ネットワークのラウンドトリップ、ディスク読み取り、計算のすべてがレイテンシーに寄与します。<a href="/ja/chapter/caching" class="sys" style="color:#7209B7">キャッシング</a>サービスは最近またはよくアクセスされるデータをメモリに格納し、同じデータに対する後続のリクエストをより速く処理できるようにします。キャッシングは分散システムのパフォーマンスを向上させコストを削減するための最も効果的な技術の一つです。</p>

<p>キャッシングの背後にある基本的な洞察は<em>局所性</em>の原理です：最近アクセスされたデータはすぐに再びアクセスされる可能性が高い（時間的局所性）、最近アクセスされたデータの近くのデータもアクセスされる可能性が高い（空間的局所性）。適切に設計されたキャッシュはこれらのパターンを利用して高い割合のリクエストを低速のバッキングストアではなく高速のインメモリストレージから提供します。</p>

<h2>インターフェース</h2>

<p><span class="sidenote"><strong><code>caching/src/lib.rs</code></strong></span>
<a href="/ja/chapter/caching" class="sys" style="color:#7209B7">キャッシング</a>サービスは4つのプロシージャを提供します。getとsetがコアの読み書きインターフェースを形成します。<a href="/ja/chapter/configuration" class="sys" style="color:#3A86FF">コンフィグレーション</a>サービスとは異なりset操作はエントリの有効期間を制御するTTL（Time-to-Live）パラメータを受け付けます。delete操作は明示的なキャッシュ無効化を可能にしstats操作は運用メトリックを公開します。</p>

<pre class="code-caching"><code>pub const GET_PROCEDURE: ProcedureId = 1;
pub const SET_PROCEDURE: ProcedureId = 2;
pub const DELETE_PROCEDURE: ProcedureId = 3;
pub const STATS_PROCEDURE: ProcedureId = 4;

#[derive(Debug, Serializable, Deserializable)]
pub struct GetArgs {
    pub key: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct GetResult {
    pub value: String,
    pub hit: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct SetArgs {
    pub key: String,
    pub value: String,
    pub ttl_secs: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct StatsResult {
    pub hits: i32,
    pub misses: i32,
    pub size: i32,
}</code></pre>

<h2>実装</h2>

<p><span class="sidenote"><strong><code>caching/src/main.rs</code></strong></span>
キャッシュサーバーは高速キー検索のための<code>HashMap</code>とアクセス順序追跡のための<code>VecDeque</code>を組み合わせたインメモリデータ構造を維持します。各エントリは値と有効期限を格納します。これにより時間ベースの有効期限付きLRU（Least Recently Used）エビクションポリシーが実装されます。</p>

<pre class="code-caching"><code>const MAX_CAPACITY: usize = 10000;

struct CacheEntry {
    value: String,
    expires_at: Instant,
}

struct Cache {
    entries: HashMap&lt;String, CacheEntry&gt;,
    lru_order: VecDeque&lt;String&gt;,
    hits: i32,
    misses: i32,
    max_capacity: usize,
}</code></pre>

<p>get操作は値を返す前に有効期限を確認します。エントリが期限切れの場合は削除されミスとして扱われます。ヒットの場合キーはLRUキューの先頭に移動され最近アクセスされたことが記録されます。バックグラウンドタスクが定期的に期限切れエントリをクリーンアップしキャッシュが再アクセスされないかもしれない古いデータで一杯になるのを防ぎます。</p>

<h2>設計の議論</h2>

<p>LRUエビクションポリシーはキャッシュ容量管理のいくつかの戦略の一つです。LRUはアクセスパターンが時間的局所性を示す場合にうまく機能します。他の戦略にはLFU（Least Frequently Used）やランダムエビクションなどがあります。</p>

<p>TTLベースの有効期限は古いデータの提供に対する安全策を提供します。TTLなしではキャッシュされた値はバッキングストアが更新された後も無期限に存続する可能性があります。TTLの選択はトレードオフを表します：短いTTLは古さを減らしますがバッキングストアへの負荷を増加させ、長いTTLはヒット率を改善しますが古いデータを提供するリスクがあります。</p>

<p>statsプロシージャを通じて公開されるヒット/ミスの追跡は運用において非常に有用です。健全なキャッシュは高いヒット率（90%以上のことが多い）を持つべきです。ヒット率の急激な低下はアクセスパターンの変化、設定エラー、または容量問題を示している可能性があります。これらのメトリックは<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスが追跡しアラートできるようなシグナルです。</p>
"##
}

pub fn chapter_storage() -> &'static str {
    r##"
<h1>第8章: ストレージ</h1>

<p><span class="newthought">実行中のシステムのすべてのデータは</span>メモリに存在します&mdash;&mdash;高速で揮発性で有限です。プロセスがクラッシュしたりサーバーが電源を失ったりするとそのデータは消滅します。<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>サービスは<em>永続的な</em>データ保存を提供します：サービスに書き込まれたデータがプロセスの再起動、ハードウェア障害、電源障害にも耐えるという保証です。ストレージはステートフルなシステムが構築される基盤です。</p>

<p>ストレージエンジンの設計は競合する要求のバランスを取ることを含みます。読み取りは高速であるべきです（理想的にはメモリから提供）。書き込みは耐久性があるべきです（確認前に安全にディスクに保存）。スペースは効率的に使用されるべきです（古いデータはコンパクション）。そしてシステム全体がクラッシュ後に迅速に回復すべきです。私たちの実装はこれらの各懸念に古典的な技術で対処します：ライトアヘッドログです。</p>

<h2>インターフェース</h2>

<p><span class="sidenote"><strong><code>storage/src/lib.rs</code></strong></span>
<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>サービスは4つのプロシージャを提供します。getとputがデータの読み書きの基本的なキーバリューインターフェースを形成します。deleteはキーを削除しscanはプレフィックスに一致する複数のエントリを取得します。</p>

<pre class="code-storage"><code>pub const GET_PROCEDURE: ProcedureId = 1;
pub const PUT_PROCEDURE: ProcedureId = 2;
pub const DELETE_PROCEDURE: ProcedureId = 3;
pub const SCAN_PROCEDURE: ProcedureId = 4;

#[derive(Debug, Serializable, Deserializable)]
pub struct GetArgs {
    pub key: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct GetResult {
    pub value: String,
    pub found: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct PutArgs {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct DeleteArgs {
    pub key: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ScanArgs {
    pub prefix: String,
    pub limit: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ScanResult {
    pub entries: String,
}</code></pre>

<h2>実装</h2>

<p><span class="sidenote"><strong><code>storage/src/engine.rs</code></strong></span>
<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>サービスの心臓部はその<code>StorageEngine</code>です。エンジンは高速読み取りのためのインメモリ<code>HashMap</code>、耐久性のためのライトアヘッドログ（WAL）、効率的なリカバリのためのスナップショットメカニズムを維持します。この組み合わせはSQLiteからPostgreSQLまでのデータベースに見られる古典的なパターンです。</p>

<pre class="code-storage"><code>pub struct StorageEngine {
    data: HashMap&lt;String, String&gt;,
    wal_path: PathBuf,
    snapshot_path: PathBuf,
    operations_since_snapshot: usize,
}

const COMPACTION_THRESHOLD: usize = 1000;</code></pre>

<p>ライトアヘッドログが耐久性の鍵です。putでもdeleteでもすべての書き込み操作はインメモリデータ構造が更新される前にまずディスク上のWALファイルに追記されます。これにより書き込みを確認した直後にプロセスがクラッシュしてもログから操作を回復できることが保証されます。</p>

<p>リカバリは2段階でインメモリ状態を再構築します：まず最新のスナップショット（存在する場合）をロードし、次にスナップショット後に書き込まれたWALエントリをリプレイします。時間の経過とともにWALは操作の蓄積により成長します。コンパクションは現在のインメモリ状態のスナップショットを書き込み、WALを切り詰めることでこれを解決します。</p>

<h2>設計の議論</h2>

<p>ライトアヘッドログパターンは強い耐久性保証を提供します：データは書き込みが確認される前にディスクに書き込まれます。しかし私たちの実装は単一のディスク上の単一のファイルに書き込みます。本番のストレージサービスはディスクやマシンの障害に耐えるためにデータを複数のサーバーにレプリケートします。</p>

<p>インメモリの<code>HashMap</code>はO(1)の読み取りを提供しますが総データサイズを利用可能なメモリに制限します。より大きなデータセットに対してはストレージエンジンはLSMツリーやBツリーなどのディスク上のデータ構造を使用します。</p>

<p>scan操作のプレフィックスベースのフィルタリングは多用途のプリミティブです。名前空間化、範囲クエリ、列挙などのパターンを可能にします。ハイライトシステムはこのパターンを使用してユーザーごとページごとのハイライトを<code>hl:{user_id}:{page_slug}</code>のようなキーの下に格納します。</p>
"##
}

pub fn chapter_monitoring() -> &'static str {
    r##"
<h1>第14章: モニタリング</h1>

<p><span class="newthought">観測できない分散システムは</span>、運用できない分散システムです。何かがうまくいかないとき&mdash;&mdash;プラネタリスケールコンピュータでは、どこかで<em>常に</em>何かがうまくいっていません&mdash;&mdash;運用者は何が起きているのか、どこで起きているのか、そしてできれば<em>なぜ</em>起きているのかを知る必要があります。<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスは、システム内のすべてのサービスからヘルスとパフォーマンスのデータを収集、保存、公開します。</p>

<p>モニタリングは2つの対象に奉仕します。<em>人間</em>に対しては、システムの振る舞いを理解しインシデントに対応するためのダッシュボード、アラート、診断データを提供します。<em>マシン</em>に対しては、ロードシェディング、フェイルオーバー、オートスケーリングなどの自動化アクションを可能にするヘルスシグナルを提供します。例えば<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>サービスは、モニタリングからのヘルスシグナルを使用して、不健全なサーバーへのトラフィック送信を回避できます。</p>

<h2>インターフェース</h2>

<p><span class="sidenote"><strong><code>monitoring/src/lib.rs</code></strong></span>
<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスは4つのプロシージャを公開します。reportプロシージャはサービスからメトリクスデータポイントを受け取ります。heartbeatプロシージャはヘルスステータスの更新を受け取ります。queryプロシージャはメトリクスの時系列を取得し、healthプロシージャはすべての既知のサービスのヘルスステータスを返します。</p>

<pre class="code-monitoring"><code>pub const REPORT_PROCEDURE: ProcedureId = 1;
pub const HEARTBEAT_PROCEDURE: ProcedureId = 2;
pub const QUERY_PROCEDURE: ProcedureId = 3;
pub const HEALTH_PROCEDURE: ProcedureId = 4;

#[derive(Debug, Serializable, Deserializable)]
pub struct ReportArgs {
    pub service: String,
    pub metric: String,
    pub value: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct HeartbeatArgs {
    pub service: String,
    pub status: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct QueryArgs {
    pub service: String,
    pub metric: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct QueryResult {
    pub values: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct HealthArgs {
    pub placeholder: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct HealthResult {
    pub services: String,
}</code></pre>

<p><code>ReportArgs</code>構造体は、何を計測しているかを識別するために汎用的な<code>service</code>と<code>metric</code>のペアを使用し、計測値として整数<code>value</code>を持ちます。このシンプルなスキーマは、リクエスト数、レイテンシー、キュー深度、キャッシュヒット率など、幅広い種類のメトリクスを表現できます。</p>

<h2>実装</h2>

<p><span class="sidenote"><strong><code>monitoring/src/main.rs</code></strong></span>
モニタリングサーバーは2つのデータ構造を維持します：各サービスのステータスと最終ハートビート時刻を追跡するヘルスレジストリと、報告された値のローリングウィンドウを保持するメトリクスストアです。</p>

<pre class="code-monitoring"><code>const HEARTBEAT_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_METRIC_WINDOW: usize = 100;

struct ServiceHealth {
    status: String,
    last_heartbeat: Instant,
}

struct MonitoringState {
    health: HashMap&lt;String, ServiceHealth&gt;,
    metrics: HashMap&lt;String, Vec&lt;i32&gt;&gt;,
}</code></pre>

<p>ハートビートハンドラはサービスのヘルスステータスとタイムスタンプを更新します。サービスは定期的にハートビートを送信することが期待されています。サービスがハートビートウィンドウを逃した場合、モニタリングシステムはそれを不健全としてマークします：</p>

<pre class="code-monitoring"><code>pub async fn heartbeat(
    payload: &amp;str,
    state: &amp;mut MonitoringState,
) -&gt; Response {
    let args = HeartbeatArgs::deserialize(payload)
        .expect("Failed to deserialize payload");
    let health = state.health.entry(args.service.clone())
        .or_insert(ServiceHealth {
            status: args.status.clone(),
            last_heartbeat: Instant::now(),
        });
    health.status = args.status;
    health.last_heartbeat = Instant::now();
    Response { payload: "OK".to_string() }
}</code></pre>

<p>レポートハンドラはメトリクス値をローリングウィンドウに保存します。各メトリクスキーはサービス名とメトリクス名の組み合わせ（例：<code>storage:latency</code>）で形成されます。ウィンドウは最新の100件の値を保持し、メモリ使用量を制限しつつ、平均やパーセンタイルなどの統計を計算するのに十分なデータを提供します：</p>

<pre class="code-monitoring"><code>pub async fn report(
    payload: &amp;str,
    state: &amp;mut MonitoringState,
) -&gt; Response {
    let args = ReportArgs::deserialize(payload)
        .expect("Failed to deserialize payload");
    let key = format!("{}:{}", args.service, args.metric);
    let values = state.metrics.entry(key).or_insert_with(Vec::new);
    values.push(args.value);
    if values.len() &gt; MAX_METRIC_WINDOW {
        values.remove(0);
    }
    Response { payload: "OK".to_string() }
}</code></pre>

<p>バックグラウンドタスクが定期的にタイムアウト期間内にハートビートを送信していない古いサービスをチェックし、不健全としてマークします。これがモニタリングシステムの障害検出における主要なメカニズムです：</p>

<pre class="code-monitoring"><code>fn check_stale_services(&amp;mut self) {
    let now = Instant::now();
    for (service, health) in self.health.iter_mut() {
        if now.duration_since(health.last_heartbeat) &gt; HEARTBEAT_TIMEOUT {
            if health.status != "unhealthy" {
                println!("Service {} marked unhealthy (heartbeat timeout)",
                    service);
                health.status = "unhealthy".to_string();
            }
        }
    }
}</code></pre>

<h2>設計の議論</h2>

<p>ハートビートパターンは、サービス障害を検出するためのシンプルで効果的な方法です。各サービスは定期的にモニタリングシステムに「生きています」というメッセージを送信します。モニタリングシステムがタイムアウト期間内にサービスからの連絡を受けなかった場合、サービスが障害を起こしたと判断します。タイムアウトは慎重に調整する必要があります：短すぎると一時的なネットワーク遅延により健全なサービスが不健全とマークされる可能性があり、長すぎると実際の障害の検出に時間がかかりすぎます。</p>

<p>ローリングメトリクスウィンドウは、メモリ効率とデータ保持のバランスです。100件の固定ウィンドウは、メモリ使用量を制限しつつ基本的な統計に十分なデータを提供します。Prometheusのような本番モニタリングシステムは、設定可能な保持期間と古いデータのダウンサンプリングを備えた、より洗練されたストレージを使用します。</p>

<p>重要なアーキテクチャ原則は、モニタリングは<em>プル</em>型か<em>プッシュ</em>型のいずれかであるべきで、両方ではないということです。私たちの実装はプッシュモデルを使用しています：サービスがメトリクスとハートビートをモニタリングシステムに送信します。代替手段であるプルモデル（Prometheusが使用）は、モニタリングシステムが各サービスから能動的にメトリクスをスクレイプします。プッシュはサービスにとってよりシンプルですが、サービスが完全に消失した場合の検出が困難です。プルは障害検出を自動化しますが、モニタリングシステムがすべてのサービスを事前に知っている必要があります。</p>

<p>healthプロシージャはすべてのサービスのステータスを単一のレスポンスで返すため、他のシステム（フロントエンドダッシュボードなど）がシステムヘルスの包括的なビューを表示しやすくなります。この集約はモニタリングシステムの一般的なパターンであり、ステータスページや運用ダッシュボードの基盤を形成します。</p>
"##
}

pub fn chapter_implementation() -> &'static str {
    r##"
<h1>第9章: 実装</h1>

<p><span class="newthought">設計ドキュメント</span>は、システムが何をすべきかを記述します。実装とは、その記述が動作するソフトウェアになるプロセスです。設計とコードの間のギャップは、微妙なバグが生まれ、パフォーマンスが勝敗を分け、システムの真の性格が現れる場所です。この章では、私たちが構築したすべてのシステムに共通するパターンと、その実装を導く原則を検討します。</p>

<h2>実装パターン</h2>

<p>プラネタリスケールコンピュータのすべてのサービスは同じ構造パターンに従います：インターフェースを定義する共有ライブラリ、ロジックを実装するサーバーバイナリ、そして並行リクエスト間で安全に管理される共有状態です。</p>

<p>共有ライブラリ（<code>lib.rs</code>）は、プロシージャ識別子、リクエストおよびレスポンスの構造体、そしてクライアント側のヘルパー関数を定義します。このファイルが契約です。サーバー（プロシージャを実装するため）とクライアント（それらを呼び出すため）の両方にインポートされます。ライブラリは共有されるため、変更は慎重に行う必要があります&mdash;&mdash;プロシージャ識別子の変更なしにリクエスト構造体を変更すると、古いクライアントと新しいサーバー間の互換性が壊れます。</p>

<p>サーバーバイナリ（<code>main.rs</code>）は、プロシージャ識別子に基づいて受信リクエストを適切なハンドラにディスパッチするリクエストハンドラを実装します。各ハンドラはリクエストペイロードをデシリアライズし、操作を実行し、レスポンスをシリアライズします。サーバーは共有状態の初期化、ディスカバリへの登録、バックグラウンドタスクの開始も行います。</p>

<p>共有状態は<code>Arc&lt;Mutex&lt;T&gt;&gt;</code>（またはリード負荷の高いワークロードの場合は<code>Arc&lt;RwLock&lt;T&gt;&gt;</code>）でラップされ、複数のリクエストハンドラスレッドからの安全な並行アクセスを可能にします。これは非同期タスク間での共有可変状態のための標準的なRustパターンです。</p>

<h2>バックグラウンドタスク</h2>

<p>ほとんどのサービスは、リクエスト・レスポンスサイクルの外で行う作業が必要です。<a href="/ja/chapter/caching" class="sys" style="color:#7209B7">キャッシング</a>サービスは期限切れエントリのクリーンアップのためにバックグラウンドタスクを実行します。<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスは古いハートビートをチェックします。<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>サービスは古い登録をクリーンアップします。<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>サービスはコンパクションをトリガーします。</p>

<p>これらのバックグラウンドタスクは共通のパターンに従います：スリープ間隔でループする非同期タスクをスポーンし、共有状態のロックを取得し、メンテナンスを実行し、ロックを解放します。重要な制約は、バックグラウンドタスクがロックを長時間保持してはならないことです。さもなければリクエストハンドラをブロックしてしまいます。</p>

<h2>エラー処理</h2>

<p>私たちの実装はエラー処理に対してプラグマティックなアプローチを取ります。内部エラー（整形式の内部トラフィックでのデシリアライズ失敗など）は<code>expect</code>を使用します&mdash;&mdash;これらはランタイム条件ではなくバグを示します。外部エラー（他のサービスへの接続時のネットワーク障害など）は、通常クライアントへのエラーレスポンスの返却やバックオフ付きリトライによって、グレースフルに処理されます。</p>

<p>この区別は運用において重要です。<code>expect</code>からのパニックは何かが根本的に間違っていることを意味し、サービスを再起動すべきです。グレースフルなエラーは、サービスが正しく機能しているが環境で一時的な問題に遭遇したことを意味します。</p>

<h2>テスト</h2>

<p>インターフェースファーストの設計パターンは、自然にテストをサポートします。各サービスのインターフェースが型付き構造体として定義されているため、ユニットテストはサーバーを起動したりネットワーク呼び出しを行ったりすることなく、リクエストペイロードを構築し、ハンドラを直接呼び出し、レスポンスペイロードを検証できます。統合テストはフルサーバーを起動し、RPC呼び出しを行ってエンドツーエンドの振る舞いを検証します。</p>

<p>分散システムにとって最も価値のあるテストは、ユニットテストや統合テストではなく<em>障害注入</em>テストです：ディスカバリサービスが利用不可の場合はどうなるか？ストレージの書き込みが失敗した場合はどうなるか？コンセンサスメンバーがレプリケーション中にクラッシュした場合はどうなるか？これらのテストはシステムのレジリエンスを検証します。プラネタリスケールにおいて最終的に重要なのはレジリエンスです。</p>
"##
}

pub fn chapter_operation() -> &'static str {
    r##"
<h1>第10章: 運用</h1>

<p><span class="newthought">システムを構築すること</span>は仕事の半分に過ぎません。残りの半分は、それを動作させ続けることです。運用とは、本番環境でシステムをデプロイ、モニタリング、メンテナンス、進化させる実践です。運用が困難なシステムは、どんなに設計と実装が優れていても、最終的には障害を起こします。</p>

<h2>サービスの起動</h2>

<p>プラネタリスケールコンピュータは、特定の順序で起動する必要がある複数のサービスで構成されています。<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>サービスは、他のサービスが登録できるようになる前に利用可能でなければなりません。<a href="/ja/chapter/configuration" class="sys" style="color:#3A86FF">コンフィギュレーション</a>サービスは次に起動すべきです。他のサービスが設定を読み取る可能性があるからです。その後、残りのサービス&mdash;&mdash;<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>、<a href="/ja/chapter/caching" class="sys" style="color:#7209B7">キャッシング</a>、<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>、<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>、およびアプリケーションサービス&mdash;&mdash;は任意の順序で起動できます。成功するまでディスカバリへの登録をリトライするからです。</p>

<p><code>start.sh</code>スクリプトがこの順序をエンコードしています。各サービスをバックグラウンドプロセスとして起動し、起動を少し待ってから次に進みます。これは開発には十分ですが、本番環境には不十分です。本番ではサービスは再起動、リソース制限、依存関係の順序付けを処理するプロセススーパーバイザによって管理されるべきです。</p>

<h2>ヘルスチェック</h2>

<p>サービスが稼働し始めたら、運用者はそれらが<em>健全</em>であるかどうかを知る必要があります。実行中のプロセスは必ずしも健全なプロセスではありません&mdash;&mdash;デッドロックに陥っているかもしれず、トラフィックに圧倒されているかもしれず、依存先に到達できないかもしれません。ヘルスチェックは、サービスが現在の状態を報告するための標準的な方法を提供します。</p>

<p>私たちのサービスは、ヘルスレポーティングに<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスのハートビートメカニズムを使用します。各サービスは定期的にステータス付きのハートビートを送信します。モニタリングサービスはハートビートウィンドウを逃したサービスを不健全としてマークします。この情報はダッシュボードに反映されて運用者に問題を通知し、<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>サービスに反映されて不健全なサーバーへのトラフィック送信を回避します。</p>

<h2>オブザーバビリティ</h2>

<p>ヘルスチェックは何かが間違っている<em>かどうか</em>を教えてくれます。オブザーバビリティは<em>何が</em>、<em>なぜ</em>かを教えてくれます。オブザーバビリティの3つの柱は、メトリクス（時系列の数値測定）、ログ（離散的なイベント）、トレース（複数のサービスを通じたリクエストの経路）です。</p>

<p><a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスがメトリクスを処理します。各サービスは任意のメトリクス値（リクエスト数、レイテンシー、エラー率）を報告でき、ローリングウィンドウに保存されます。ログについては、各サービスが標準出力に書き込み、ログ管理システムによって収集・集約できます。分散トレーシング&mdash;&mdash;ディスカバリ、ルーティング、バックエンドサービスを横断する単一のリクエストの追跡&mdash;&mdash;は、私たちのシステムを拡張できる領域です。</p>

<h2>運用ランブック</h2>

<p>午前3時に何かがうまくいかないとき、運用者は第一原理から推論する必要があってはなりません。運用ランブックは、一般的な障害シナリオとその修復手順を文書化します。私たちのシステムの主要なランブックには次のものが含まれます：ディスカバリサービスがダウンした場合（再起動する；他のサービスは自動的に再登録する）、ストレージが満杯の場合（コンパクションをトリガーするか容量を拡張する）、コンセンサスアンサンブルがクォーラムを失った場合（障害を起こしたメンバーを特定して再起動する）。</p>

<p>最良のランブックは、システムを構築した人々によって書かれ、運用する人々によって更新され、まだ機能することを確認するために定期的にテストされます。時間の経過とともに、最も一般的なランブックのステップは自動化され、人間の運用者の負担を軽減すべきです。</p>
"##
}

pub fn chapter_scheduling() -> &'static str {
    r##"
<h1>第11章: スケジューリング</h1>

<p><span class="newthought">プラネタリスケールコンピュータ</span>は、数千から数百万台のマシンで構成され、各マシンは多くのプロセスを実行できます。スケジューリングとは、これらのマシン上で作業を<em>いつ</em>、<em>どこで</em>実行するかを決定する技術と科学です。<a href="/ja/dashboard/scheduling" class="sys" style="color:#FF6B35">スケジューリング</a>サービスは、フリート全体を統括します&mdash;&mdash;サービスプロセスのスポーン、ポートの割り当て、ヘルスのモニタリング、望ましい状態と実際の状態の調整を行います。</p>

<h2>スケジューラのデータモデル</h2>

<p><span class="sidenote"><strong><code>scheduling/src/main.rs</code></strong></span>
スケジューラは2つのコアデータ構造を維持します。<em>ServiceSpec</em>は望ましい状態を記述します：サービス名、Cargoマニフェストパス、オプションのバイナリ名、望ましいレプリカ数。<em>Instance</em>は現実を記述します：ID、ポート、OSプロセスID、ヘルスステータスを持つ実行中のプロセスです。</p>

<pre class="code-scheduling"><code>struct ServiceSpec {
    name: String,
    manifest_path: String,
    bin_name: String,
    desired_replicas: i32,
}

struct Instance {
    id: String,
    service_name: String,
    port: u16,
    pid: u32,
    status: String, // "starting", "healthy", "unhealthy", "stopped"
}</code></pre>

<p>望ましい状態と実際の状態のギャップが、すべてのスケジューリング決定を駆動します。サービススペックが3レプリカを指定しているのに2インスタンスしか実行されていない場合、スケジューラはもう1つスポーンします。インスタンスのヘルスチェックが失敗した場合、スケジューラはそれを不健全としてマークし、置き換える可能性があります。</p>

<h2>プロセスのスポーン</h2>

<p><span class="sidenote"><strong><code>echo/src/bin/server_v1.rs</code></strong> &mdash; このパターンはすべてのサービスに現れます</span>
スケジューラは各サービスを<code>std::process::Command</code>経由でOSプロセスとしてスポーンし、<code>PORT</code>環境変数を通じて割り当てられたポートを渡します。システム内のすべてのサービスは起動時にこの変数をチェックします：</p>

<pre class="code-scheduling"><code>let addr = std::env::var("PORT")
    .map(|p| format!("127.0.0.1:{}", p))
    .unwrap_or_else(|_| SYSTEM_ADDRESS.to_string());</code></pre>

<p>この3行のパターンはすべてのサービスの<code>main.rs</code>に現れます。サービスはスタンドアロンでよく知られたポートで実行する（開発用）ことも、動的に割り当てられたポートを受け入れる（スケジューラによる管理時）こともできます。スポーンされたプロセスは<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>に自己登録し、他のサービスから即座に発見可能になります。</p>

<h2>フリートブートストラップ</h2>

<p><span class="sidenote"><strong><code>scheduling/src/main.rs</code></strong></span>
起動時に、スケジューラはハードコードされた構成テーブルからフリート全体をブートストラップします。各エントリはサービス名、Cargoマニフェストパス、オプションのバイナリ名、レプリカ数、ベースポートを指定します。単一レプリカのサービスはよく知られたポートを取得します。マルチレプリカのサービスはベースからの連番ポートを取得します。</p>

<pre class="code-scheduling"><code>const DEFAULT_FLEET: &amp;[FleetEntry] = &amp;[
    FleetEntry { name: "security",      manifest_path: "security/Cargo.toml",
                 bin_name: "", replicas: 1, base_port: 11100 },
    FleetEntry { name: "configuration", manifest_path: "configuration/Cargo.toml",
                 bin_name: "", replicas: 1, base_port: 10500 },
    FleetEntry { name: "echo",          manifest_path: "echo/Cargo.toml",
                 bin_name: "server_v1", replicas: 3, base_port: 10100 },
    FleetEntry { name: "frontend",      manifest_path: "frontend/Cargo.toml",
                 bin_name: "", replicas: 2, base_port: 8081 },
    // ... storage, caching, routing, monitoring, release
];</code></pre>

<p><span class="sidenote"><strong><code>scheduling/src/main.rs</code></strong> &mdash; <code>spawn_instance</code></span>
各エントリに対して、スケジューラは<code>spawn_instance</code>を呼び出します。これはマニフェストパスで<code>cargo run</code>コマンドを構築し、<code>PORT</code>環境変数を通じて割り当てられたポートを渡します。子プロセスIDは、スケジューラが後でインスタンスを必要に応じてキルできるようにキャプチャされます。</p>

<pre class="code-scheduling"><code>fn spawn_instance(&amp;mut self, spec: &amp;ServiceSpec, port: u16)
    -&gt; Option&lt;Instance&gt;
{
    let mut cmd = Command::new("cargo");
    cmd.arg("run")
       .arg("--manifest-path").arg(&amp;manifest);
    if !spec.bin_name.is_empty() {
        cmd.arg("--bin").arg(&amp;spec.bin_name);
    }
    cmd.env("PORT", port.to_string());
    match cmd.spawn() {
        Ok(child) =&gt; Some(Instance {
            id, service_name: spec.name.clone(),
            port, pid: child.id(),
            status: "starting".to_string(),
        }),
        Err(e) =&gt; None,
    }
}</code></pre>

<h2>ヘルスモニタリング</h2>

<p><span class="sidenote"><strong><code>scheduling/src/main.rs</code></strong> &mdash; <code>health_check_loop</code></span>
バックグラウンドループが5秒ごとに各インスタンスにTCP接続を試みてプローブします。接続が成功すればインスタンスは健全とマークされます。失敗すれば不健全とマークされます。これは最もシンプルなヘルスチェックです&mdash;&mdash;本番のスケジューラはアプリケーションレベルのヘルスエンドポイント、リソース使用率、レスポンスレイテンシーもチェックするでしょう。</p>

<pre class="code-scheduling"><code>async fn health_check_loop(shared_state: Arc&lt;Mutex&lt;SchedulerState&gt;&gt;) {
    loop {
        sleep(Duration::from_secs(5)).await;
        let mut state = shared_state.lock().await;
        for instance in state.instances.iter_mut() {
            if instance.status == "stopped" { continue; }
            let addr = format!("127.0.0.1:{}", instance.port);
            match tokio::net::TcpStream::connect(&amp;addr).await {
                Ok(_)  =&gt; { instance.status = "healthy".to_string(); }
                Err(_) =&gt; { instance.status = "unhealthy".to_string(); }
            }
        }
    }
}</code></pre>

<h2>RPCインターフェース</h2>

<p><span class="sidenote"><strong><code>scheduling/src/lib.rs</code></strong></span>
スケジューリングサービスは<a href="/ja/chapter/routing" class="sys" style="color:#F4845F">RPC</a>フレームワークを通じて5つのプロシージャを公開します。<code>SCHEDULE_SERVICE</code>は新しいサービススペックを登録して調整します。<code>LIST_INSTANCES</code>はすべての実行中インスタンスを返します。<code>SCALE_SERVICE</code>はレプリカ数を更新します。<code>STOP_INSTANCE</code>はプロセスIDで特定のインスタンスをキルします。<code>GET_SERVICE</code>は1つのサービスのスペックとインスタンスを返します。</p>

<pre class="code-scheduling"><code>pub const SCHEDULE_SERVICE_PROCEDURE: ProcedureId = 401;
pub const LIST_INSTANCES_PROCEDURE: ProcedureId = 402;
pub const SCALE_SERVICE_PROCEDURE: ProcedureId = 403;
pub const STOP_INSTANCE_PROCEDURE: ProcedureId = 404;
pub const GET_SERVICE_PROCEDURE: ProcedureId = 405;

#[derive(Debug, Serializable, Deserializable)]
pub struct ScheduleServiceArgs {
    pub name: String,
    pub manifest_path: String,
    pub bin_name: String,
    pub replicas: i32,
}</code></pre>

<p><a href="/ja/dashboard/scheduling">スケジューリングダッシュボード</a>は、これらのプロシージャを使用してフリートを表示し、運用者がサービスをスケールしたり個々のインスタンスを停止したりできるようにします。</p>
"##
}

pub fn chapter_release() -> &'static str {
    r##"
<h1>第12章: リリース</h1>

<p><span class="newthought">変更されないソフトウェア</span>は、改善されないソフトウェアです。<a href="/ja/dashboard/release" class="sys" style="color:#4CC9F0">リリース</a>サービスは、フリート全体のローリングデプロイメントを管理し、<a href="/ja/chapter/scheduling" class="sys" style="color:#FF6B35">スケジューラ</a>と連携してダウンタイムなしでインスタンスを1バッチずつ置き換えます。</p>

<h2>インターフェース</h2>

<p><span class="sidenote"><strong><code>release/src/lib.rs</code></strong></span>
リリースサービスは5つのプロシージャを公開します。<code>CREATE_RELEASE</code>は新しいデプロイメントを開始します。<code>GET_RELEASE</code>と<code>LIST_RELEASES</code>は状態を検査します。<code>ADVANCE_RELEASE</code>はデプロイメントを1バッチ前進させます。<code>ROLLBACK</code>は進行中のデプロイメントを元に戻します。</p>

<pre class="code-release"><code>pub const CREATE_RELEASE_PROCEDURE: ProcedureId = 501;
pub const GET_RELEASE_PROCEDURE: ProcedureId = 502;
pub const LIST_RELEASES_PROCEDURE: ProcedureId = 503;
pub const ADVANCE_RELEASE_PROCEDURE: ProcedureId = 504;
pub const ROLLBACK_PROCEDURE: ProcedureId = 505;

#[derive(Debug, Serializable, Deserializable)]
pub struct CreateReleaseArgs {
    pub service: String,
    pub version: String,
    pub description: String,
}</code></pre>

<h2>リリースライフサイクル</h2>

<p><span class="sidenote"><strong><code>release/src/main.rs</code></strong></span>
リリースはシンプルな状態マシンを通じて進行します：<code>created</code> &rarr; <code>deploying</code> &rarr; <code>deployed</code>（または<code>rolled_back</code>）。各状態遷移は明示的です&mdash;&mdash;運用者が<a href="/ja/dashboard/release">リリースダッシュボード</a>を通じてリリースを前進させることで、ロールアウトのペースを制御できます。</p>

<pre class="code-release"><code>struct Release {
    id: String,
    service: String,
    version: String,
    description: String,
    status: String,         // "created", "deploying", "deployed", "rolled_back"
    old_instances: Vec&lt;String&gt;,
    new_instances: Vec&lt;String&gt;,
    batch_progress: i32,
}</code></pre>

<h2>ローリングアップデート</h2>

<p><span class="sidenote"><strong><code>release/src/main.rs</code></strong> &mdash; <code>create_release</code></span>
リリースが作成されると、サービスは<a href="/ja/chapter/scheduling" class="sys" style="color:#FF6B35">スケジューラ</a>にターゲットサービスの現在のインスタンスを問い合わせ、それらを「旧」インスタンスとしてスナップショットします。バッチサイズは<code>max(1, total / 10)</code>として計算されます&mdash;&mdash;バッチあたりフリートの約10%、最低1インスタンスです。</p>

<pre class="code-release"><code>let svc_result = scheduling::get_service(
    SCHEDULER_ADDR, args.service.clone()
).await;
let old_instances: Vec&lt;String&gt; = svc_result.instances
    .split(';').map(|s| s.to_string()).collect();
let total = old_instances.len() as i32;
let batch_size = std::cmp::max(1, total / 10);</code></pre>

<p><span class="sidenote"><strong><code>release/src/main.rs</code></strong> &mdash; <code>advance_release</code></span>
<code>ADVANCE_RELEASE</code>への各呼び出しは1バッチを置き換えます。ハンドラはスケジューラにスケールアップを指示し、ヘルスを待ち、次に1つの旧インスタンスを停止します：</p>

<ol>
<li>スケジューラにバッチ用の新しいインスタンスをスポーンさせる</li>
<li>新しいインスタンスがヘルスチェックに合格するまで待つ</li>
<li>スケジューラに対応する旧インスタンスを停止させる</li>
<li>旧インスタンスは古さベースのクリーンアップにより<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>から登録解除される</li>
</ol>

<pre class="code-release"><code>// Scale up: add one replica
scheduling::scale_service(
    SCHEDULER_ADDR, release.service.clone(), current_total + 1,
).await;

// Wait for new instance to become healthy
sleep(Duration::from_secs(2)).await;

// Scale down: stop one old instance
let old_id = release.old_instances.remove(0);
scheduling::stop_instance(SCHEDULER_ADDR, old_id).await;

release.batch_progress += 1;
if release.old_instances.is_empty() {
    release.status = "deployed".to_string();
}</code></pre>

<p>このプロセスにより、ロールアウト中のどの時点でも、サービスにはトラフィックを処理するのに十分な健全なインスタンスがあることが保証されます。<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>層とディスカバリサービスは、停止されたインスタンスからのトラフィックを自動的に新しいインスタンスに振り向けます。</p>

<h2>ロールバック</h2>

<p><span class="sidenote"><strong><code>release/src/main.rs</code></strong> &mdash; <code>rollback</code></span>
デプロイメントに問題が発生した場合、<code>ROLLBACK</code>プロシージャはアクティブなリリースを<code>rolled_back</code>としてマークします。旧インスタンスは新しいインスタンスの健全性が確認された後にのみ停止されるため、デプロイメント中のロールバックは単にプロセスを停止するだけです&mdash;&mdash;残りの旧インスタンスはトラフィックの提供を続けます。高速ロールバックの鍵は、新しいものが証明される前に古いものを<em>決して</em>削除しないことです。</p>

<h2>スケジューラとの統合</h2>

<p><span class="sidenote"><strong><code>release/src/lib.rs</code></strong></span>
リリースサービスはプロセスを直接スポーンしません。代わりに、すべてのプロセス管理をRPC呼び出しを通じてスケジューラに委譲します：レプリカの追加に<code>scheduling::scale_service()</code>、削除に<code>scheduling::stop_instance()</code>を使用します。この関心の分離により、スケジューラは何が実行されているかの唯一の真実の源として維持され、リリースサービスは変更の<em>順序</em>と<em>ペース</em>を管理します。<a href="/ja/dashboard/release">リリースダッシュボード</a>でリリースを作成し、ローリングデプロイメントを段階的に進めることができます。</p>
"##
}

pub fn chapter_security() -> &'static str {
    r##"
<h1>第13章: セキュリティ</h1>

<p><span class="newthought">プラネタリスケールコンピュータ</span>は広大な攻撃対象面です。<a href="/ja/dashboard/security" class="sys" style="color:#D62828">セキュリティ</a>サービスは、システムのダッシュボードにトークンベースの認証を提供し、実際の分散システムを保護する認証、認可、トークン管理の原則を実演します。</p>

<h2>インターフェース</h2>

<p><span class="sidenote"><strong><code>security/src/lib.rs</code></strong></span>
セキュリティサービスは4つのプロシージャを公開します。<code>CREATE_TOKEN</code>は名前と権限セットを持つ新しいトークンを生成します。<code>VALIDATE_TOKEN</code>はトークンが有効かどうかをチェックし、関連するIDを返します。<code>REVOKE_TOKEN</code>は侵害されたトークンを無効化します。<code>LIST_TOKENS</code>はダッシュボード用にすべてのアクティブなトークンを列挙します。</p>

<pre class="code-security"><code>pub const CREATE_TOKEN_PROCEDURE: ProcedureId = 601;
pub const VALIDATE_TOKEN_PROCEDURE: ProcedureId = 602;
pub const REVOKE_TOKEN_PROCEDURE: ProcedureId = 603;
pub const LIST_TOKENS_PROCEDURE: ProcedureId = 604;

#[derive(Debug, Serializable, Deserializable)]
pub struct CreateTokenArgs {
    pub name: String,
    pub permissions: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ValidateTokenResult {
    pub valid: i32,
    pub name: String,
    pub permissions: String,
}</code></pre>

<h2>トークンベース認証</h2>

<p><span class="sidenote"><strong><code>security/src/main.rs</code></strong></span>
セキュリティサービスは認証トークン&mdash;&mdash;保護された操作へのアクセスを付与する不透明な文字列&mdash;&mdash;を管理します。各トークンは名前（所有者の識別）、権限のセット、作成タイムスタンプを持ちます。フロントエンドは、すべての機密性の高いダッシュボード操作（状態を変更するPOSTリクエスト）で<code>auth_token</code>クッキーをチェックし、セキュリティサービスの<code>VALIDATE_TOKEN</code>プロシージャを呼び出します。</p>

<pre class="code-security"><code>struct TokenEntry {
    name: String,
    token: String,
    permissions: String,
    created_at: u64,
}

struct SecurityState {
    tokens: HashMap&lt;String, TokenEntry&gt;,
    rng_state: u64,  // xorshift64 seed
}</code></pre>

<h2>トークン生成：分散システムにおけるPRNG</h2>

<p><span class="sidenote"><strong><code>security/src/main.rs</code></strong></span>
トークン生成は興味深い問題を提起します：分散システムではランダム性はどこから来るのか？ハードウェア乱数生成器は低速です。暗号論的PRNG（<code>/dev/urandom</code>のような）はより良いですが、それでもシステムコールを伴います。教育目的の実装では、xorshift64&mdash;&mdash;高速で自己完結型の擬似乱数生成器&mdash;&mdash;を使用します：</p>

<pre class="code-security"><code>fn xorshift64(&amp;mut self) -&gt; u64 {
    let mut x = self.rng_state;
    x ^= x &lt;&lt; 13;
    x ^= x &gt;&gt; 7;
    x ^= x &lt;&lt; 17;
    self.rng_state = x;
    x
}</code></pre>

<p>xorshift64生成器は周期2<sup>64</sup>&minus;1の一様分布64ビット値を生成します。2つの出力を連結して128ビットの16進トークンを形成します。本番システムでは暗号論的に安全なPRNGを使用しますが、xorshift64はコアコンセプトを実証します：シード値から一見ランダムな出力を生成する決定論的関数です。シードは起動時のシステムクロックから導出され、各インスタンスのトークンストリームを一意にします。</p>

<h2>ブートストラップ問題</h2>

<p><span class="sidenote"><strong><code>security/src/main.rs</code></strong> &mdash; 起動時のadminトークンシーディング</span>
トークンベースの認証は鶏と卵の問題を作り出します：すべての変更操作に有効なトークンが必要な場合、最初のトークンをどうやって作成するのか？初期のアプローチは<em>ブートストラップ例外</em>&mdash;&mdash;1つのルートを認証なしにすること&mdash;&mdash;でしたが、それは公開デプロイメントで攻撃対象面を作成します。代わりに、起動時に環境変数からadminトークンをシードします：</p>

<pre class="code-security"><code>let mut initial_state = SecurityState::new();
if let Ok(token) = std::env::var("ADMIN_TOKEN") {
    if !token.is_empty() {
        initial_state.tokens.insert(token.clone(), TokenEntry {
            name: "admin".to_string(),
            token,
            permissions: "admin".to_string(),
            created_at,
        });
    }
}</code></pre>

<p>起動スクリプト（<code>start.sh</code>）は<code>openssl rand -hex 16</code>でランダムトークンを生成し、<code>ADMIN_TOKEN</code>としてエクスポートし、運用者に表示します。運用者はブラウザコンソール（<code>document.cookie = "auth_token=...;path=/"</code>）でクッキーを設定し、ダッシュボードを使用して追加のトークンを作成できます。トークン作成を含むすべてのPOSTルートには有効な<code>auth_token</code>クッキーが必要です。</p>

<p>このパターンは実際のシステムでは標準的です。Kubernetesはクラスター初期化時にブートストラップトークンを作成します。クラウドプロバイダーはプロビジョニング時にシードされたIAMルート認証情報を使用します。重要な洞察は、ブートストラップシークレットは認証なしのHTTPエンドポイントではなく、デプロイ環境に存在すべきだということです。</p>

<h2>認可ミドルウェア</h2>

<p><span class="sidenote"><strong><code>frontend/src/main.rs</code></strong></span>
フロントエンドは認可をミドルウェア&mdash;&mdash;各保護されたルートハンドラの前に実行される関数&mdash;&mdash;として実装します：</p>

<pre class="code-security"><code>async fn require_admin(headers: &amp;str) -&gt; bool {
    if let Some(token) = parse_cookie(headers, "auth_token") {
        let result = security::validate_token(SECURITY_ADDR, token).await;
        return result.valid == 1;
    }
    false
}</code></pre>

<p>このパターン&mdash;&mdash;リクエストから認証情報を抽出し、中央機関に対して検証し、結果に基づいてアクセスをゲーティングする&mdash;&mdash;は、APIゲートウェイ、サービスメッシュ、Webフレームワークがプラネタリスケールで使用するのと同じパターンです。<a href="/ja/dashboard/security">セキュリティダッシュボード</a>ではトークンの作成、アクティブなトークンの表示、侵害されたトークンの取り消しができます。</p>

<h2>インテグリティ</h2>

<p><span class="newthought">認証は</span>誰が操作できるかを制御しますが、量に対する保護にはなりません。有効なユーザーでもリクエストでシステムを圧倒でき、攻撃者はリソースを消費するのに認証情報を必要としません。<em>インテグリティ</em>とは、敵対的な条件下でもシステムが機能し続けることを保証する実践です&mdash;&mdash;レート制限、IPブラックホール化、深層防御です。</p>

<h3>レート制限</h3>

<p><span class="sidenote"><strong><code>loadbalancer/src/main.rs</code></strong> &mdash; <code>TokenBucket</code>構造体</span>
ロードバランサーは<a href="https://en.wikipedia.org/wiki/Token_bucket">トークンバケット</a>アルゴリズムを使用してIPごとのレート制限を実装します。各IPアドレスは最大30トークン（バースト容量）を保持するバケットを取得し、毎秒2トークン（毎分約120リクエストを持続）で補充されます。各リクエストは1トークンを消費します。バケットが空の場合、リクエストは<code>429 Too Many Requests</code>で拒否されます：</p>

<pre class="code-security"><code>fn try_consume(&amp;mut self) -&gt; bool {
    let now = Instant::now();
    let elapsed = now.duration_since(self.last_refill).as_secs_f64();
    self.tokens = (self.tokens + elapsed * RATE_REFILL).min(RATE_CAPACITY);
    self.last_refill = now;
    if self.tokens &gt;= 1.0 {
        self.tokens -= 1.0;
        true
    } else {
        false
    }
}</code></pre>

<p>トークンバケットはバーストを許容しつつ（ユーザーがページを読み込む際に複数のリソースを一度にフェッチする）持続的なレートを強制するため優雅です。代替手段の固定ウィンドウカウンターには境界問題があり、クライアントがウィンドウの境界をまたいでリクエストのタイミングを合わせることで制限の2倍を送信できます。</p>

<h3>IPブラックホール化</h3>

<p><span class="sidenote"><strong><code>loadbalancer/src/main.rs</code></strong> &mdash; <code>record_violation()</code></span>
レート制限だけでは不十分です。繰り返し拒否される攻撃者は、各拒否でCPUサイクルを消費します。ロードバランサーは自動的にエスカレートします：IPが60秒以内に10回の拒否を蓄積した場合、5分間<em>ブラックホール化</em>&mdash;&mdash;禁止されます。ブラックホール化されたIPはリクエストボディが読まれる前に即座に<code>429 Retry-After: 300</code>を受け取ります：</p>

<pre class="code-security"><code>fn record_violation(&amp;mut self, ip: IpAddr) {
    let now = Instant::now();
    let (count, window_start) = self.violations
        .entry(ip).or_insert((0, now));
    if now.duration_since(*window_start) &gt; BLACKHOLE_WINDOW {
        *count = 0;
        *window_start = now;
    }
    *count += 1;
    if *count &gt;= BLACKHOLE_THRESHOLD {
        self.blacklist.insert(ip, now + BLACKHOLE_DURATION);
        self.violations.remove(&amp;ip);
    }
}</code></pre>

<p>バックグラウンドタスクが60秒ごとに3つのマップ（レート制限バケット、ブラックリストエントリ、違反カウンター）をスイープして期限切れの状態を削除します。これにより長時間稼働するデプロイメントからのメモリ成長を防ぎます。</p>

<h3>深層防御</h3>

<p>これらの保護は層として重なります。<a href="/ja/chapter/load-balancing" class="sys" style="color:#2A9D8F">ロードバランサー</a>は最初の防御線です：レート制限とブラックホール化はリクエストがバックエンドサービスに到達する前に行われます。フロントエンドは第2の防御線です：すべてのダッシュボードの変更操作はセキュリティサービスに対して検証された有効な<code>auth_token</code>クッキーを必要とします。<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスは可視性を提供します&mdash;&mdash;ロードバランサーは<code>rate_limited</code>と<code>blackholed</code>のメトリクスを報告し、運用者がリアルタイムで攻撃を検出できるようにします。</p>

<p>イントロスペクションエンドポイント（<code>/__lb_status</code>と<code>/__lb_strategy</code>）はループバックアドレスに制限されており、外部ユーザーがバックエンドトポロジーを読み取ったりロードバランシング戦略を変更したりすることを防ぎます。すべてのバックエンドサービスは<code>127.0.0.1</code>にバインドされインターネットから到達不能です&mdash;&mdash;ロードバランサーのみが<code>0.0.0.0</code>でリッスンします。これは本番リバースプロキシが使用するのと同じアーキテクチャです：強化された単一のエントリポイントが内部サービスにトラフィックを振り分けます。</p>
"##
}

pub fn chapter_capacity() -> &'static str {
    r##"
<h1>第15章: キャパシティ</h1>

<p><span class="newthought">すべてのシステムには</span>限界があります。サーバーが処理できるリクエスト数には上限があります。ディスクが保存できるバイト数には上限があります。ネットワークリンクが運べる帯域幅には上限があります。<em>キャパシティ</em>とは、これらの限界の測定と、システムがワークロードに対応するのに十分なリソースを確保する実践です。</p>

<p>キャパシティはスタックのあらゆるレベルで測定されます。ハードウェアレベル：CPUコア、メモリギガバイト、ディスクIOPS、ネットワーク帯域幅。ソフトウェアレベル：毎秒リクエスト数、同時接続数、キュー深度、キャッシュヒット率。サービスレベル：提供可能なユーザー数、保存可能なデータ量、達成可能なレイテンシー。</p>

<p>負荷とパフォーマンスの関係は線形であることはまれです。1秒あたり1,000リクエストを10msレイテンシーで処理するサーバーは、2,000で15ms、3,000で50ms、3,500で完全に崩壊するかもしれません。これらの非線形な関係を理解すること&mdash;&mdash;負荷テスト、モデリング、経験を通じて&mdash;&mdash;はキャパシティプランニングに不可欠です。</p>

<p>キャパシティ管理は継続的なプロセスです。トラフィックが成長し、新機能が追加され、使用パターンが変化する中で、システムのキャパシティ要件は進化します。<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスは時間の経過に伴うキャパシティ使用率を追跡するために必要なデータを提供し、<a href="/ja/chapter/configuration" class="sys" style="color:#3A86FF">コンフィギュレーション</a>サービスはキャパシティパラメータ（接続プールサイズやキャッシュ制限など）を再デプロイなしに調整することを可能にします。</p>
"##
}

pub fn chapter_utilization() -> &'static str {
    r##"
<h1>第16章: 使用率</h1>

<p><span class="newthought">キャパシティは</span>システムが<em>どれだけできるか</em>を教えてくれます。使用率はシステムが<em>どれだけ行っているか</em>を教えてくれます。使用率はキャパシティのパーセンテージとして表されます：CPU使用率70%のサーバー、85%使用済みのディスク、最大帯域幅の40%を運んでいるネットワークリンク。</p>

<p>高い使用率はリソースが効率的に使用されていることを意味しますが、トラフィックスパイクや予期しない障害に対するヘッドルームがほとんどないことも意味します。低い使用率はシステムがオーバープロビジョニングされていること&mdash;&mdash;リソースに支払っているが使用されていないこと&mdash;&mdash;を意味します。適切なポイントはサービスの要件によって異なります：レイテンシーに敏感なサービスはバーストのための余裕を残すために50%の使用率を目標とするかもしれません。一方、バッチ処理システムは90%を目標とするかもしれません。</p>

<p>使用率はすべてのリソース次元で同時に監視する必要があります。CPUの使用率が低くてもメモリの使用率が高いサーバーは、依然として障害のリスクがあります。ボトルネックリソース&mdash;&mdash;キャパシティに最も近いもの&mdash;&mdash;がシステムの実効キャパシティを決定します。ボトルネックの特定と解消は、コアとなる運用スキルです。</p>

<p><a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスは、キャッシュヒット率、キュー深度、リクエストレートなどの使用率メトリクスを追跡します。これらのメトリクスは、CPUやメモリ使用率などのシステムレベルのメトリクスと組み合わせることで、システムのリソースがどれだけ効果的に使用されているかの包括的な画像を提供します。</p>
"##
}

pub fn chapter_efficiency() -> &'static str {
    r##"
<h1>第17章: 効率性</h1>

<p><span class="newthought">効率性は</span>、消費されるリソース単位あたりにシステムが生産する有用な仕事量を測定します。1台のサーバーで毎秒10,000リクエストを処理するシステムは、同じスループットに10台のサーバーを必要とするシステムよりも効率的です。プラネタリスケールでは、小さな効率改善が巨大な節約に複利的に積み重なります&mdash;&mdash;100万台のサーバー全体で10%の改善は、10万台分のサーバーリソースを解放します。</p>

<p>効率性の改善はスタックのあらゆるレイヤーから生まれます。アルゴリズムレベル：ハッシュマップの検索はO(1)対線形スキャンのO(n)。データ構造レベル：<a href="/ja/chapter/caching" class="sys" style="color:#7209B7">キャッシング</a>サービスのLRUエビクションは最も有用なデータをメモリに保持します。プロトコルレベル：<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>サービスのコネクションプーリングはリクエストごとの接続オーバーヘッドを排除します。システムレベル：<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>サービスのコンパクションは削除されたエントリからディスク空間を回収します。</p>

<p>効率性とシンプルさ、信頼性、開発速度などの他の品質との間には緊張関係があります。時期尚早な最適化は、保守やデバッグが困難な複雑なコードを生み出すことが多いです。最も効果的なアプローチは、まずシンプルなシステムを構築し、本番でのパフォーマンスを測定し、実際のボトルネックを特定し（往々にして予想とは異なる場所にある）、外科的に最適化することです。</p>
"##
}

pub fn chapter_load_testing() -> &'static str {
    r##"
<h1>第18章: 負荷テスト</h1>

<p><span class="newthought">測定できないものは</span>管理できず、検証していない測定は信頼できません。負荷テストは、システムにさまざまな条件下でのパフォーマンス特性を測定するために、制御された人工的なトラフィックを負荷させます。以下のような疑問に答えます：このシステムは毎秒何リクエスト処理できるか？レイテンシーが許容できなくなるのはどの時点か？負荷下で依存先が障害を起こした場合はどうなるか？</p>

<p>負荷テストにはいくつかの形態があります。<em>ベースラインテスト</em>は通常の予想トラフィック下でのパフォーマンスを測定します。<em>ストレステスト</em>は期待される限界を超えてシステムを押し、ブレークポイントを見つけます。<em>ソークテスト</em>は中程度の負荷で長期間実行し、緩やかなリソースリークを検出します。<em>スパイクテスト</em>は突然のトラフィックバーストをシミュレートし、システムのサージ吸収能力を検証します。</p>

<p>負荷テストにおける最も一般的な間違いは、依存関係のコンテキストではなくシステムを孤立してテストすることです。<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>サービスのパフォーマンスは<a href="/ja/chapter/caching" class="sys" style="color:#7209B7">キャッシング</a>サービスのヒット率に依存し、それはトラフィックパターンに依存し、それは負荷テストの設計に依存します。現実的な負荷テストは、現実的なデータとアクセスパターンで完全な依存チェーンを実行する必要があります。</p>

<p>負荷テスト結果はキャパシティモデルとSLO（サービスレベル目標）と比較して、システムが要件を満たしているかどうかを判断する必要があります。結果はまた、コード変更によるパフォーマンスの退行を以前のテスト実行と比較して検出できるようにアーカイブする必要があります。</p>
"##
}

pub fn chapter_planning() -> &'static str {
    r##"
<h1>第19章: プランニング</h1>

<p><span class="newthought">キャパシティプランニング</span>は、将来のリソースニーズを予測し、需要が到来する前にインフラストラクチャが利用可能であることを確保する実践です。キャパシティ不足は危機です。キャパシティ過多は浪費です。良いプランニングはこれらの極端の間を縫います。</p>

<p>プランニングはデータから始まります：<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスからの過去の使用率トレンド、プロダクトチームからの成長予測、トラフィックスパイクを引き起こす可能性のある今後のローンチやイベントの知識。単純なトレンド外挿（ストレージが月5%成長しているなら、6ヶ月後にもう1台のサーバーが必要になる）は安定した成長に有効です。ステップ関数（一夜でトラフィックが倍増するプロダクトローンチ）にはより明示的なプランニングが必要です。</p>

<p>リードタイムは重要です。新しいハードウェアの注文に3ヶ月かかり、トラフィックが月10%成長している場合、キャパシティの70%でハードウェアを注文する必要があります&mdash;&mdash;枯渇してからではありません。クラウドインフラストラクチャはリードタイムを短縮します（数ヶ月ではなく数分で新しいサーバー）が、コスト管理や予約キャパシティに関する独自のプランニング課題を導入します。</p>

<p>最良のキャパシティプランには予期しない事態のための<em>バッファ</em>が含まれます：バイラルイベントによるトラフィックスパイク、実効キャパシティを減少させるハードウェア障害、低速化してキューを詰まらせる依存先。一般的な経験則は、予想ピーク使用率の20〜30%上のヘッドルームを維持することです。</p>
"##
}

pub fn chapter_degradation() -> &'static str {
    r##"
<h1>第20章: グレースフルデグラデーション</h1>

<p><span class="newthought">需要がキャパシティを超えた場合</span>、システムには2つの選択肢があります：完全に障害するか、グレースフルに劣化するかです。グレースフルデグラデーションとは、すべてのユーザーにエラーを返すのではなく、機能を減少させながらサービスを継続することを意味します。遅いウェブサイトと到達不能なウェブサイトの違いです。</p>

<p>デグラデーション戦略には<em>ロードシェディング</em>（残りを保護するためにリクエストの一部を拒否する）、<em>機能削減</em>（リソース消費を減少させるために高コストの機能を無効にする）、<em>優先キューイング</em>（高優先度リクエストを低優先度リクエストより先に処理する）が含まれます。各戦略は継続的な可用性のために一部の機能をトレードオフします。</p>

<p>私たちのシステムでは、<a href="/ja/chapter/caching" class="sys" style="color:#7209B7">キャッシング</a>サービスが<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>サービスの自然なデグラデーションパスを提供します。ストレージサービスが過負荷になった場合、キャッシングサービスはリクエストを完全に失敗させるのではなく、古いデータを提供できます。<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>サービスは過負荷のバックエンドへのトラフィック送信を停止し、より健全なインスタンスに負荷を分散できます。</p>

<p>デグラデーションはテストする必要があります。一度も実行されたことのないデグラデーション戦略は、機能しないデグラデーション戦略です。カオスエンジニアリング&mdash;&mdash;本番環境で意図的に障害を注入する実践&mdash;&mdash;は、デグラデーションメカニズムが本当に必要になる前に設計通りに機能することを検証する実践です。</p>
"##
}

pub fn chapter_load_balancing() -> &'static str {
    r##"
<h1>第21章: ロードバランシング</h1>

<p><span class="newthought">複数のサーバーが</span>同じリクエストを処理できる場合、ロードバランサーがどのサーバーが各リクエストを受け取るべきかを決定します。効果的なロードバランシングは、他のサーバーがアイドル状態にある間に1台のサーバーがボトルネックにならないことを保証します。分散システムにおけるスケーラビリティと信頼性の両方を達成するための基本的な技術です。</p>

<h2>ゲートウェイロードバランサー</h2>

<p>私たちのシステムのエントリポイントは、複数のフロントエンドインスタンスの前に位置するゲートウェイロードバランサーです。ヘルスステータスとアクティブ接続数で追跡されるバックエンドのプールを維持します：</p>

<span class="sidenote">完全な実装は<code>loadbalancer/src/main.rs</code>を参照してください。</span>

<pre class="code-loadbalancer"><code>struct Backend {
    address: String,
    healthy: bool,
    active_connections: usize,
}

struct LoadBalancer {
    backends: Vec&lt;Backend&gt;,
    strategy: String,
    next_index: usize,
}</code></pre>

<p>ロードバランサーはバックエンドを動的に発見します。5秒ごとにバックグラウンドタスクが<code>discovery::list("frontend")</code>を呼び出してバックエンドリストを更新します。新しいバックエンドは自動的に追加されます。<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>から消えたバックエンドは削除されます。</p>

<h2>バランシング戦略</h2>

<p>ゲートウェイは4つの戦略をサポートしており、<code>/__lb_strategy</code>エンドポイントまたは<code>STRATEGY</code>環境変数で実行時に選択できます：</p>

<p><em>ラウンドロビン</em>はリクエストを順番に配分し、不健全なバックエンドをスキップします。サーバーとリクエストが均一な場合にうまく機能します：</p>

<pre class="code-loadbalancer"><code>// round-robin: increment next_index, skip unhealthy
let start = self.next_index;
let total = self.backends.len();
for offset in 0..total {
    let idx = (start + offset) % total;
    if self.backends[idx].healthy {
        self.next_index = (idx + 1) % total;
        return Some(idx);
    }
}</code></pre>

<p><em>最少接続</em>は各リクエストをアクティブ接続が最も少ない健全なバックエンドに送信します。これは異種のリクエストコストに自然に対応します&mdash;&mdash;遅いリクエストは接続をより長く占有し、後続のトラフィックを他に振り向けます：</p>

<pre class="code-loadbalancer"><code>// least-connections: pick the healthy backend with fewest active
let mut best = healthy[0];
for &amp;i in &amp;healthy {
    if self.backends[i].active_connections
        &lt; self.backends[best].active_connections
    {
        best = i;
    }
}
Some(best)</code></pre>

<p><em>ランダム</em>は健全なバックエンドを一様にランダムに選択します。シンプルでステートレスですが、バックエンド数が少ない場合に不均一な分配を生む可能性があります。</p>

<p><em>二者択一（Pick-2）</em>は特に優雅なアルゴリズムです：2つのランダムな健全なバックエンドを選び、アクティブ接続が少ない方を選択します。研究により、これは純粋なランダム選択よりも指数関数的に良い負荷分配を達成し、最小限の調整で済むことが示されています。</p>

<pre class="code-loadbalancer"><code>// pick-2: choose 2 random, take the less loaded
let a = healthy[rng.gen_range(0..healthy.len())];
let mut b = a;
while b == a {
    b = healthy[rng.gen_range(0..healthy.len())];
}
if backends[a].active_connections &lt;= backends[b].active_connections {
    Some(a)
} else {
    Some(b)
}</code></pre>

<h2>設計の議論</h2>

<p>バランシング戦略の選択はワークロードに依存します。ラウンドロビンはリクエストのコストがほぼ等しくサーバーが均一な場合にうまく機能します。最少接続は異種のリクエストコストに自然に適応します。Pick-2はバランスを取り、最少接続のほとんどの利点をランダム選択のシンプルさで提供します。</p>

<p>2層アーキテクチャ&mdash;&mdash;エッジでのゲートウェイバランシングと<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>層内でのサービスレベルバランシング&mdash;&mdash;は深層防御を提供します。ゲートウェイがシンプルなラウンドロビンを使用していても、ルーティング層は個々のバックエンドサービスへのトラフィックを独立して最適化できます。</p>
"##
}

pub fn chapter_consistency() -> &'static str {
    r##"
<h1>第22章: 一貫性</h1>

<p><span class="newthought">データが複数のサーバーに</span>レプリケートされると、根本的な疑問が生じます：読み取り側はどのようなデータの保証を得られるのか？<em>一貫性</em>とは、分散システムにおける書き込みとその後の読み取りの関係を支配するルールのセットです。</p>

<h2>ストレージクォーラムレプリケーション</h2>

<p><a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>サービスは3レプリカ（N=3）として動作し、各レプリカが独自のWALとスナップショットを維持します。すべての値に単調増加するバージョン番号がタグ付けされます：</p>

<span class="sidenote"><code>storage/src/engine.rs</code>でバージョニングを参照してください。</span>

<pre class="code-storage"><code>struct VersionedValue {
    value: String,
    version: u64,
}

// put assigns the next version
pub fn put(&amp;mut self, key: String, value: String) -&gt; u64 {
    let version = self.next_version;
    self.next_version += 1;
    self.append_wal(&amp;format!("VPUT {}={}@{}", key, value, version));
    self.data.insert(key, VersionedValue { value, version });
    version
}</code></pre>

<p>クォーラム書き込みはクライアントに返す前にW個のackを必要とします。W=2、N=3の場合、書き込みノードはローカル書き込みを実行し（1つのackとしてカウント）、<code>discovery::list("storage")</code>を通じてピアにレプリケートし、追加の1つのピアackを待ちます。</p>

<p>クォーラム読み取りは対称的に機能します：R=2の場合、読み取りノードはローカルと1つのピアから読み取り、最新バージョンの値を返します。重要な不変条件は<strong>W + R &gt; N</strong>（2 + 2 &gt; 3）であり、これは任意の読み取りクォーラムが任意の書き込みクォーラムと重なることを保証します&mdash;&mdash;読み取りセット内の少なくとも1つのノードが最新の書き込みを持っています。</p>

<h2>バージョンゲーティング</h2>

<p><span class="sidenote"><strong><code>storage/src/engine.rs</code></strong></span>
レプリケーションメッセージが到着した場合、受信ノードはそれを受け入れるかどうかを決定する必要があります。<code>put_versioned</code>メソッドはlast-writer-winsを実装します：書き込みは、そのバージョンが現在の値のバージョン以上の場合にのみ受け入れられます。これにより、ネットワークで遅延した古いレプリケーションメッセージが新しいデータを上書きすることを防ぎます。</p>

<pre class="code-storage"><code>pub fn put_versioned(
    &amp;mut self, key: String, value: String, version: u64,
) -&gt; bool {
    if let Some(current) = self.data.get(&amp;key) {
        if version &lt; current.version {
            return false; // Reject stale write
        }
    }
    // Accept the write
    true
}</code></pre>
"##
}

pub fn chapter_placement() -> &'static str {
    r##"
<h1>第23章: プレースメント</h1>

<p><span class="newthought">プレースメントは</span>、物理インフラストラクチャのどこにデータや計算を配置すべきかを決定します。良いプレースメントはレイテンシーを削減し（データを消費者の近くに配置する）、信頼性を向上させ（障害ドメインをまたいでレプリカを分散する）、リソース使用率を最適化します（マシン間の負荷を均衡させる）。</p>

<p>データプレースメント戦略には<em>ハッシュ</em>（ハッシュ関数を使用してデータをサーバーに決定論的に割り当てる）、<em>レンジパーティショニング</em>（連続するキー範囲をサーバーに割り当てる）、<em>ディレクトリベース</em>のプレースメント（各キーについてルックアップサービスに問い合わせる）が含まれます。<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>サービスはサービスプレースメントのためのシンプルなディレクトリとして機能しますが、データレベルではなくサービスレベルで動作します。</p>

<p>コンシステントハッシュはデータプレースメントにとって特に重要な技術です。サーバーとキーの両方を仮想リング上の位置に割り当て、各キーはリング上で時計回りの次のサーバーに保存されます。サーバーが追加または削除された場合、キーのごく一部だけが再割り当てされ、データ移動を最小限にします。</p>

<p>プレースメントは障害ドメインを考慮する必要があります：障害が相関する物理的または論理的な境界です。ラック障害はラック内のすべてのサーバーに影響します。電源障害は電源ドメイン内のすべてのラックに影響します。建物の障害は建物内のすべての電源ドメインに影響します。重要なデータのすべてのレプリカを同じラックに配置するとレプリケーションの目的が失われます。</p>

<p>リージョンは最大の障害ドメインです。地理的リージョンにまたがって分散することで、データセンター全体に影響する災害から保護します。<a href="/ja/chapter/geo-replication">第24章: Geoレプリケーション</a>では、私たちのシステムが各リージョンでフルスタックを実行し、フェデレーテッドディスカバリとWALベースのレプリケーションがギャップを埋める方法を説明します。</p>
"##
}

pub fn chapter_geo_replication() -> &'static str {
    r##"
<h1>第24章: Geoレプリケーション</h1>

<p><span class="newthought">単一のデータセンターで</span>動作するシステムは、単一障害点を持つシステムです。いくつレプリカを実行しても、いかに慎重にフェイルオーバーを設計しても、光ファイバーをバックホーで切断されたり、送電網の停電、冷却システムの障害により、すべてが一度にオフラインになる可能性があります。グローバル分散&mdash;&mdash;複数の地理的リージョンでフルシステムを実行すること&mdash;&mdash;は、プラネタリスケールのシステムがリージョン障害を乗り越え、世界中のユーザーに低レイテンシーでサービスを提供する方法です。</p>

<h2>なぜ分散するのか</h2>

<p>3つの力がグローバル分散を駆動します。<em>レイテンシー</em>：光がサンフランシスコとニューヨーク間を往復するのに74ms、アムステルダムまでは165msかかります。ユーザーはこれに気づきます。各リージョンにシステムのコピーを実行することで、ほとんどのリクエストがローカルで処理されます。<em>可用性</em>：別々のリージョンの独立したインフラストラクチャは、あるリージョンの障害が他に影響しないことを意味します。SFOリージョンがダウンしても、NYCとAMSはトラフィックの提供を続けます。<em>データ主権</em>：一部の法域では、市民に関するデータをその境界内に留めることを要求しています。マルチリージョンアーキテクチャはコンプライアンスを可能にします。</p>

<h2>リージョンごとのフルスタック</h2>

<p>私たちのアプローチはシンプルです：各リージョンが完全なシステムを実行します。<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>、<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>、<a href="/ja/chapter/caching" class="sys" style="color:#7209B7">キャッシング</a>、<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>、<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>、<a href="/ja/chapter/scheduling" class="sys" style="color:#FF6B35">スケジューリング</a>&mdash;&mdash;すべてのサービスがすべてのリージョンで実行されます。ローカルリクエストはリージョンを離れません。リージョン境界を越えるのは2つだけ：ストレージレプリケーション（あるリージョンで書き込まれたデータが最終的に他のリージョンに現れる）とキャッシュ無効化（古いエントリがどこでもパージされる）です。</p>

<span class="sidenote">これは「アクティブ-アクティブ」マルチリージョンデプロイメントと呼ばれることがあります：すべてのリージョンが読み取りと書き込みを独立して処理でき、非同期レプリケーションで同期を維持します。</span>

<h2>WireGuardメッシュ</h2>

<p>リージョン間はプライベートなWireGuardメッシュネットワークで通信します。各リージョンは共有<code>10.0.0.0/24</code>サブネット上にWireGuard IPを持ちます：</p>

<pre class="code-block"><code>SFO   10.0.0.1
NYC   10.0.0.2
AMS   10.0.0.3</code></pre>

<span class="sidenote">WireGuardは最小限のオーバーヘッドで暗号化され認証されたトンネルを提供します。メッシュトポロジーは、すべてのリージョンがハブを経由せずに他のすべてのリージョンに直接到達できることを意味します。ネットワークオーバーレイの詳細は<a href="/ja/chapter/network">第32章: ネットワーク</a>を参照してください。</span>

<h2>フェデレーテッドディスカバリ</h2>

<p>マルチリージョンアーキテクチャの鍵となる洞察は、<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>自体がフェデレートされることです。各リージョンのディスカバリインスタンスは2つのレジストリを維持します：直接登録されたサービスの<em>ローカル</em>レジストリと、他のリージョンのピアディスカバリインスタンスから転送されたサービスの<em>フェデレーテッド</em>レジストリです。</p>

<p>バックグラウンドタスクが5秒ごとに実行されます。ローカルに登録されたすべてのサービスを収集し、<code>127.0.0.1</code>アドレスをリージョンのWireGuard IPに書き換え、<code>FEDERATED_REGISTER</code> RPCを使用して各ピアディスカバリインスタンスにこれらの登録を転送します。</p>

<p>結果として2つの相補的なビューが得られます。<code>discovery::list("storage")</code>はすべてのリージョンのすべてのストレージインスタンスを返します&mdash;&mdash;グローバルに伝播する必要があるキャッシュ無効化に便利です。<code>discovery::list_local("storage")</code>は現在のリージョンのインスタンスのみを返します&mdash;&mdash;クロスリージョンレイテンシーがコンセンサスを非実用的にするクォーラム操作に便利です。</p>

<h2>WALテーラー</h2>

<p>クロスリージョンストレージレプリケーションは<em>WALテーラー</em>によって処理されます。ローカルの<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>インスタンスが生成するライトアヘッドログファイルを読み取り、リモートストレージインスタンスにリプレイする軽量サービスです。各リージョンで1インスタンスが実行されます。</p>

<h2>レイテンシーのトレードオフ</h2>

<p>3リージョンデプロイメントにおけるリージョン間レイテンシーはおおよそ：</p>

<pre class="code-block"><code>SFO ↔ NYC    ~74ms RTT
SFO ↔ AMS   ~165ms RTT
NYC ↔ AMS    ~92ms RTT</code></pre>

<p>これらのレイテンシーにより、ほとんどのワークロードでクロスリージョンのクォーラムコンセンサスは非実用的です。私たちのアーキテクチャはリージョン内の一貫性にはローカルクォーラム（高速、サブミリ秒）を使用し、リージョン間には非同期レプリケーション（結果整合性、書き込みパスにレイテンシーペナルティなし）を使用します。</p>

<p>これはグローバル分散の根本的なトレードオフです：リージョン間の強い一貫性か低レイテンシーの書き込みかのどちらかを選べますが、両方は選べません。私たちのシステムは結果的なクロスリージョン一貫性を伴う低レイテンシー書き込みを選択しています。</p>
"##
}

pub fn chapter_localization() -> &'static str {
    r##"
<h1>第25章: ローカライゼーション</h1>

<p><span class="newthought">私たちはサーバーを</span>ユーザーの近くに配置してレイテンシーを削減します。しかしレイテンシーはミリ秒だけで測られるものではありません&mdash;&mdash;理解のレイテンシーでもあります。読者がすべての文を外国語から頭の中で翻訳しなければならない場合、理解の「ラウンドトリップタイム」は爆発的に増加します。ローカライゼーションとは、システムを特定の言語、地域、文化的慣習に適応させる実践です。データをユーザーの近くにレプリケートするのと同じように（<a href="/ja/chapter/geo-replication">第24章: Geoレプリケーション</a>を参照）、<em>コンテンツ</em>をユーザーの母国語でレプリケートして理解を近づけます。</p>

<span class="sidenote">国際化（i18n）は、ローカライゼーションを可能にするエンジニアリング作業です&mdash;&mdash;ロジックを変更せずにロケール固有のコンテンツを入れ替えられるようにコードを構造化すること。ローカライゼーション（l10n）は、特定のロケール向けのコンテンツを生産する行為です。この章では両方を扱います。</span>

<h2>ロケール検出</h2>

<p>ローカライズされたシステムが最初に答えなければならない質問は：このユーザーはどの言語を望んでいるか？です。私たちは優先順位で複数のシグナルをチェックする検出チェーンを使用します：</p>

<pre class="code-loadbalancer"><code>fn detect_lang(path: &amp;str, headers: &amp;str) -&gt; (Lang, &amp;str) {
    // 1. URL prefix: /ja/chapter/systems → Lang::Ja
    if path.starts_with("/ja/") {
        return (Lang::Ja, &amp;path[3..]);
    }
    if path == "/ja" {
        return (Lang::Ja, "/");
    }

    // 2. Cookie: lang=ja (persisted preference)
    if let Some(lang) = parse_cookie(headers, "lang") {
        if lang == "ja" {
            return (Lang::Ja, path);
        }
    }

    // 3. Accept-Language header: ja vs en quality values
    let lang = parse_accept_language(headers);
    (lang, path)
}</code></pre>

<p>チェーンは原則に従います：<strong>明示的な選択が暗黙的なシグナルを上書きする</strong>。URLプレフィックスが最も明示的です&mdash;&mdash;ユーザーが特定の言語のリンクをクリックしました。クッキーは以前の明示的な選択を記録します。<code>Accept-Language</code>ヘッダーはユーザーが意識的に設定していないかもしれないブラウザレベルのデフォルトです。</p>

<h2>コンテンツモジュール構造</h2>

<p>各言語には英語のオリジナルをミラーリングする独自のコンテンツモジュールがあります。英語コンテンツは<code>content.rs</code>に、日本語翻訳は<code>content_ja.rs</code>にあります。両方のモジュールは同一の関数シグネチャを公開します。ディスパッチ関数が検出された言語に基づいて正しいモジュールにルーティングします。</p>

<h2>翻訳はインフラストラクチャ</h2>

<p>よくある間違いは翻訳を後付けとして扱うことです&mdash;&mdash;システム全体を1つの言語で構築し、後から翻訳を付け足そうとすること。より良いアプローチは、ローカライゼーション<em>パイプライン</em>を先に構築することです：</p>

<p>1. <code>Lang</code>列挙型と検出チェーンを設計する。</p>
<p>2. コンテンツディスパッチ関数と並列モジュール構造を構築する。</p>
<p>3. 言語スイッチャーと<code>hreflang</code>タグを追加する。</p>
<p>4. <em>その後で</em>翻訳を生産する。</p>

<p>こうすることで、翻訳が始まる前にインフラストラクチャがテストされ動作しています。翻訳が到着すると、既存のモジュールにスロットインし、追加のコード変更なしで即座に機能します。</p>

<div style="background: var(--sidenote-bg); border: 1px solid var(--sidenote-border); border-radius: 6px; padding: 16px 20px; margin: 24px 0;">
<strong>試してみてください：</strong>サイドバーの<strong>English</strong>をクリックして、この章を英語で表示してみてください。URLが<code>/chapter/localization</code>に変わり、すべてのナビゲーションラベルが英語に切り替わることに注目してください。<strong>日本語</strong>をクリックして戻ります。
</div>
"##
}

pub fn chapter_traffic() -> &'static str {
    r##"
<h1>第26章: トラフィック</h1>

<p><span class="newthought">トラフィックは</span>分散システムの生命線です。トラフィックパターン&mdash;&mdash;リクエストがいつ到着するか、どこから来るか、何を要求するか&mdash;&mdash;を理解することは、キャパシティプランニング、パフォーマンス最適化、異常検知に不可欠です。</p>

<p>トラフィックパターンにはいくつかの共通の特徴があります。<em>日次パターン</em>は人間の活動の日々のリズムに従い、覚醒時間にピークがあり夜間に谷があります。<em>週次パターン</em>は平日と週末を区別します。<em>季節パターン</em>は休日、プロダクトローンチ、マーケティングキャンペーンなどのイベントを反映します。これらのパターンを理解することで、運用者はキャパシティの事前プロビジョニングや低トラフィック期間中のメンテナンスのスケジューリングが可能になります。</p>

<p>トラフィック管理技術には<em>レート制限</em>（クライアントが単位時間あたりに行えるリクエスト数を制限する）、<em>スロットリング</em>（リクエストを拒否するのではなく遅くする）、<em>アドミッション制御</em>（バックエンドリソースを消費する前にエッジでリクエストを拒否する）が含まれます。これらの技術は、自然な成長、フラッシュクラウド、悪意のある攻撃からシステムを過負荷から保護します。</p>

<p><a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスは、リクエストレート、エラーレート、レイテンシー分布のリアルタイム可視性を提供することで、トラフィック管理において重要な役割を果たします。これらのメトリクスの急激な変化は、トラフィックサージ、依存先の障害、新しくデプロイされたバージョンのバグを示している可能性があります。</p>
"##
}

pub fn chapter_faults() -> &'static str {
    r##"
<h1>第27章: 障害</h1>

<p><span class="newthought">プラネタリスケールコンピュータでは</span>、障害（フォールト）は例外的なイベントではなく&mdash;&mdash;標準です。数百万のコンポーネントがあれば、どこかで常に何かが故障しています：ディスクに不良セクタが発生し、ネットワークスイッチがパケットをドロップし、サーバーがメモリ不足になり、ソフトウェアバグがクラッシュを引き起こしています。問題は障害が発生するかどうかではなく、発生したときにシステムがどう対応するかです。</p>

<p>障害はそのスコープと期間によって分類できます。<em>一時的な障害</em>は短時間です：瞬間的なネットワークグリッチ、ガベージコレクションの一時停止、短時間のCPUスパイク。これらはリトライで最も効果的に処理されます。障害は通常自己解決するからです。<em>間欠的な障害</em>は予測不能に再発します：不安定なディスク、パケットロスのあるネットワークリンク、メモリリークのあるサービス。これらは検出と修復が必要です。<em>永続的な障害</em>は持続します：死んだディスク、故障したサーバー、破損したデータセット。これらは交換またはリカバリが必要です。</p>

<p>フォールトトレランスはあらゆるレベルの冗長性を通じて構築されます。データは複数のサーバーにレプリケートされます（<a href="/ja/chapter/consensus" class="sys" style="color:#06D6A0">コンセンサス</a>システムのように）。サービスは複数のマシンで実行されます（<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>サービスによる管理）。ネットワークパスは複製されます。電源にはバックアップがあります。目標は、単一の障害&mdash;&mdash;そして理想的には2つの同時障害の組み合わせ&mdash;&mdash;がユーザーに見える障害を引き起こさないことを保証することです。</p>

<p>検出はトレランスと同じくらい重要です。<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスはハートビートタイムアウトとメトリクスの異常を通じて障害を検出します。障害が早く検出されるほど、自動フェイルオーバーであれ人間の介入であれ、早く軽減できます。</p>
"##
}

pub fn chapter_outages() -> &'static str {
    r##"
<h1>第28章: 障害（アウテージ）</h1>

<p><span class="newthought">アウテージ</span>は、システムのフォールトトレランスを超える障害の目に見える結果です。十分な数のコンポーネントが同時に障害を起こした場合、またはカスケード障害が依存関係を通じて伝播した場合、システムはユーザーにサービスを提供できなくなります。アウテージは分散システムの生涯における最も重大なイベントです。</p>

<p>アウテージにはさまざまな原因があります。<em>ハードウェア障害</em>は個々のサーバーやラック全体をダウンさせます。<em>ソフトウェアバグ</em>はサービスのすべてのインスタンスを同時にクラッシュさせることがあります。<em>設定エラー</em>はルーティング、セキュリティ、キャパシティパラメータを誤って構成する可能性があります。<em>依存先の障害</em>は、障害を起こしたサービスがその依存元をキューに詰まらせ、最終的に障害に至るカスケードを引き起こします。<em>過負荷</em>は、トラフィックがシステムのキャパシティを超えた場合に発生します。</p>

<p>アウテージの影響は、その<em>スコープ</em>（影響を受けるユーザー数）、<em>期間</em>（どのくらい続くか）、<em>深刻度</em>（データが失われるのか、単にアクセスできなくなるだけなのか）によって決まります。ユーザーの1%に影響する1分間の部分的なアウテージと、データ損失を伴う1時間の完全なアウテージはまったく異なるものです。インシデント分類システムは、組織がアウテージに適切に対応するためのトリアージを支援します。</p>

<p>アウテージから得られる最も重要な教訓は、アウテージは単に乗り越えるだけでなく、研究されるべきだということです。ポストインシデントレビュー（責任を問わない振り返り）は、根本原因、寄与要因、そして将来の同様のアウテージを防止する是正措置を特定します。これらのレビューから得られた知見は、時間をかけて蓄積され、システムをより耐障害性の高いものにする組織的知識となります。</p>
"##
}

pub fn chapter_resources() -> &'static str {
    r##"
<h1>第29章: リソース</h1>

<p><span class="newthought">プラネタリスケールコンピュータ</span>は物理的なリソース上で動作します：コンピュート（命令を実行するプロセッサ）、メモリ（アクティブデータのための高速ストレージ）、永続ストレージ（耐久性のあるデータのための低速ストレージ）、そしてネットワーク（すべてを接続するリンク）です。これらのリソース&mdash;&mdash;その特性、限界、コスト&mdash;&mdash;を理解することは、効率的なシステムの設計に不可欠です。</p>

<p>各リソースタイプには速度と容量の階層があります。コンピュートについて：レジスタ、L1キャッシュ、L2キャッシュ、L3キャッシュ、メインメモリ、ディスクの順で、各ステップはおおよそ前のステップより1桁遅く1桁大きくなります。ストレージについて：NVMe SSD、SATA SSD、回転ディスク、テープの順で、アクセス時間はマイクロ秒から秒の範囲に及びます。ネットワークについて：ループバック、ローカルネットワーク、データセンターネットワーク、広域ネットワーク、大陸間リンクの順で、レイテンシーはマイクロ秒から数百ミリ秒の範囲に及びます。</p>

<p>リソースコストは桁違いに異なります。CPUサイクルのコストは実質的にゼロです。ディスクI/Oにはミリ秒単位のコストがかかります。大陸間のネットワークラウンドトリップには数百ミリ秒のコストがかかります。<a href="/ja/chapter/caching" class="sys" style="color:#7209B7">キャッシング</a>サービスは、この階層を活用して、頻繁にアクセスされるデータをメモリ（ナノ秒）に保持し、ストレージ（ミリ秒）やリモートサービス（数十ミリ秒）からの取得を避けています。</p>
"##
}

pub fn chapter_servers() -> &'static str {
    r##"
<h1>第30章: サーバー</h1>

<p><span class="newthought">サーバーは</span>、プラネタリスケールコンピュータにおけるコンピュートの基本単位です。現代のサーバーは、コンパクトなフォームファクタに膨大な能力を詰め込んでいます：数十のCPUコア、数百ギガバイトのメモリ、テラバイト単位のストレージ、毎秒数百ギガビットのネットワークリンクです。サーバーアーキテクチャを理解することは、ワークロードのサイジングやパフォーマンス特性の理解にとって重要です。</p>

<p>データセンターのサーバーは、通常19インチの標準ラックにマウントされます。1つのラックには40台以上のサーバーが収容でき、ネットワークスイッチ、電力分配、ケーブル管理装置も含まれます。ラックの密度&mdash;&mdash;コンピュート能力と消費電力の両面で&mdash;&mdash;はデータセンター設計の主要な制約条件です。</p>

<p>サーバー選定では、多くの次元のバランスを取る必要があります：CPUコア数対クロック速度、メモリ容量対帯域幅、ストレージ容量対IOPS、ネットワーク帯域幅対レイテンシーです。ワークロードによってプロファイルは異なります：<a href="/ja/chapter/caching" class="sys" style="color:#7209B7">キャッシング</a>サービスは大容量メモリと高速ネットワークが必要ですがストレージは少なくて済みます。<a href="/ja/chapter/storage" class="sys" style="color:#5E60CE">ストレージ</a>サービスは大容量ディスクと耐久性のある書き込みが必要ですがCPUコア数は少なくて済みます。コンピュート集約型のサービスは多数の高速コアが必要です。</p>

<p>ヘテロジニアスコンピュートへのトレンド&mdash;&mdash;CPUと並んでGPU、FPGA、カスタムアクセラレータを追加すること&mdash;&mdash;は、サーバー選定にさらなる次元を加えます。機械学習のトレーニングのようなワークロードはGPUアクセラレーションから大きな恩恵を受け、暗号化処理は専用ハードウェアにオフロードできます。</p>
"##
}

pub fn chapter_buildings() -> &'static str {
    r##"
<h1>第31章: 建物</h1>

<p><span class="newthought">データセンター</span>は、プラネタリスケールコンピュータの物理的な拠点です。現代のデータセンターはエンジニアリングの驚異であり、数千台のサーバーを高い稼働率で24時間稼働させるために、その収容、電力供給、冷却、接続を一から設計された建物です。</p>

<p>データセンターの設計はいくつかの制約によって決まります。電力が主要なコストであり主要な制約です&mdash;&mdash;大規模なデータセンターは小さな都市と同程度の電力を消費することがあります。冷却は、数千台のサーバーが発生する熱を放散しなければなりません。その技術は従来のエアコンから、液体冷却、寒冷地でのフリーエア冷却まで多岐にわたります。ネットワーク接続は、他のデータセンターやより広いインターネットへの高帯域幅・低レイテンシーのリンクを提供する必要があります。</p>

<p>データセンターの立地は戦略的な意思決定です。考慮すべき要素には、ユーザーへの近接性（低レイテンシーのため）、再生可能エネルギーへの近接性（持続可能性のため）、自然災害のリスク（地震、洪水、ハリケーン）、政治的安定性、そして不動産コストが含まれます。主要なクラウドプロバイダーは、すべての有人大陸にデータセンターを運営しており、各<em>リージョン</em>には通常、冗長性のために複数のデータセンター（<em>アベイラビリティーゾーン</em>と呼ばれる）が含まれています。</p>
"##
}

pub fn chapter_network() -> &'static str {
    r##"
<h1>第32章: ネットワーク</h1>

<p><span class="newthought">ネットワーク</span>は、プラネタリスケールコンピュータの神経系です。ラック内のサーバー同士、データセンター内のラック同士、リージョン内のデータセンター同士、そして地球全体のリージョン同士を接続します。ネットワークの特性&mdash;&mdash;帯域幅、レイテンシー、信頼性、コスト&mdash;&mdash;は、分散システムの設計を根本的に形作ります。</p>

<p>データセンター内のネットワークは、通常、マルチティア階層またはスパイン・リーフトポロジーとして構成されます。トップオブラック（ToR）スイッチがラック内のサーバーを接続します。アグリゲーションスイッチがクラスタ内のラックを接続します。コアスイッチがデータセンター内のクラスタを接続します。各ティアで帯域幅が集約され、障害ドメインが拡大します。</p>

<p>データセンター間のネットワーク接続は、専用光ファイバーリンク（同一メトロポリタンエリア内のデータセンター間）からリース回線や公衆インターネット（大陸間接続）まで多岐にわたります。同一リージョン内のデータセンター間のレイテンシーは通常1〜5ミリ秒です。大陸間では50〜200ミリ秒です。これらのレイテンシー制約はシステム設計に直接影響します：大陸間の強い一貫性はコストが高くなります。すべての書き込みがラウンドトリップを待つ必要があるからです。</p>

<p>ネットワーク障害は、分散システムにおけるアウテージの主要な原因です。サーバー障害（通常は独立して発生する）とは異なり、ネットワーク障害は大規模なサーバー群を同時にパーティション化し、スプリットブレインシナリオを引き起こす可能性があります。このようなシナリオに対処するために、<a href="/ja/chapter/consensus" class="sys" style="color:#06D6A0">コンセンサス</a>プロトコルが設計されています。</p>

<p>私たちの3リージョンデプロイメントは、WireGuardメッシュを使用してSFO、NYC、AMSを接続するプライベートオーバーレイネットワークを構築しています。<a href="/ja/chapter/geo-replication">第24章: Geoレプリケーション</a>では、このメッシュがフェデレーテッドディスカバリとクロスリージョンストレージレプリケーションをどのように実現するかを説明しています。</p>
"##
}

pub fn chapter_power() -> &'static str {
    r##"
<h1>第33章: 電力</h1>

<p><span class="newthought">電力は</span>プラネタリスケールコンピュータの究極のリソース制約です。すべて&mdash;&mdash;コンピューティング、ストレージ、ネットワーキング、冷却&mdash;&mdash;には電力が必要です。データセンターがサーバーに供給できる電力量が、その施設の最大コンピュート能力を決定します。</p>

<p>データセンターにおける電力供給は、複数レベルの冗長性を備えています。電力網からの商用電力が主要な電源です。無停電電源装置（UPS）は、系統電源の停電時にディーゼル発電機が起動するまでの数秒から数分間、バッテリーバックアップを提供します。発電機は燃料供給の制約のみで、数日から数週間にわたり施設に電力を供給できます。</p>

<p>電力使用効率（PUE）は、データセンター全体の消費電力とコンピューティング機器が消費する電力の比率を測定します。PUEが2.0の場合、コンピューティングに使用される1ワットごとに、冷却、照明、その他のオーバーヘッドに別の1ワットが使用されることを意味します。現代のデータセンターはPUE 1.1〜1.2を達成しており、電力の大部分がコンピューティングに直接使われていることを意味します。ホット/コールドアイル封じ込め、フリーエア冷却、液体冷却などの技術がこれらの改善に貢献しています。</p>

<p>データセンターの電力消費が環境に与える影響はますます重要になっています。主要な事業者は再生可能エネルギー源やカーボンオフセットに投資しています。再生可能エネルギー源&mdash;&mdash;水力発電ダム、風力発電所、太陽光発電施設&mdash;&mdash;の近くにデータセンターを配置することが、立地選定の重要な要素になりつつあります。</p>
"##
}

pub fn chapter_infra_management() -> &'static str {
    r##"
<h1>第34章: 管理</h1>

<p><span class="newthought">プラネタリスケールコンピュータの</span>物理インフラストラクチャを管理することは、膨大な運用上の課題です。複数の施設にまたがる数千台のサーバーを抱えているため、ハードウェア障害は日常的に発生します。ディスクが故障し、メモリにエラーが発生し、ネットワークカードが誤動作し、サーバー全体が応答しなくなります。効果的なインフラストラクチャ管理には、検出、診断、是正、交換のすべてのステップで自動化が必要です。</p>

<p>自動化されたハードウェア管理システムは、すべてのコンポーネントのインベントリ、健全性、ライフサイクルを追跡します。ディスクが（SMARTメトリクスを通じて）差し迫った障害の兆候を示した場合、システムは自動的に影響を受けるサーバーからトラフィックをドレインし、交換をスケジュールし、正常なレプリカにデータを移行します。サーバーが応答しなくなった場合、システムは電源サイクルを実行し、回復しない場合は物理的な修理が必要であるとマークします。</p>

<p>ファームウェアとBIOSのアップデートは、最小限の中断で数千台のサーバーにロールアウトする必要があります。これには、更新前にサーバーからワークをドレインするためのスケジューリングシステムとの連携と、更新がリグレッションを引き起こしていないことの検証が必要です。これらの操作の規模は手動管理を不可能にします&mdash;&mdash;すべてが自動化され、監査可能でなければなりません。</p>
"##
}

pub fn chapter_maintenance() -> &'static str {
    r##"
<h1>第35章: メンテナンス</h1>

<p><span class="newthought">メンテナンスとは</span>、プラネタリスケールコンピュータを健全な状態に保つために必要な継続的な作業です。メンテナンスのためにオフラインにできるパーソナルコンピュータとは異なり、プラネタリスケールコンピュータはユーザーへのサービスを継続しながらメンテナンスを行わなければなりません。これには、メンテナンス活動とインフラ上で実行されているサービス間の慎重な調整が必要です。</p>

<p>計画メンテナンスには、ハードウェア交換（故障したコンポーネントの交換）、ソフトウェアアップデート（オペレーティングシステムのパッチ、ファームウェアアップデート）、容量拡張（新しいサーバーやラックの追加）が含まれます。各メンテナンス作業は、実行中のサービスへの影響を最小限に抑えるようスケジュールされなければならず、スケジューリングと配置システムを使用して、メンテナンスが必要なサーバーからワークを移動させます。</p>

<p>計画外のメンテナンス&mdash;&mdash;予期しない障害への対応&mdash;&mdash;は、より困難なシナリオです。サーバーが予期せず故障した場合、<a href="/ja/chapter/discovery" class="sys" style="color:#F7B731">ディスカバリ</a>と<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスが障害を検出し、<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>サービスがそのサーバーへのトラフィック送信を停止し、障害を起こしたサーバーがリーダーであった場合は<a href="/ja/chapter/consensus" class="sys" style="color:#06D6A0">コンセンサス</a>システムが新しいリーダーを選出します。障害を起こしたサーバーは、その後通常のメンテナンスサイクルの一環として修理または交換されます。</p>
"##
}

pub fn chapter_edges() -> &'static str {
    r##"
<h1>第36章: エッジ</h1>

<p><span class="newthought">ネットワークのエッジ</span>は、プラネタリスケールコンピュータがユーザーと出会う場所です。エッジコンピューティングは、計算とデータをユーザーに近い場所&mdash;&mdash;ローカルデータセンター、ポイントオブプレゼンス（PoP）、セルタワー、さらにはエンドユーザーのデバイス&mdash;&mdash;に移動させます。これにより、レイテンシーが削減され、バックボーントラフィックが減少し、リアルタイムの応答性を必要とするアプリケーションが可能になります。</p>

<p>コンテンツデリバリーネットワーク（CDN）は、エッジインフラストラクチャの最も一般的な形態です。CDNは静的コンテンツ（画像、動画、スクリプト）を世界中の数百のエッジロケーションにキャッシュし、最も近いロケーションからユーザーにサービスを提供します。これにより、200ミリ秒の大陸間フェッチが5ミリ秒のローカルキャッシュヒットに変わります。</p>

<p>キャッシング以外にも、エッジコンピューティングはユーザーに近い場所でアプリケーションロジックを実行できます。エッジファンクション（クラウドファンクションに似ていますがエッジロケーションで実行される）は、オリジンデータセンターへのラウンドトリップなしに、認証、パーソナライゼーション、リクエストルーティングを処理できます。</p>

<p>エッジコンピューティングの課題は、多数の小さなロケーション間で一貫性を管理することです。各エッジロケーションは、古いデータや矛盾する状態の潜在的な発生源です。私たちが学んできた技術&mdash;&mdash;TTLを使ったキャッシング、結果整合性、重要なデータのためのコンセンサス&mdash;&mdash;はすべてエッジにも適用されますが、トレードオフは可用性と低レイテンシーの方向にシフトします。</p>

<p>私たちの3リージョンデプロイメント（SFO、NYC、AMS）は、エッジに近い分散の実用的な形態です&mdash;&mdash;数百のロケーションではありませんが、主要な人口集中地域に低レイテンシーアクセスを提供するには十分です。<a href="/ja/chapter/geo-replication">第24章: Geoレプリケーション</a>では、各リージョンでフルスタックが稼働し、非同期のクロスリージョンレプリケーションが行われる仕組みを紹介しています。</p>
"##
}

pub fn chapter_site_events() -> &'static str {
    r##"
<h1>第37章: サイトイベント</h1>

<p><span class="newthought">サイトイベント</span>とは、プラネタリスケールコンピュータの可用性、パフォーマンス、正確性に影響を与える重大なインシデントです。単一サービスへの短時間のレイテンシースパイク（軽微）から、数時間に及ぶ完全なデータセンター障害（重大）まで多岐にわたります。組織がサイトイベントをどう検出し、対応し、学ぶかが、そのシステムの長期的な信頼性を決定します。</p>

<p>サイトイベントのライフサイクルには明確なフェーズがあります：検出、トリアージ、軽減、解決、ポストインシデントレビュー。検出は<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスを通じて自動化されるべきです&mdash;&mdash;ヘルスメトリクスが閾値を超えたり、トラフィックパターンに異常が検出されたりするとアラートが発火します。目標は、ユーザーが報告する前にインシデントを検出することです。</p>

<p>トリアージはインシデントの深刻度とスコープを判断します。全ユーザーに影響しているのか、サブセットか？データにリスクはあるか？インシデントは拡大しているか？これらの質問への回答が対応を決定します：誰がページされるか、どのようなコミュニケーションが行われるか、どのような即時のアクションが取られるか。</p>

<p>軽減は、根本原因がまだ理解されていなくても、できるだけ早くサービスを回復することに焦点を当てます。一般的な軽減アクションには、最近のデプロイメントのロールバック、健全なレプリカへのフェイルオーバー、過負荷のコンポーネントへの圧力を減らすためのロードシェディング、不具合のある機能の無効化が含まれます。根本原因分析は後で、ポストインシデントレビューで行われます。</p>
"##
}

pub fn chapter_detection() -> &'static str {
    r##"
<h1>第38章: 検知</h1>

<p><span class="newthought">あらゆるインシデント管理の</span>第一歩は、何かが間違っていることを知ることです。検知は、サイレントな障害とアクティブな対応の間の橋渡しです。組織がインシデントを早く検知するほど、影響範囲は小さく、復旧時間は短くなります。検知レイテンシー&mdash;&mdash;問題が発生してから誰かにアラートが通知されるまでの時間&mdash;&mdash;は、チームが追跡できる最も重要な信頼性メトリクスの1つです。</p>

<p>自動検知は<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスが収集するシグナルに依存します：ハートビート、レイテンシーパーセンタイル、エラーレート、飽和メトリクス、注文完了率などのビジネスレベルの指標。アラート閾値は、これらのシグナルが通常の変動からアクション可能な領域に移行するタイミングを定義します。閾値が低すぎるとアラート疲れを生み、高すぎるとインシデントが見逃されます。異常検知&mdash;&mdash;正常なパターンを学習し偏差をフラグ付けする統計モデル&mdash;&mdash;は、新しい障害モードをキャッチするために静的閾値を補完できます。</p>

<p>すべてのインシデントが自動システムでキャッチされるわけではありません。ユーザーレポート、サポートチケット、ソーシャルメディアの言及は、特に内部メトリクスがキャプチャしない方法でユーザー体験に影響する問題に対して、貴重な検知チャネルです。堅牢な検知戦略は、自動モニタリングと人間の観察を組み合わせて、どのカテゴリの障害も長期間見逃されないようにします。</p>

<p>オンコールエンジニアは検知チェーンにおける人間のリンクです。アラートが発火すると、オンコール対応者はそれを確認し、本当のインシデントを表しているかどうかを評価し、次のステップを決定する必要があります。<a href="/ja/chapter/routing" class="sys" style="color:#2A9D8F">ルーティング</a>サービスは、対応者が調査している間に不健全なバックエンドからトラフィックを自動的にシフトでき、即時の人間の介入なしに時間を稼ぎます。</p>
"##
}

pub fn chapter_escalation() -> &'static str {
    r##"
<h1>第39章: エスカレーション</h1>

<p><span class="newthought">インシデントが検出されたら</span>、次の重要な判断はどれだけ緊急に対応するか、誰が関与する必要があるかです。エスカレーションは、適切なタイミングで適切な人々にインシデントを上げるプロセスです。エスカレーション不足は重大な問題を少なすぎる対応者に任せます。過度のエスカレーションは注意力を浪費し、深刻度システムに対するシニシズムを生みます。</p>

<p>深刻度レベルはインシデントの緊急性に関する共有語彙を提供します。一般的なスキームは4段階を使用します：全ユーザーに影響する完全なサービス障害にSEV1、大きなサブセットに影響する重大な劣化にSEV2、限定的なユーザー影響の部分的な問題にSEV3、即時のユーザー影響のない軽微な問題にSEV4。各深刻度レベルは特定の対応にマッピングされます：誰がページされるか、どれだけ迅速に対応すべきか、どのようなコミュニケーションが期待されるか。</p>

<p>インシデントコマンダーの役割はエスカレーションの要石です。この人物がインシデント対応を所有します：対応者を調整し、調査タスクを委任し、さらなるエスカレーションの判断をし、ステークホルダーへのコミュニケーションが流れることを保証します。インシデントコマンダーは最も上級のエンジニアである必要はありません&mdash;&mdash;プレッシャーの下で冷静に対応を組織できる人である必要があります。</p>

<p>マルチサービスインシデント&mdash;&mdash;あるシステムの障害が他のシステムにカスケードする場合&mdash;&mdash;は、チーム間の調整を必要とします。<a href="/ja/chapter/monitoring" class="sys" style="color:#B5179E">モニタリング</a>サービスはどのサービスが影響を受けているかを明らかにでき、サイトイベントフレームワークは適切なチームを集めるための構造を提供します。</p>
"##
}

pub fn chapter_root_causes() -> &'static str {
    r##"
<h1>第40章: 根本原因</h1>

<p><span class="newthought">インシデントが軽減された後</span>、最も重要な作業が始まります：なぜ起きたのかを理解することです。根本原因分析は、何が壊れたかを特定するだけでなく、障害が発生することを許した深いシステム的条件を理解することです。サーバーのディスク容量が不足したことは直接的な原因です。根本原因はキャパシティアラートが存在しなかったこと、または新しいサービスにログローテーションが設定されていなかったことかもしれません。</p>

<p>「5つのなぜ」技法はシンプルですが効果的な方法です：なぜ障害が発生したかを問い、なぜその条件が存在したかを問い、システム的な原因に到達するまで繰り返します。答えが単一の根本原因であることは稀です。ほとんどのインシデントには複数の寄与要因&mdash;&mdash;潜在的なバグ、設定のギャップ、モニタリングの盲点&mdash;&mdash;が重なって障害を生み出しています。すべての寄与要因を特定することは、1つの「根本原因」を特定して責任を問うことよりも価値があります。</p>

<p>責任追及は学習の敵です。エンジニアがミスに対する罰を恐れると、情報を隠し、組織は障害から学ぶ能力を失います。責任を問わないポストインシデントレビューは、個人ではなくシステムに焦点を当てます。「誰がこれを引き起こしたか？」ではなく「どのような条件がこれを許したか、そしてシステムをどう変えれば二度と起きないようにできるか？」が問いです。</p>

<p>調査結果の文書化は不可欠です。よく書かれたポストインシデントレビューは、タイムライン、寄与要因、影響、是正措置を記録します。これらの文書は組織の機関記憶の一部となり、将来のエンジニアが過去のインシデントを直接経験することなく学べるようにします。</p>
"##
}

pub fn chapter_remediation() -> &'static str {
    r##"
<h1>第41章: 修復</h1>

<p><span class="newthought">修復（レメディエーション）とは</span>、インシデント後にシステムを完全な健全性に戻す作業です。3つの時間スケールで動作します：出血を止めるための即時軽減、システムを安定化するための短期修正、根本原因に対処するための長期是正措置。各時間スケールでは、速度と徹底さの間で異なるトレードオフが必要です。</p>

<p>即時軽減は完璧さよりも可用性を優先します。不良デプロイメントのロールバック、健全なレプリカへのフェイルオーバー、非クリティカルな負荷のシェディング、フィーチャーフラグによる不具合機能の無効化&mdash;&mdash;これらのアクションは根本原因がまだ理解されていなくてもサービスを迅速に回復します。<a href="/ja/chapter/degradation" class="sys" style="color:#555">デグラデーション</a>と<a href="/ja/chapter/load-balancing" class="sys" style="color:#555">ロードバランシング</a>の技術は軽減ツールキットにおける必須のツールです。</p>

<p>短期修正は直接的な技術的原因に対処します。メモリリークがサービスをクラッシュさせた場合、短期修正はリークをパッチします。設定変更がカスケード障害を引き起こした場合、短期修正は設定を元に戻しバリデーションを追加します。これらの修正は通常の<a href="/ja/chapter/release" class="sys" style="color:#555">リリース</a>プロセスを通じてデプロイされます。</p>

<p>長期是正措置はポストインシデントレビューから生まれ、インシデントの発生を許したシステム的条件をターゲットにします。以前に観測されていなかった障害モードのモニタリング追加、キャパシティプランニングの改善、障害のクラスを排除するためのコンポーネントの再設計などが含まれます。アクションアイテムは完了まで追跡されなければなりません&mdash;&mdash;割り当てられたが完了しないアクションアイテムは再発に対する保護を提供しません。</p>
"##
}

pub fn chapter_prevention() -> &'static str {
    r##"
<h1>第42章: 予防</h1>

<p><span class="newthought">最良のインシデント</span>は、決して起きないインシデントです。予防は、リアクティブな対応からプロアクティブなレジリエンスへ焦点を移します。すべての障害を防ぐことはできませんが、意図的なエンジニアリングプラクティスはインシデントのクラス全体を排除し、発生するインシデントの深刻度を軽減できます。</p>

<p>カオスエンジニアリングは、実際のインシデントを引き起こす前に弱点を発見するために、本番システムに意図的に障害を注入する実践です。ゲームデイ&mdash;&mdash;チームが大規模な障害をシミュレートする計画された演習&mdash;&mdash;は、技術的なレジリエンスと人間の準備態勢の両方を構築します。プレモーテムはポストインシデントレビューを逆転させます：新しいシステムを立ち上げる前に、チームはすでに壊滅的に失敗したと想像し、何がうまくいかなかった可能性があるかを逆算して特定します。</p>

<p>手作業の自動化は強力な予防措置です。ランブックのすべての手動ステップは、ストレス下での人的エラーの機会です。ルーチンの運用タスク&mdash;&mdash;証明書のローテーション、キャパシティスケーリング、フェイルオーバー手順&mdash;&mdash;を自動化することで、これらのエラーが起きやすいステップを排除し、エンジニアが新しい問題に集中できるようにします。深層防御は、単一の障害がサイト全体のアウテージにカスケードしないことを保証します。</p>

<p>予防は最終的には文化的な実践です。<a href="/ja/chapter/security" class="sys" style="color:#555">セキュリティ</a>レビュー、<a href="/ja/chapter/load-testing" class="sys" style="color:#555">負荷テスト</a>、責任を問わないポストインシデントプロセスに投資する組織は、信頼性が全員の責任である文化を構築します。インシデント再発クラスの追跡&mdash;&mdash;同じタイプのインシデントが二度と起きないことを確保する&mdash;&mdash;は、組織が障害から単に生き残るのではなく学んでいることの最も強いシグナルです。</p>
"##
}

pub fn chapter_communication() -> &'static str {
    r##"
<h1>第43章: コミュニケーション</h1>

<p><span class="newthought">サイトイベント中</span>、コミュニケーションは技術的対応と同じくらい重要です。ユーザーは問題が存在すること、対処中であること、いつ解決される見込みかを知る必要があります。内部チームは努力を調整し、調査結果を共有し、作業の重複を避ける必要があります。効果的なコミュニケーションが、うまく管理されたインシデントと混沌としたインシデントの違いになり得ます。</p>

<p>インシデント中の内部コミュニケーションは、通常、すべての対応者が観察結果を共有しアクションを調整できる専用チャネル（チャットルーム、ブリッジコール、またはその両方）を使用します。<em>インシデントコマンダー</em>が対応をリードし、タスクを委任し、進捗を追跡し、決定を下します。<em>スクライブ</em>がイベントのタイムライン、取られたアクション、その結果を記録し、ポストインシデントレビューの原材料を作成します。</p>

<p>外部コミュニケーションは透明性と正確性のバランスが必要です。根本原因に関する時期尚早な声明は誤りである可能性があり、信頼を損ないます。ステータスページの更新は、何がわかっているか（インシデントのスコープと影響）、何が行われているか（進行中の軽減アクション）、次の更新はいつか（期待値の設定）を述べるべきです。原因について推測するよりも「調査中」と述べる方が良いです。</p>

<p>ポストインシデントレビューは最も価値のあるコミュニケーション成果物です。責任を問わない文書として書かれ、タイムライン、根本原因、寄与要因、是正措置を記述します。これらのレビューは、組織全体で共有され、機関記憶を構築し、同じクラスのインシデントの再発を防ぎます。最良のエンジニアリング組織は、ポストインシデントレビューを官僚的なオーバーヘッドではなく、最も重要な学習メカニズムの1つとして扱います。</p>
"##
}

pub fn afterword() -> &'static str {
    r##"
<h1>あとがき</h1>

<p><span class="newthought">この旅は</span>、シンプルな観察から始まりました：インターネットはコンピュータを使うことの意味を変えた。42の章を通じて、私たちはプラネタリスケールコンピューティングを可能にする機械を&mdash;&mdash;一つ一つ&mdash;&mdash;構築してきました。シリアライゼーション形式とRPCプロトコルから、コンセンサスアルゴリズム、キャッシング層とストレージエンジン、サーバー、建物、電力システムの物理インフラストラクチャまで。</p>

<p>一つの教訓を持ち帰っていただけるとすれば、これらのシステムは魔法ではないということです。制約の中で意図的なトレードオフを行うエンジニアの仕事です。私たちが研究したすべてのシステムは選択です：一貫性か可用性か、レイテンシーかスループットか、シンプルさか柔軟性か。システムエンジニアリングの技術は完璧な答えを見つけることではなく、目の前の問題にとってどのトレードオフが正しいかを理解することにあります。</p>

<p>この分野はまだ若いです。私たちは真にプラネタリスケールのコンピュータの第一世代を構築しており、最も困難な問題の多くはまだ未解決です。異なるプライバシー法を持つ法域にまたがるシステムをどう構築するか？プラネタリスケールコンピューティングのエネルギー消費を持続可能にするにはどうするか？これらの強力なシステムが、構築する余裕のある者だけでなく、全人類に奉仕することをどう保証するか？</p>

<p>本書を書いたのは、これらのシステムがどう機能するかを理解することは、最大規模の企業のエンジニアだけに留保された特権であるべきではないと信じるからです。技術は秘密ではありません&mdash;&mdash;経験を通じて得られ、論文やポストモーテムを通じて共有され、何年もの運用を通じて洗練されています。本書がその知識の一部をよりアクセスしやすくし、あなたがそれを使って信頼性と効率性だけでなく人間味のあるシステムを構築してくれることを願っています。</p>

<p>プラネタリスケールコンピュータは目的地ではありません。それを受け継ぐ各世代のエンジニアによって構築され、再構築される継続的なプロジェクトです。このプロジェクトの小さな一部をあなたと共有できたことを嬉しく思います。</p>

<p class="attribution">&mdash; <em>Justin J. Meza, San Francisco, 2025</em></p>
"##
}

pub fn colophon() -> &'static str {
    r##"
<h1>奥付</h1>

<p><span class="newthought">本書は</span>、Edward Tufteの仕事に触発されたワイドマージンとサイドノートレイアウトを提供する<code>tufte-book</code>ドキュメントクラスを使用して、LaTeXで執筆・組版されました。本文はPalatinoで設定されています。コードリストは<code>listings</code>パッケージによってレンダリングされたモノスペースフォントを使用しています。</p>

<p>本書で説明されているすべてのサービス&mdash;&mdash;正規化、RPC、ディスカバリ、ルーティング、エコー、コンフィギュレーション、キャッシング、ストレージ、モニタリング&mdash;&mdash;はRustで実装されスタンドアロンのバイナリにコンパイルされます。コードはテキストと並行して読まれることを意図しています。本番環境への堅牢化よりもシンプルさが優先されました：目標は、デプロイ準備の整ったソフトウェアを構築することではなく、各概念を数百行のコードで理解可能にすることでした。</p>

<p>本書のWeb版は、それ自体が本書で説明されているシステムの1つです。フロントエンドは外部Webフレームワークを使用せずにRustで書かれた生のTCP HTTPサーバーです。本書の章を提供し、ストレージとキャッシングサービスを通じてユーザーのハイライトを管理し、実行中のシステムの状態を表示するライブダッシュボードを提供します。サービスのコンステレーション全体が単一のシェルスクリプトから起動されます。</p>

<p>大規模言語モデルは、原稿とコードの両方の準備に広く使用されました。ドラフティングパートナー、コードレビュアー、そして疲れを知らないペアプログラマーとして機能しました。すべてのエラー、省略、疑わしいトレードオフについては著者が責任を負います。</p>
"##
}
