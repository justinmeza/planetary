// frontend/src/content.rs
// All chapter content as HTML string fragments, converted from psc.tex
// and written new for Configuration, Caching, Storage, and Monitoring.

pub fn foreword() -> &'static str {
    r##"
<h1>Foreword</h1>

<p><span class="newthought">The Internet</span> has fundamentally changed the way that we interact
with computers.  Prior to the 21st century, if you wanted to interact with a
computer, the computer was usually relatively near by (in the case of
time-shared and terminal-based mainframes) or right in front of you (in the
case of a personal computer).</p>

<p>Something changed, however, around the turn of the millennium.  High-bandwidth,
low-latency internet connections became more common.  Surfaces with which to
access the Internet, such as laptops, tablets, phones, and watches, began
enabling humans around the world to remain as connected to the digital world as
they would like.</p>

<p>At the same time, advances in processors, memory, storage, and network have
enabled facilities densely packed with servers to pop up around the globe.  Not
only that, many of these facilities rent out their hardware for anyone to use,
for a price.  All it takes is an Internet connection, a good idea, and a small
amount of capital to launch the next multi-billion dollar industry.</p>

<p>Unfortunately, the most successful practices, design patterns, and best known
methods often remain elusive to all but a select set of engineers and
scientists at the biggest institutions.  Even if someone wanted to recreate and
explore some of the same challenges, how would they find or fund the sheer
number of machines required to do so?</p>

<p>This book is for anyone who wants to learn how to design, build, and operate
computer systems.  Whether you have an idea that you want to bring to life, or
have an existing system that you want to get running in the best shape
possible, or if you just want to learn what goes on behind the scenes in a
multi-million-server infrastructure &mdash; there is something in this book for
you.</p>

<p>And while the field of computer systems is constantly evolving, I hope to share
with you some of the techniques that have stood the test of time &mdash;
battle-hardened through many millions of machine-years of operation.  At the
same time, I hope to pique your curiosity in some of the newer emerging fields,
so that perhaps your own ideas may help advance these fields.</p>

<p>I believe that we are at the precipice of a new paradigm of computing &mdash;
planetary scale computing.  Just as we had to rethink what it meant to build
and operate computers when moving from mainframes to personal computers and
from personal computers to warehouse-scale computers, so too will we need to
revisit our assumptions about computers that span the globe.</p>

<p>My goal is to paint a picture of what is possible and how to achieve it based
on my own experience and the experience of others.  I hope to expand the realm
of possibility in your mind so that you do not settle for the wrong solution to
a problem. I am excited for you to join me on this journey.  Onward!</p>

<p class="attribution">&mdash; <em>Justin J. Meza, San Francisco, 2024</em></p>
"##
}

pub fn preface() -> &'static str {
    r##"
<h1>Preface</h1>

<p><span class="newthought">This book</span> describes how to design, build, and operate some of the
largest and most complex computer organizations humanity has conceived.  As
such, we naturally have a lot of information to cover!  While every attempt has
been made to describe how to do things &ldquo;from the ground up,&rdquo; we do assume the
reader has some background knowledge of computers, networks, operating systems,
and programming languages.</p>

<p>Since we are only at the dawn of planetary scale computing, I fully expect and
encourage readers to identify and suggest improvements to the systems presented
in this book.  I consider this book a living digest of the best known methods
and I hope that it evolves with the field as new techniques are discovered and
debated over time.  I will attempt to keep it updated with the best practices
and patterns from the wild.</p>

<p>All of the code in this book is freely available for study.  I attempted to
keep the code as simple as possible to understand the key concepts, and no
simpler.  I attempt to note the drawbacks of such simplistic code, where
relevant.  Large language models were a valuable asset in creating such a large
and cohesive corpus of code, and I encourage interested readers to leverage
these tools to analyze, learn from, and extend the information presented in
this book.</p>
"##
}

pub fn chapter_systems() -> &'static str {
    r##"
<h1>Chapter 1: Systems</h1>

<p><span class="newthought">If you have</span> used the Internet, then you have interacted with
computer systems.  A computer system is a collection of software and hardware
that is used to perform some work.  Unlike a computer program, which may run on
a single piece of hardware, a computer system is spread out &mdash; across multiple
programs, devices, servers, and even geographic locations.</p>

<p>Computer systems come in many shapes and sizes.  You may already be familiar
with some common systems that run on single computers, such as operating
systems and file storage systems.  Systems are the glue that hold together
different hardware and software components.  This property of computer systems
is fundamental:  A computer system manages a set of components.</p>

<p>Systems are the primary building block for constructing planetary scale
computers.  A key feature of systems is that they are composable via some
standard interface.  A well-designed system behaves for a collection of
hardware and software like an abstracted and encapsulated function behaves for a
program.  The primary goal of any system is to provide a reusable way to
accomplish some task.</p>

<h2>Architectures</h2>

<p>Because computer systems provide well-defined interfaces that expose a system's
functionality, humans have come up with different ways to relate one system to
another.  If systems were hollow blocks in the physical world, we would only
have so many ways to put the blocks in relation to one another.  We could put
some blocks <em>inside</em> others, <em>beside</em> others, or <em>on top
of</em> others.</p>

<h3>Monolithic</h3>

<p>A monolithic system architecture is like one big system that contains other
systems inside of it.  The one big system provides as many (or as few)
interfaces to the systems it contains as needed.  The big system &ldquo;ties
together&rdquo; all of the other systems.  A web app that exposes all of
its features into different HTTP endpoints is an
example of a monolithic system.</p>

<p>Monolithic systems have the advantage of avoiding communication with other
systems in order to get their work done because all of the functionality they
need is bundled in the same binary.  A disadvantage of a monolithic system is
that in order to update a small component of the system, you need to build and
release a new version of the entire system.</p>

<p>You might choose a monolithic system when you need to avoid communication
between components, when your binary can fit within the resources of a single
server (but can be distributed across many servers), and if your build and
release process can handle the volume of changes to components of the
monolithic binary.  With the correct build and release process, a monolithic
approach can support many hundreds of features and thousands of changes per
day.</p>

<h3>Micro</h3>

<p>Micro systems are like smaller blocks that sit beside one another.  Each micro
system performs a specific, encapsulated task with a relatively simple
interface.  A system that stores and accesses values in persistent
storage is an example of a micro system.  Whereas
monolithic systems exist as a single binary with all functionality sharing the
same address space, micro systems exist as separate binaries and run as
separate processes.</p>

<p>Separating functionality into separate binaries comes with trade-offs.
Different micro systems can be implemented in separate programming languages,
allowing for greater development flexibility.  Micro systems can be updated and
released individually, increasing overall system availability.  In addition,
micro systems can be deployed across completely separate devices.</p>

<p>Operation becomes more complex with micro systems.  A micro system must
communicate with other micro systems whose functionality it uses, creating
dependencies.  Because micro systems may not be physically situated close to
one another, latency becomes a key concern.  Testing the interactions between
sets of micro services must be done carefully so as to reproduce the
characteristics of the real world.</p>

<h3>Tiered</h3>

<p>Tiered systems are a hybrid of monolithic and micro systems where work is
divided among a small number of tiers and each tier communicates only with the
tiers &ldquo;above&rdquo; and &ldquo;below&rdquo; it.  For lower volumes of work and smaller
numbers of contributors, tiers can achieve some of the benefits of micro
services while achieving the lower operational costs associated with monolithic
services.</p>

<p>There usually comes a point where for higher volumes of work and larger number
of contributors, the disadvantages of monolithic services appear for tiered
services.  At first, additional tiers can be added by factoring out distinct
work into its own tier.  However, some common functionality, such as service
discovery, security, and privacy, may be required across all the tiers,
requiring a micro service approach.</p>

<h2>Communication</h2>

<p>Systems are the building blocks of planetary scale computers.  And while a
single system is useful, a collection of systems is truly powerful.  To allow
one system to harness the utility of another system, the systems must interact
with one another.  We usually call a system that relies on another system the
<em>client</em> and the system that is relied on the <em>server</em>.  There
exist several common ways to handle communication between a client and a
server.</p>

<h3>Shared Libraries</h3>

<p>A shared library allows for one system to run within the same process and
address space as another system.  The shared library implements some
functionality which a system designer can utilize as if the shared library were
a part of the system itself.  A compiler either compiles and links an object
representing the shared library at compile time or the operating system links
dynamically a previously compiled object file at run time.</p>

<p>Shared libraries have the lowest communication overhead of any interaction
method between systems &mdash; a single function call.  Because an entire copy of
the shared system must be included in the system that is sharing it, shared
libraries can lead to binary bloat.  In addition, a system must be recompiled
and redeployed in order to use any new shared library features, causing shared
libraries to potentially increase the burden of system operators.</p>

<p>It is often helpful to use shared libraries in combination with other system
interaction methods.  For example, if a server returns some data to a client
that must be cached, instead of having every client implement its own method to
cache the data, a shared library for communicating with the server can be
provided that handles caching <em>and</em> server communication.</p>

<h3>Inter Process</h3>

<p>Inter process communication (IPC) relies on systems using the operating system
to communicate with one another.  Strictly speaking, any means of passing
information between processes is a valid form of inter process communication:
Files, pipes, shared memory, and sockets, to name a few.  A system that runs on
a device and provides some functionality to other systems on the same device
via inter process communication is sometimes called a &ldquo;sidecar.&rdquo;</p>

<p>Unlike shared libraries, inter process communication allows the sidecar process
to be updated independently of the process that uses its functionality.  In
addition, inter process communication has relatively low communication overhead
compared to other forms of system interaction.  A downside of inter process
communication is that each sidecar contends for compute, memory, storage,
network, and other resources on the devices it runs on.</p>

<p>It is usually best to have only a small number of widely used sidecars deployed
on devices.  Because sidecars limit system resources, if you choose to use
sidecar systems it is important for you to keep track of their resource usage
and decide on how much is too much.  You may also need to promote a sidecar to
a different form of interaction if its resource needs become too high.</p>

<h3>Networks</h3>

<p>Networks use protocols and relay devices to transfer information across vast
distances.  As networks provide the interconnect for planetary scale computers,
we will devote an entire chapter to them.  Here, though, we look at how systems
can interact with each other using networks.  We assume an Ethernet network
using a packet-based network protocol.</p>

<p>To communicate over a network, a server listens on a socket and waits for a
client to connect.  A client uses the address of a server to send packets of
information to the server.  If the server understands the packets, it can
construct its own packets to send back to the client.  This pattern repeats
until either the client, the server, or the network closes the connection.</p>

<p>Networks unlock the power of <em>distributed</em> system interaction.  When
connected to wide area networks and the Internet, systems can communicate
across the far reaches of the planet and even into space.  In addition to
latency (which can range from tens to hundreds of milliseconds across the
planet), a downside of network interaction is that the reliability of the
communication is limited by the reliability of the network and the protocol.</p>

<h3>Normalization</h3>

<p>Because a client may reside on a completely different operating environment
(hardware, architecture, operating system, and so on) from the server it wishes
to transfer information to, the client must normalize any information it sends
to the server.  For example, if we simply sent the bits of memory exactly as
they appeared on a client's device to a server, there is no guarantee that the
server would interpret the bits in the same way as the client.</p>

<p>To normalize the bits a client sends and a server receives, the client performs
a process called <em>serialization</em> on any information it sends and the
server performs an inverse process called <em>deserialization</em> to
reconstruct the information.  This process happens behind the scenes for the
client and server.</p>

<p>We can implement a <a href="/chapter/systems" class="sys" style="color:#E63946">normalization</a> system as a shared library.  We use the
<code>proc-macro</code>, <code>syn</code>, and <code>quote</code> crates to make it easier
to introspect on the Rust syntax tree and implement procedural macros to
serialize and deserialize simple <code>struct</code>s with <code>i32</code>,
<code>bool</code>, and <code>String</code> elements.  A full implementation would
handle other data types and escaped delimiters.</p>

<p><span class="sidenote"><strong><code>normalization/src/lib.rs</code></strong></span>
We start by defining the <code>Serializable</code> and <code>Deserializable</code> traits
to annotate <code>struct</code>s that support normalization.  The traits provide
functions that allow a <code>struct</code> to be converted to and from a
<code>String</code> representation.  We also define an <code>enum</code> for any errors
we encounter during serialization and deserialization.</p>

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
We then implement a serialization function for simple <code>struct</code>s.  The first thing
we do is define a function that takes a Rust <code>struct</code> and, for each of its fields,
depending on the type of the field, converts the value of the field to a string
representation.  We handle <code>String</code> types separately in order to escape special
characters.</p>

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

<p>At last, we get to the heart of the serialization routine.  We define a
procedural macro that implements the <code>serialize</code> function, which
converts the fields of a struct to a serialized string using the
<code>generate_serialization_for_type</code> function.  The Rust compiler calls
this function on structures derived from the <code>Serializable</code> trait.</p>

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

<p>The deserialization macro follows a similar pattern in reverse: it parses a
serialized <code>String</code>, splits it into key&ndash;value pairs (taking care to handle
escaped commas and colons), and reconstructs each field of the <code>struct</code>
according to its type.</p>

<p><span class="sidenote"><strong><code>normalization/src/lib.rs</code></strong></span>
To use the <a href="/chapter/systems" class="sys" style="color:#E63946">normalization</a> library, we simply <code>use</code> the traits
and declare a <code>struct</code> as <code>derive</code>d from them.</p>

<pre class="code-normalization"><code>pub use normalization_macros::{Serializable, Deserializable};

#[derive(Serializable, Deserializable)]
pub struct Sample {
    pub number: i32,
    pub flag: bool,
    pub text: String,
}</code></pre>

<p><span class="sidenote"><strong><code>normalization/tests/tests.rs</code></strong></span>
Later on, when we want to use the derived traits, we simply call the
<code>serialize</code> or <code>deserialize</code> function to normalize our
<code>struct</code> to and from a <code>String</code> representation.</p>

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

<p>Now that we can normalize the representation of <code>struct</code>s, we can share
them easily between systems.  We next turn to an abstract interface for how one
system can use another system to do work.</p>

<h3>Remote Procedure Calls</h3>

<p>Remote procedure calls (RPCs) provide an elegant wrapper around network
communication.  The goal of remote procedure calls is to provide a
function-like interface between systems that may be distributed across vast
distances.  This interface achieves the distribution benefits of network
communication with the programmability benefits of shared libraries.</p>

<p>Remote procedure calls were proposed by Andrew Nelson in 1981 and later
implemented by Birrell and Nelson in 1984.  At a
high level, they consist of an interface that specifies how to transfer
information between systems in a way that abstracts the fact that those systems
may reside on separate devices far away from each other.</p>

<p>The remote procedure call interface is translated into code that performs
serialization on the client, network transfer of a request from the client to a
server, deserialization on the server, work on the server, serialization on the
server, network transfer of a response from the server back to the client, and
deserialization on the client.</p>

<p>Remote procedure calls are a fundamental aspect of planetary scale computers.
If networks are the interconnect of planetary scale computers, remote procedure
calls are the bus protocol.  We can use the <a href="/chapter/systems" class="sys" style="color:#E63946">normalization</a> system to perform
serialization and deserialization between the client and the server.  We use the
<code>tokio</code> library to handle network communication since the details of
network access are not the focus of this book.</p>

<p><span class="sidenote"><strong><code>rpc/src/lib.rs</code></strong></span>
We begin by defining the interface between the remote procedure call client and
server.  The client sends a request to a server, specifying a number identifying
the procedure it would like the server to execute as well as a payload containing
data from the client.  The server processes the request and sends a response back
to the client containing a payload with the result of the request.</p>

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

<p>A note on procedure identifiers and payloads.  When you want to create a new
procedure, or update the payload structure of an existing procedure identifier,
it is important to select a new, unique procedure identifier.  This is because
payload changes are <em>not</em> backwards compatible between a client using
the new interface and a server implementing the old interface.</p>

<p>You also want to keep existing implementations of the old interface available
on the server until you are certain that no clients are sending requests to the
old interface anymore.  For this reason, you should always deploy a new version
of a server before deploying the corresponding new version of the client &mdash; old
client requests sent to new servers can still be serviced, but new client
requests sent to old servers will never be serviced.</p>

<p><span class="sidenote"><strong><code>rpc/src/server.rs</code></strong></span>
A remote procedure call server listens on a socket for a request, spawns a
thread to handle the request, and calls a request handler for the request.  The
request handler is implemented by the server.  Depending on the procedure the
client asks the server to perform, the server will execute different code.</p>

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

<p>Notice that each thread keeps its network socket open, waiting for additional
data (or for the socket to be closed).  This allows a client to send a stream
of requests to a server, without establishing network connections for each
request.  This reduces the latency and resources needed to handle the
additional requests, but if requests are long running there is a risk that we
could run out of sockets on the server.</p>

<p><span class="sidenote"><strong><code>rpc/src/client.rs</code></strong></span>
A remote procedure call client connects to a server at a particular address and
sends a request to execute the procedure the client identifies on the payload
the client provides.</p>

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

<h3>Client&ndash;Server</h3>

<p>The most common system architecture is the <em>client&ndash;server</em>
architecture, so named because it involves a client device communicating with a
server device.  In this architecture, the client sends a request to a server,
and the server sends a response back to the client.  A client could be a
command line interface, a stand alone binary, or a server that itself wishes to
send a request.</p>

<p>To solidify our understanding of the client&ndash;server architecture, and tie
together normalization and remote procedure calls, we will examine a simple
<a href="/chapter/systems" class="sys" style="color:#00B4D8">echo</a> client and server.  The <a href="/chapter/systems" class="sys" style="color:#00B4D8">echo</a> client sends a request whose
payload is a string that it would like the <a href="/chapter/systems" class="sys" style="color:#00B4D8">echo</a> server to repeat back to
the client.  The <a href="/chapter/systems" class="sys" style="color:#00B4D8">echo</a> server then inspects the string and returns it as
the payload of a response back to the client.</p>

<p><span class="sidenote"><strong><code>echo/src/lib.rs</code></strong></span>
We start by specifying the components of the <a href="/chapter/systems" class="sys" style="color:#00B4D8">echo</a> system in a shared library.
The library provides an identity to the procedure that the client can request
and that the server can perform.  It also defines the structure of the request
and response payloads.</p>

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
In the <a href="/chapter/systems" class="sys" style="color:#00B4D8">echo</a> server, we wait for a client to send a request.  When the server
receives a request, it calls a handler function to process the request.  The
handler checks to see if the requested procedure is the one that the server
recognizes, and if so, the server deserializes the request and places its message
in a response.</p>

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
The <a href="/chapter/systems" class="sys" style="color:#00B4D8">echo</a> client is responsible for constructing a request to send to an
<a href="/chapter/systems" class="sys" style="color:#00B4D8">echo</a> server and sending the request.  The client serializes the message,
specifies the identity of the procedure, sends the request to the server, awaits
the server's response, and prints the payload of the response.</p>

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

<p>The client&ndash;server architecture is a powerful abstraction with which to build
planetary scale computers.  It improves manageability by dividing the work that
needs to be done into separate entities that can be more easily managed than
one big entity.  It encourages modularity and reusability by exposing simple
interfaces with which to perform specific work.  It provides isolation and
scalability by allowing components of work to be run on a flexible number of
threads or devices.</p>

<h3>State</h3>

<p>The <a href="/chapter/systems" class="sys" style="color:#00B4D8">echo</a> system we just built was quite simplistic and did not need to
manage any state among requests.  However, many real systems (including the
ones we will look at next) often need to manage some state in order to perform
their work.  There is a straightforward way to share state safely among request
threads using atomic reference counters.  We refine our <a href="/chapter/systems" class="sys" style="color:#F4845F">rpc</a> shared
library with a function that allows us to share state among requests.</p>

<p><span class="sidenote"><strong><code>rpc/src/server.rs</code></strong></span>
The main difference compared to <code>start_server</code> is the use of a generic
type parameter <code>&lt;T&gt;</code> to represent the state that we wish to share among
requests.  We add a parameter to our function to pass the shared state and
modify the type declaration of our handler to also receive the shared state.
Before calling the handler, we clone the shared state so that the handler has
its own copy of the shared state.</p>

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

<h2>Transfer</h2>

<p>The voyage of a bit of information transferred from one system to another is a
fascinating journey.  It involves multiple stages, potentially many thousands
of miles, and avoiding hazards along the way that can prevent the successful
delivery.  We next trace the chronological flow of information from one system
to another and the concepts and systems that are involved in doing so.</p>

<h3>Discovery</h3>

<p>A client must know how to discover the location of a server it wishes to
communicate with.  However, a server can potentially move between devices or
there can be many devices that copies of the server run on &mdash; how can a client
find the right server given this flux?  When faced with a problem in
implementing a planetary scale computer, we typically turn to solving the
problem with a system.</p>

<p>A <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> system is responsible for taking a system that a client wants
to connect to, choosing a device for a client to connect to, and responding to
the client with the chosen device.  Thus, a discovery system takes as an input
some identifier for a system, and responds with the address of a device that
the client can connect to in order to communicate with the server.</p>

<p><span class="sidenote"><strong><code>discovery/src/lib.rs</code></strong></span>
We begin by defining the identifiers for the procedures that the <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a>
system implements.  A server uses the register procedure to submit the name of
the system it implements along with the address and port that the server is
listening on.  The query procedure allows a client to request a system by its
name and receive a response indicating the address and port of a server that
implements that system.</p>

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

<p>The <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> system uses shared state to keep track of the registered
systems.  The registry it maintains contains a mapping of system name to server
addresses as well as the time when an address last registered with the
<a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> system.  When an address for a system is requested, the
registry randomly selects and returns one.  A clean up function removes stale
registered addresses.</p>

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

<p>The <a href="/chapter/systems" class="sys" style="color:#F4845F">rpc</a> shared library makes it easy to implement the logic for the
<a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> service.  We define helper functions for processing
requests based on the request's procedure identifier and handling the register
and query procedures using the shared registry state.  Since the registry is
shared between multiple threads, a request handler function ensures that a
thread receives exclusive access to the registry before processing the request.</p>

<p>When a server becomes available, it registers its address with the <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a>
system as being capable of serving requests to its corresponding system
identifier.  Since servers can come and go, a system must occasionally
re-register itself with the <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> service.  Of course, even the
<a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> system can become unavailable.  When this happens, a server
retries connecting to the <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> system with an exponentially
increasing period, up to a maximum amount.</p>

<p>A critical question, however, is, how do services discover the <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a>
service in the first place?  One option is to maintain a set of addresses where
a client or server should always be able to reach a <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> service.
Other options include using the domain name service (DNS) or multicast to
locate the <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> service.</p>

<h3>Routing</h3>

<p>A server's availability can change over time and an available server discovered
by a client at one point in time can later become unavailable.  Thus, it is
helpful to build a layer of abstraction between a client's discovery of servers
and routing requests to those servers.  We call such a service the <a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a>
service and its primary job is to get requests from a client to a server efficiently
and reliably.</p>

<p>The <a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> service (which a client can initially discover using the
<a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> service) resolves a service identifier to an available server
for the service.  Unlike the <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> service, the <a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> service
also checks the health of the set of servers for a service and only routes
requests to available servers.  In addition, the <a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> service performs
connection pooling and load balancing across the set of servers for a service.</p>

<p><span class="sidenote"><strong><code>routing/src/lib.rs</code></strong></span>
We base our <a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> system on the concept of a connection pool.  Each
system that a client requests from the <a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> service has its own pool of
connections.  Each connection in that pool is an active network socket to a
server for the system.  Reusing sockets eliminates the need for the network
protocol latency and resource overheads associated with establishing connections
between a client and a server.</p>

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

<p>The <a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> system can run in two modes:  As a proxy system and as a
shared library.  The proxy system version uses a remote procedure call
interface to route requests between clients and servers, with the connection
pool maintained on the <a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> server.  The shared library version
performs all of the routing functionality on a client, including managing the
connection pool.</p>

<p><span class="sidenote"><strong><code>routing/src/lib.rs</code></strong></span>
The shared library performs similar functionality to the proxy system, but
within a structure that a client instantiates.</p>

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

<p>We can now update the <a href="/chapter/systems" class="sys" style="color:#00B4D8">echo</a> system to use the <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> and
<a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> systems to simplify the client&ndash;server communication process.</p>

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

<h3>Delivery</h3>

<p>After a client discovers and routes a request to a server, the request needs to
be delivered to the server process.  Because another request can arrive to a
server while a server is busy processing previous requests, it is useful
to maintain a queue of active requests.  New requests are placed in the queue and
delivered requests are removed from the queue.</p>

<p>The delivery queue can be maintained either in the client where the request is
sent or the server where the request is received.  Placing queues on the server
removes the need for managing flow control of requests to the server to avoid
congestion, but can lead to server queues filling up without good load
balancing.</p>

<h3>Concurrency</h3>

<p>A server can process multiple requests concurrently by using a request queue
and worker tasks.  The server accepts incoming connections and places requests
into a bounded channel, which provides natural back pressure.  A separate
worker task dequeues and processes requests:</p>

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

<h3>Timeouts</h3>

<p>Faults in a planetary scale computer can cause a server to take longer to
respond than a client expects &mdash; or to not respond at all.  A software bug
could cause a server to stop or slow down.  A hardware fault could render the
device a server is running on unreliable.  Or, a network or power fault could
take an entire building or geographic region offline.</p>

<p>To be resilient in the face of unpredictable bugs, faults, and outages, a
system can bound the amount of time spent trying to do work using timeouts.
Timeouts benefit clients by exposing the fact that a request is taking too
long.  The client can use that information to make an intelligent decision
about what to do next.  Timeouts also benefit servers by preventing long
standing requests from occupying precious queue slots and system resources.</p>

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

<h3>Retries</h3>

<p>When a request times out or encounters a transient error, a client can retry
the request.  However, retries must be implemented carefully.  If many clients
retry at the same time, they can create a &ldquo;thundering herd&rdquo; that overwhelms
the server.  To mitigate this, we add random <em>jitter</em> to the retry delay,
spreading out the retries over time:</p>

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

<p>As systems grow, they eventually need to span multiple geographic regions.
<a href="/chapter/global-distribution">Chapter 24: Global Distribution</a>
explores how to replicate a full system stack across regions while keeping
latency low and data consistent.</p>
"##
}

pub fn chapter_configuration() -> &'static str {
    r##"
<h1>Chapter 4: Configuration</h1>

<p><span class="newthought">Every distributed system</span> must adapt to its environment.
Server addresses change, feature flags toggle, rate limits adjust, and
operational parameters shift &mdash; often while the system is running.  Hardcoding
these values into binaries means recompiling and redeploying every time a
parameter changes.  A <a href="/chapter/configuration" class="sys" style="color:#3A86FF">configuration</a> service decouples runtime parameters
from the code that uses them, allowing operators to change system behavior
without touching source code or restarting processes.</p>

<p>At its core, a configuration service is a distributed key&ndash;value store with a
twist: clients don't just read values, they <em>watch</em> for changes.  When a
configuration value changes, systems that depend on it should learn of the
change quickly so they can adapt.  This makes configuration a foundational
service &mdash; nearly every other service in a planetary scale computer depends on
it.</p>

<h2>Interface</h2>

<p><span class="sidenote"><strong><code>configuration/src/lib.rs</code></strong></span>
The <a href="/chapter/configuration" class="sys" style="color:#3A86FF">configuration</a> service exposes five procedures through its RPC interface.
The basic operations &mdash; get, set, and delete &mdash; provide standard key&ndash;value
semantics.  The list operation supports prefix-based enumeration of keys,
useful for discovering all configuration under a namespace like
<code>storage.</code> or <code>caching.</code>.  The watch operation enables
clients to poll for changes to a specific key.</p>

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

<p>Each struct derives <code>Serializable</code> and <code>Deserializable</code> from
the <a href="/chapter/systems" class="sys" style="color:#E63946">normalization</a> system, enabling them to be transmitted over the
network via <a href="/chapter/systems" class="sys" style="color:#F4845F">rpc</a>.  The <code>ListResult</code> returns keys as a
comma-separated string &mdash; a deliberate simplicity that avoids the need for
list or array normalization.</p>

<h2>Implementation</h2>

<p><span class="sidenote"><strong><code>configuration/src/main.rs</code></strong></span>
The server maintains a <code>ConfigStore</code> containing an in-memory
<code>HashMap</code> and a broadcast channel for notifying watchers of changes.
The broadcast channel uses Tokio's <code>broadcast::channel</code>, which supports
multiple receivers and bounded buffering.</p>

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

<p>The handler for the set procedure inserts the key&ndash;value pair into the hash map
and then broadcasts the change through the watcher channel.  This means that
any system that is watching for changes to this key will be notified as soon as
the set completes:</p>

<pre class="code-configuration"><code>pub async fn set(payload: &amp;str, store: &amp;mut ConfigStore) -&gt; Response {
    let args = SetArgs::deserialize(payload)
        .expect("Failed to deserialize payload");
    store.data.insert(args.key.clone(), args.value.clone());
    let _ = store.watchers.send((args.key, args.value));
    Response { payload: "OK".to_string() }
}</code></pre>

<p>The list handler filters keys by prefix, a pattern that allows hierarchical
organization of configuration.  For example, a storage service might store its
settings under keys like <code>storage.compaction_interval</code> and
<code>storage.max_size</code>, and retrieve all storage-related keys by listing
with the prefix <code>storage.</code>.</p>

<pre class="code-configuration"><code>pub async fn list(payload: &amp;str, store: &amp;mut ConfigStore) -&gt; Response {
    let args = ListArgs::deserialize(payload)
        .expect("Failed to deserialize payload");
    let keys: Vec&lt;String&gt; = store.data.keys()
        .filter(|k| k.starts_with(&amp;args.prefix))
        .cloned()
        .collect();
    let result = ListResult { keys: keys.join(",") };
    Response { payload: result.serialize() }
}</code></pre>

<p>The watch procedure returns the current value for a key, allowing clients to
poll for changes.  A more sophisticated implementation might hold the connection
open and push changes, but polling keeps the RPC interface simple and stateless.</p>

<h2>Design Discussion</h2>

<p>Several design trade-offs are worth noting in this implementation.  The
in-memory store provides fast reads and writes but does not survive restarts.
A production configuration service would persist its data to durable storage
&mdash; in fact, it could use the <a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a> service we will examine later.</p>

<p>The broadcast channel for watchers is a pragmatic choice.  It decouples
the notification mechanism from the storage mechanism, and its bounded buffer
prevents a slow consumer from blocking writers.  However, if the buffer fills
up, late-arriving watchers will miss events &mdash; a trade-off that favors
availability over completeness.</p>

<p>The prefix-based listing pattern is a common technique in configuration
systems.  It enables namespacing (grouping related keys) without introducing
a more complex data model like directories or trees.  Systems like etcd and
Consul use similar prefix-based enumeration in their key&ndash;value stores.</p>

<p>Configuration is often the first service to start and the last to stop
in a distributed system.  Because almost every other service depends on
configuration, its availability is critical.  In a production environment, the
configuration service would be replicated across multiple servers using a
consensus protocol to ensure that configuration data is always available even
when individual servers fail.</p>
"##
}

pub fn chapter_caching() -> &'static str {
    r##"
<h1>Chapter 7: Caching</h1>

<p><span class="newthought">Accessing data</span> from its source &mdash; whether a database,
a remote service, or persistent storage &mdash; takes time.  Network round trips,
disk reads, and computation all contribute to latency.  A <a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a> service
stores recently or frequently accessed data in memory so that subsequent
requests for the same data can be served faster.  Caching is one of the most
effective techniques for improving the performance and reducing the cost of
distributed systems.</p>

<p>The fundamental insight behind caching is the principle of <em>locality</em>:
data that was accessed recently is likely to be accessed again soon (temporal
locality), and data near recently accessed data is also likely to be accessed
(spatial locality).  A well-designed cache exploits these patterns to serve a
high percentage of requests from fast in-memory storage rather than slower
backing stores.</p>

<h2>Interface</h2>

<p><span class="sidenote"><strong><code>caching/src/lib.rs</code></strong></span>
The <a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a> service provides four procedures.  Get and set form the core
read/write interface.  Unlike the <a href="/chapter/configuration" class="sys" style="color:#3A86FF">configuration</a> service, the set
operation accepts a time-to-live (TTL) parameter that controls how long the
entry remains valid.  A delete operation allows explicit cache invalidation,
and a stats operation exposes operational metrics.</p>

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

<p>The <code>GetResult</code> includes a <code>hit</code> field (1 for a cache hit, 0 for
a miss), allowing clients to distinguish between a missing key and an empty
value.  This is important for avoiding cache stampedes, where many clients
simultaneously fetch the same missing key from the backing store.</p>

<h2>Implementation</h2>

<p><span class="sidenote"><strong><code>caching/src/main.rs</code></strong></span>
The cache server maintains an in-memory data structure that combines a
<code>HashMap</code> for fast key lookups with a <code>VecDeque</code> for
tracking access order.  Each entry stores a value and an expiration time.
This implements an LRU (Least Recently Used) eviction policy with
time-based expiration.</p>

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

<p>The get operation checks the expiration time before returning a value.
If the entry has expired, it is removed and treated as a miss.  On a
hit, the key is moved to the front of the LRU queue, marking it as
recently accessed:</p>

<pre class="code-caching"><code>fn get(&amp;mut self, key: &amp;str) -&gt; Option&lt;String&gt; {
    if let Some(entry) = self.entries.get(key) {
        if entry.expires_at &lt; Instant::now() {
            self.entries.remove(key);
            self.lru_order.retain(|k| k != key);
            self.misses += 1;
            return None;
        }
        let value = entry.value.clone();
        // Move to front of LRU
        self.lru_order.retain(|k| k != key);
        self.lru_order.push_front(key.to_string());
        self.hits += 1;
        Some(value)
    } else {
        self.misses += 1;
        None
    }
}</code></pre>

<p>The set operation handles capacity management.  When the cache is full and a
new entry needs to be inserted, the least recently used entry (at the back of
the deque) is evicted.  This ensures the cache never exceeds its memory budget
while keeping the most useful entries in memory:</p>

<pre class="code-caching"><code>fn set(&amp;mut self, key: String, value: String, ttl_secs: i32) {
    let ttl = if ttl_secs &gt; 0 {
        Duration::from_secs(ttl_secs as u64)
    } else {
        Duration::from_secs(3600) // Default 1 hour TTL
    };

    let entry = CacheEntry {
        value,
        expires_at: Instant::now() + ttl,
    };

    self.lru_order.retain(|k| k != &amp;key);

    // Evict if at capacity
    while self.entries.len() &gt;= self.max_capacity {
        if let Some(evicted_key) = self.lru_order.pop_back() {
            self.entries.remove(&amp;evicted_key);
        } else {
            break;
        }
    }

    self.entries.insert(key.clone(), entry);
    self.lru_order.push_front(key);
}</code></pre>

<p>A background task runs periodically to clean up expired entries.  This prevents
the cache from filling up with stale data that might never be accessed again:</p>

<pre class="code-caching"><code>// Background cleanup task for expired entries
let cleanup_cache = Arc::clone(&amp;cache);
tokio::spawn(async move {
    loop {
        sleep(CLEANUP_INTERVAL).await;
        cleanup_cache.lock().await.cleanup_expired();
    }
});</code></pre>

<h2>Design Discussion</h2>

<p>The LRU eviction policy is one of several strategies for managing cache
capacity.  LRU works well when access patterns exhibit temporal locality &mdash;
recently accessed items are likely to be accessed again.  Other strategies
include LFU (Least Frequently Used), which favors items accessed many times
even if not recently, and random eviction, which is simpler but less
effective.</p>

<p>TTL-based expiration provides a safety net against serving stale data.  Without
TTLs, a cached value could persist indefinitely even after the backing store
has been updated.  The choice of TTL represents a trade-off: shorter TTLs
reduce staleness but increase load on the backing store; longer TTLs improve
hit rates but risk serving outdated data.</p>

<p>The hit/miss tracking exposed through the stats procedure is invaluable for
operations.  A healthy cache should have a high hit rate (often 90% or above).
A sudden drop in hit rate might indicate a change in access patterns, a
configuration error, or a capacity problem.  These metrics are the kind of
signals that the <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service (discussed later) can track and alert
on.</p>

<p>In a production environment, caching is often deployed as a hierarchy: a local
in-process cache for the hottest data, a shared cache service (like this one)
for less-hot data, and the backing store for everything else.  Each layer
trades off latency, capacity, and consistency differently.</p>
"##
}

pub fn chapter_storage() -> &'static str {
    r##"
<h1>Chapter 8: Storage</h1>

<p><span class="newthought">All data</span> in a running system lives in memory &mdash; fast,
volatile, and finite.  When a process crashes or a server loses power, that
data vanishes.  A <a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a> service provides <em>durable</em> data persistence:
the guarantee that data written to the service will survive process restarts,
hardware failures, and power outages.  Storage is the bedrock on which
stateful systems are built.</p>

<p>The design of a storage engine involves balancing competing demands.  Reads
should be fast (ideally served from memory).  Writes should be durable (safely
on disk before acknowledging).  Space should be used efficiently (old data
compacted away).  And the whole system should recover quickly after a crash.
Our implementation addresses each of these concerns with a classic technique:
the write-ahead log.</p>

<h2>Interface</h2>

<p><span class="sidenote"><strong><code>storage/src/lib.rs</code></strong></span>
The <a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a> service provides four procedures.  Get and put form the basic
key&ndash;value interface for reading and writing data.  Delete removes a key, and
scan retrieves multiple entries matching a prefix, useful for range queries
and enumeration.</p>

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

<p>Like the <a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a> service, the <code>GetResult</code> includes a
<code>found</code> field to distinguish between a missing key and an empty value.
The <code>ScanResult</code> returns entries as semicolon-delimited
<code>key=value</code> pairs, keeping the serialization format simple while
supporting multi-entry responses.</p>

<h2>Implementation</h2>

<p><span class="sidenote"><strong><code>storage/src/engine.rs</code></strong></span>
The heart of the <a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a> service is its <code>StorageEngine</code>.  The engine
maintains an in-memory <code>HashMap</code> for fast reads, a write-ahead log (WAL)
for durability, and a snapshot mechanism for efficient recovery.  This
combination is a classic pattern found in databases from SQLite to PostgreSQL.</p>

<pre class="code-storage"><code>pub struct StorageEngine {
    data: HashMap&lt;String, String&gt;,
    wal_path: PathBuf,
    snapshot_path: PathBuf,
    operations_since_snapshot: usize,
}

const COMPACTION_THRESHOLD: usize = 1000;</code></pre>

<p>The write-ahead log is the key to durability.  Every write operation &mdash;
whether a put or a delete &mdash; is first appended to the WAL file on disk before
the in-memory data structure is updated.  This ensures that even if the process
crashes immediately after acknowledging a write, the operation can be recovered
from the log:</p>

<pre class="code-storage"><code>fn append_wal(&amp;mut self, entry: &amp;str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&amp;self.wal_path)
        .expect("Failed to open WAL");
    writeln!(file, "{}", entry).expect("Failed to write to WAL");
    self.operations_since_snapshot += 1;
}

pub fn put(&amp;mut self, key: String, value: String) {
    self.append_wal(&amp;format!("PUT {}={}", key, value));
    self.data.insert(key, value);
}

pub fn delete(&amp;mut self, key: &amp;str) {
    self.append_wal(&amp;format!("DELETE {}", key));
    self.data.remove(key);
}</code></pre>

<p>Recovery reconstructs the in-memory state by first loading the most recent
snapshot (if one exists) and then replaying any WAL entries that were written
after the snapshot.  This two-phase recovery ensures both speed (loading a
snapshot is faster than replaying the entire history) and completeness (the WAL
captures any operations since the last snapshot):</p>

<pre class="code-storage"><code>fn recover(&amp;mut self) {
    // First, load snapshot if it exists
    if self.snapshot_path.exists() {
        if let Ok(file) = File::open(&amp;self.snapshot_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    let parts: Vec&lt;&amp;str&gt; = line.splitn(2, '=').collect();
                    if parts.len() == 2 {
                        self.data.insert(
                            parts[0].to_string(),
                            parts[1].to_string(),
                        );
                    }
                }
            }
        }
    }

    // Then, replay WAL on top
    if self.wal_path.exists() {
        if let Ok(file) = File::open(&amp;self.wal_path) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    self.apply_wal_entry(&amp;line);
                }
            }
        }
    }
}</code></pre>

<p>Over time, the WAL grows as operations accumulate.  Compaction solves this by
writing a snapshot of the current in-memory state and then truncating the WAL.
A background task triggers compaction after a threshold number of operations:</p>

<pre class="code-storage"><code>pub fn compact(&amp;mut self) {
    if self.operations_since_snapshot &lt; COMPACTION_THRESHOLD {
        return;
    }

    // Write snapshot
    let mut file = File::create(&amp;self.snapshot_path)
        .expect("Failed to create snapshot");
    for (key, value) in &amp;self.data {
        writeln!(file, "{}={}", key, value)
            .expect("Failed to write snapshot");
    }

    // Truncate WAL
    File::create(&amp;self.wal_path).expect("Failed to truncate WAL");
    self.operations_since_snapshot = 0;
}</code></pre>

<p>The scan operation supports prefix-based range queries, sorting results
lexicographically and respecting a limit parameter to prevent unbounded
responses:</p>

<pre class="code-storage"><code>pub fn scan(&amp;self, prefix: &amp;str, limit: i32) -&gt; Vec&lt;(String, String)&gt; {
    let mut results: Vec&lt;(String, String)&gt; = self.data.iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    results.sort_by(|a, b| a.0.cmp(&amp;b.0));
    if limit &gt; 0 {
        results.truncate(limit as usize);
    }
    results
}</code></pre>

<h2>Design Discussion</h2>

<p>The write-ahead log pattern provides a strong durability guarantee: data is on
disk before the write is acknowledged.  However, our implementation writes to a
single file on a single disk.  A production storage service would replicate
data across multiple servers to survive disk and machine failures.  Techniques
like chain replication or consensus-based replication (using a protocol like
Raft or Paxos) can provide this.</p>

<p>The in-memory <code>HashMap</code> provides O(1) reads but limits the total data
size to available memory.  For larger datasets, storage engines use on-disk
data structures like LSM trees (Log-Structured Merge trees) or B-trees that
can store far more data than fits in memory while still providing fast reads
through caching and indexing.</p>

<p>Compaction is a form of garbage collection for the WAL.  Without it, the log
would grow without bound, making recovery slower over time.  The compaction
threshold of 1,000 operations represents a trade-off: more frequent compaction
keeps the WAL small but consumes more I/O; less frequent compaction reduces
I/O but risks longer recovery times.</p>

<p>The scan operation's prefix-based filtering is a versatile primitive.  It
enables patterns like namespacing (all keys for a user under
<code>user:{id}:</code>), range queries (all entries in a time window), and
enumeration (listing all keys in a collection).  Our highlight system uses
this pattern to store per-user, per-page highlights under keys like
<code>hl:{user_id}:{page_slug}</code>.</p>
"##
}

pub fn chapter_design() -> &'static str {
    r##"
<h1>Chapter 2: Design</h1>

<p><span class="newthought">Before writing</span> a single line of code, a systems engineer must
answer a fundamental question: <em>what problem are we solving?</em>  The design
process is where ambiguity is transformed into clarity, where requirements are
distilled into interfaces, and where trade-offs are made explicit before they
become expensive to change.  Good design is the foundation upon which reliable
systems are built.</p>

<p>In the context of planetary scale computing, design takes on additional
dimensions.  A system that works on a single machine may fail spectacularly
when distributed across thousands of servers.  A design that is elegant for
ten users may collapse under ten million.  The design process must anticipate
scale, failure, and evolution from the very beginning.</p>

<h2>The Problem Statement</h2>

<p>Every system begins with a problem.  The problem statement defines what the
system must accomplish, who it serves, and what constraints it operates under.
A well-written problem statement is specific enough to guide design decisions
but general enough to avoid premature commitment to implementation details.</p>

<p>Consider the systems we are building in this book.  The <a href="/chapter/systems" class="sys" style="color:#00B4D8">echo</a> system's
problem statement is simple: given a message from a client, return that same
message.  The <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> system's problem is more nuanced: given a system
name, return the address of a healthy server that implements that system,
even as servers come and go.  The <a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a> system's problem adds another
dimension: persist data durably across process restarts and hardware failures.</p>

<p>Each problem statement implicitly defines the system's <em>scope</em>.  A
discovery system discovers servers &mdash; it does not route requests to them.
A storage system stores data &mdash; it does not cache it in memory for fast
access.  Keeping scope tight is one of the most important design principles.
When a system tries to do too much, it becomes harder to understand, harder
to test, and harder to operate.</p>

<h2>The Design Document</h2>

<p>A design document translates a problem statement into a concrete plan.  It
typically includes four sections: the interface the system will expose, the
data structures it will maintain, the algorithms it will use, and the
trade-offs it accepts.  The design document is a contract between the
designer and the implementer &mdash; even when they are the same person.</p>

<p>The interface section defines how other systems will interact with this one.
In our systems, interfaces are defined as RPC procedures with typed arguments
and results.  The <a href="/chapter/configuration" class="sys" style="color:#3A86FF">configuration</a> service, for example, defines five
procedures: get, set, delete, list, and watch.  Each procedure has a unique
identifier, a request structure, and a response structure.  This interface is
specified in the shared library (<code>lib.rs</code>) before any server code is
written.</p>

<p>The data structures section describes what state the system maintains and how
it is organized.  The <a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a> service maintains a hash map for fast
lookups and a deque for LRU ordering.  The <a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a> service maintains
an in-memory hash map, a write-ahead log, and periodic snapshots.  These
choices directly affect the system's performance characteristics.</p>

<p>The algorithms section describes how the system processes requests.  For many
of our services, this is straightforward: deserialize the request, perform an
operation on the data structure, serialize the response.  For more complex
systems like <a href="/chapter/consensus" class="sys" style="color:#06D6A0">consensus</a>, the algorithm section describes election
protocols, log replication, and state machine application.</p>

<p>The trade-offs section is perhaps the most important.  Every design decision
involves trade-offs, and making them explicit prevents surprises later.  The
<a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a> service trades memory for speed.  The <a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a> service
trades write latency (writing to the WAL before acknowledging) for durability.
The <a href="/chapter/configuration" class="sys" style="color:#3A86FF">configuration</a> service trades consistency (eventual notification via
broadcast channels) for availability.</p>

<h2>Interface-First Design</h2>

<p>A pattern that appears throughout our systems is <em>interface-first design</em>.
The shared library (<code>lib.rs</code>) defines the interface before the server
(<code>main.rs</code>) implements it.  This has several advantages.</p>

<p>First, it forces the designer to think about the system from the client's
perspective.  What operations does a client need?  What data does it send
and receive?  This outside-in thinking produces cleaner interfaces than
starting from the implementation and working outward.</p>

<p>Second, it enables parallel development.  Once the interface is defined,
clients can be written against it (using stubs or mocks) while the server
is being implemented.  In a large organization, different teams can work
on the client and server simultaneously.</p>

<p>Third, it provides a natural versioning boundary.  When the interface
changes, the procedure identifier changes, and both old and new versions
can coexist during the transition.  This is critical for systems that
cannot tolerate downtime during upgrades.</p>

<h2>Resources</h2>

<p>Every system consumes resources: CPU, memory, storage, network bandwidth,
and file descriptors, among others.  A good design accounts for resource
usage and establishes budgets.  The <a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a> service has a
<code>MAX_CAPACITY</code> that bounds memory usage.  The <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a>
service has a <code>MAX_METRIC_WINDOW</code> that bounds the number of data
points stored per metric.</p>

<p>Resource budgets interact with each other.  A system that uses less memory
might need more CPU (for compression).  A system that uses less network
bandwidth might need more storage (for batching).  Understanding these
interactions is a key part of the design process.</p>

<h2>Management</h2>

<p>A system that cannot be managed cannot be operated at scale.  Design must
include management concerns from the beginning: how will the system be
configured?  How will its health be monitored?  How will it be deployed
and updated?  How will operators debug problems?</p>

<p>Our systems address these concerns through integration with the
<a href="/chapter/configuration" class="sys" style="color:#3A86FF">configuration</a>, <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a>, and <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> services.
Each service registers itself with discovery on startup, reports metrics
to monitoring, and reads runtime parameters from configuration.  This
management infrastructure is as important as the business logic itself.</p>

<p>The design process is iterative.  A first design is rarely the final design.
As implementation reveals unforeseen challenges, as testing exposes edge
cases, and as operation surfaces real-world behavior, the design evolves.
The key is to make the design explicit and revisable, not to make it
perfect on the first attempt.</p>
"##
}

pub fn chapter_consensus() -> &'static str {
    r##"
<h1>Chapter 3: Consensus</h1>

<p><span class="newthought">In a distributed system,</span> multiple servers must often agree on
a shared state &mdash; who is the leader, what entries are in the log, whether a
transaction should commit.  Reaching this agreement in the presence of
failures is the problem of <em>consensus</em>.  Consensus is one of the most
fundamental and challenging problems in distributed computing, and getting it
right is essential for building reliable planetary scale computers.</p>

<p>The difficulty arises from the nature of distributed systems: messages can be
delayed, reordered, or lost; servers can crash and restart; and there is no
global clock.  Despite these challenges, consensus algorithms allow a group
of servers (called an <em>ensemble</em> or <em>cluster</em>) to behave as a single
coherent unit, even when some members fail.</p>

<h2>Quorum-Based Consensus</h2>

<p>The most widely used approach to consensus is quorum-based voting.  The key
insight is that if a majority of servers agree on a decision, then any two
majorities must overlap in at least one server.  This overlap ensures that
decisions are not lost even when some servers fail.  A system of five servers
can tolerate two failures; a system of three can tolerate one.</p>

<p>Our implementation follows the Raft consensus algorithm, designed by Diego
Ongaro and John Ousterhout for understandability.  Raft divides the consensus
problem into three sub-problems: <em>leader election</em> (choosing a single
leader), <em>log replication</em> (the leader distributing entries to followers),
and <em>safety</em> (ensuring that committed entries are never lost).</p>

<h3>Roles and State</h3>

<p><span class="sidenote"><strong><code>consensus/src/member.rs</code></strong></span>
Every member of the ensemble is in one of three roles at any given time:
leader, follower, or candidate.  The leader handles all client requests and
replicates log entries to followers.  Followers passively accept entries from
the leader.  A candidate is a follower that is attempting to become the new
leader.</p>

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

<p>The <code>Member</code> struct contains all the state a consensus participant needs.
The <code>term</code> is a logical clock that increases monotonically with each
election &mdash; it allows members to detect stale leaders.  The <code>log</code> is an
ordered sequence of entries that all members must agree upon.  The
<code>commit_index</code> tracks how far into the log has been safely replicated to
a majority, and <code>last_applied</code> tracks how far the state machine has
consumed.  The <code>state_machine</code> is the application-specific logic that
processes committed entries.</p>

<p><span class="sidenote"><strong><code>consensus/src/lib.rs</code></strong></span>
Each log entry records the term in which it was created, an action identifier,
and a payload.  The <code>StateMachine</code> trait defines how the application
processes committed entries:</p>

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

<h3>The Main Loop</h3>

<p>A member's lifecycle is a loop that switches behavior based on its current
role.  On startup, a member joins the ensemble by contacting the existing
leader (discovered via the <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> service) or, if no leader exists,
initializing a new ensemble as the first leader:</p>

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

<h3>Leader Election</h3>

<p><span class="sidenote"><strong><code>consensus/src/follower.rs</code></strong></span>
A follower monitors heartbeats from the leader.  If no heartbeat arrives
within a randomized timeout (1500&ndash;3000 milliseconds), the follower assumes
the leader has failed and transitions to the candidate role to start an
election:</p>

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

<p>The randomized timeout is critical.  If all followers used the same timeout,
they would all start elections simultaneously, splitting votes and preventing
any candidate from winning.  Randomization ensures that in most cases, a
single follower times out first and wins the election before others start
their own.</p>

<p><span class="sidenote"><strong><code>consensus/src/candidate.rs</code></strong></span>
A candidate increments its term (to distinguish this election from previous
ones), votes for itself, and requests votes from all peers.  If it receives
votes from a majority, it becomes the leader:</p>

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

    if votes &gt;= needed_votes {
        *self.role.lock().await = Role::Leader;
    }
}</code></pre>

<p>The majority requirement (<code>(peers.len() / 2) + 1</code>) is the heart of
quorum-based consensus.  In a five-member ensemble, three votes are needed.
This guarantees that two concurrent elections cannot both succeed, because
their majorities would need to overlap.</p>

<h3>Log Replication</h3>

<p><span class="sidenote"><strong><code>consensus/src/leader.rs</code></strong></span>
Once elected, the leader registers with the <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> service so that
clients and new members can find it.  It then sends periodic heartbeats to
followers by notifying them of the current peer list and term.  These
heartbeats serve double duty: they prevent followers from starting
unnecessary elections and they keep the ensemble's peer lists synchronized:</p>

<pre class="code-consensus"><code>pub async fn run_as_leader(&amp;self) {
    let address = self.address.lock().await.clone();
    discovery::register(ENSEMBLE_NAME.to_string(), address.to_string());

    let heartbeat_interval = Duration::from_millis(1500);
    let mut interval = time::interval(heartbeat_interval);
    loop {
        interval.tick().await;
        self.notify_members_of_peers().await;
    }
}</code></pre>

<p>When a client submits a new entry, the leader appends it to its own log and
then replicates the entry to all followers.  The entry is only considered
committed once a majority of members have acknowledged it.  If consensus
cannot be reached, the entry is rolled back:</p>

<pre class="code-consensus"><code>pub async fn handle_append_entry(&amp;self, args: AppendEntryArgs) -&gt; String {
    if *self.role.lock().await != Role::Leader {
        return "Member is not leader and cannot append entries.".to_string();
    }

    let entry = LogEntry {
        term: *self.term.lock().await,
        action: args.action,
        payload: args.payload,
    };

    let mut log = self.log.write().await;
    log.push(entry.clone());

    match self.replicate_entry_to_followers(&amp;entry).await {
        Ok(_) =&gt; {
            let mut commit_index = self.commit_index.lock().await;
            *commit_index = log.len();
            format!("Appended entry: {:?}", entry)
        }
        Err(e) =&gt; {
            log.pop(); // Roll back
            e
        }
    }
}</code></pre>

<p>The replication process contacts each follower and counts acknowledgments.
The entry is committed only if a majority respond positively:</p>

<pre class="code-consensus"><code>async fn replicate_entry_to_followers(
    &amp;self, entry: &amp;LogEntry,
) -&gt; Result&lt;(), String&gt; {
    let mut acks = 1; // Self-vote
    let peers = self.peers.read().await;
    for peer in peers.iter() {
        if self.replicate_entry(peer, entry).await {
            acks += 1;
        }
    }

    let majority = (peers.len() / 2) + 1;
    if acks &gt;= majority {
        Ok(())
    } else {
        Err("Failed to achieve consensus".to_string())
    }
}</code></pre>

<p><span class="sidenote"><strong><code>consensus/src/follower.rs</code></strong></span>
When a follower receives a replicated entry, it appends the entry to its log,
updates its commit index, and applies any newly committed entries to its local
state machine:</p>

<pre class="code-consensus"><code>pub async fn handle_replicate_entry(
    &amp;self, args: ReplicateEntryArgs,
) -&gt; String {
    if *self.role.lock().await != Role::Follower {
        return "Member is not follower".to_string();
    }

    let mut log = self.log.write().await;
    log.push(args.entry);

    *self.commit_index.lock().await = args.commit_index;

    let last_applied = *self.last_applied.lock().await;
    for entry in last_applied..args.commit_index {
        let log_entry = self.log.read().await[entry].clone();
        self.state_machine.lock().await
            .apply(log_entry.action, log_entry.payload).await;
    }

    ReplicateEntryResponse { ack: true }.serialize()
}</code></pre>

<h2>Application: Distributed Locking</h2>

<p>Consensus becomes truly powerful when applied to a specific domain.  A
distributed lock service demonstrates this: multiple clients need to
coordinate exclusive access to shared resources, and the lock state must
be consistent across all servers even when failures occur.</p>

<p><span class="sidenote"><strong><code>locking_v0/src/main.rs</code></strong></span>
The locking service builds on the consensus protocol.  Each server maintains
a map of locks, and lock operations (acquire and release) are recorded as
log entries.  Because the log is replicated through consensus, all servers
agree on which locks are held and by whom:</p>

<pre class="code-consensus"><code>struct Lock {
    is_locked: bool,
    owner: Option&lt;String&gt;,
}

struct Server {
    term: Arc&lt;Mutex&lt;i32&gt;&gt;,
    role: Arc&lt;Mutex&lt;Role&gt;&gt;,
    log: Arc&lt;RwLock&lt;Vec&lt;LogEntry&gt;&gt;&gt;,
    peers: Arc&lt;RwLock&lt;Vec&lt;String&gt;&gt;&gt;,
    locks: Arc&lt;Mutex&lt;HashMap&lt;String, Lock&gt;&gt;&gt;,
    commit_index: Arc&lt;Mutex&lt;usize&gt;&gt;,
    last_applied: Arc&lt;Mutex&lt;usize&gt;&gt;,
    // ...
}</code></pre>

<p>When a client requests a lock, the server checks whether it is available and,
if so, appends an acquire entry to the consensus log.  The entry is only
applied after it has been replicated to a majority of servers:</p>

<pre class="code-consensus"><code>async fn handle_acquire(&amp;self, args: AcquireArgs) -&gt; String {
    let lock_id = &amp;args.lock_id;
    let mut locks = self.locks.lock().await;
    let lock = locks.entry(lock_id.to_string()).or_insert(Lock {
        is_locked: false,
        owner: None,
    });

    if lock.is_locked {
        return "Lock is already acquired".to_string();
    } else {
        lock.is_locked = true;
        lock.owner = Some(args.owner.to_string());
        let entry = LogEntry {
            term: *self.term.lock().await,
            lock_id: lock_id.to_string(),
            action: LOCK_ACQUIRE,
            member_id: self.address.lock().await.to_string(),
        };
        self.log.write().await.push(entry);
        return "Lock acquired".to_string();
    }
}</code></pre>

<p>New members joining the ensemble must catch up on the current lock state
before they can participate.  They do this by fetching the committed log
from the leader and replaying it locally:</p>

<pre class="code-consensus"><code>async fn join_ensemble(&amp;self) {
    *self.is_full_member.lock().await = false;
    self.catch_up_log().await;
    let address = self.address.lock().await;
    let _ = self.request_full_membership(address.to_string()).await;
}</code></pre>

<h2>Design Discussion</h2>

<p>Consensus algorithms like Raft make strong guarantees: once an entry is
committed, it will not be lost even if a minority of servers fail.  These
guarantees come at a cost.  Every write must be replicated to a majority
of servers before it can be acknowledged, adding latency proportional to
the slowest server in the majority.  Read operations must also be
linearized (either by routing through the leader or using read leases)
to prevent stale reads.</p>

<p>The choice of timeouts is critical for system behavior.  The election
timeout must be long enough that normal heartbeat delays do not trigger
spurious elections, but short enough that actual leader failures are
detected promptly.  Our implementation uses 1500&ndash;3000 milliseconds for
elections and 1500 milliseconds for heartbeats, suitable for a local
network.  A geographically distributed system might use timeouts of
several seconds.</p>

<p>The <code>StateMachine</code> trait provides a clean separation between the
consensus protocol and the application logic.  Any application that can
express its state changes as a sequence of <code>(action, payload)</code> pairs
can be built on top of this consensus implementation: key-value stores,
configuration services, lock managers, and coordination services.</p>

<p>In production, consensus-based systems like etcd, ZooKeeper, and Consul
form the backbone of distributed coordination.  They provide the
primitives &mdash; leader election, distributed locking, configuration
management, and service discovery &mdash; upon which larger systems are built.</p>
"##
}

pub fn chapter_discovery() -> &'static str {
    r##"
<h1>Chapter 5: Discovery</h1>

<p><span class="newthought">In a planetary scale computer,</span> servers are ephemeral.  They
start, stop, move between machines, and scale up and down in response to
load.  A client that hardcodes the address of a server it depends on will
break the moment that server moves.  The <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> service solves this
problem by maintaining a dynamic registry of which servers are available
and where they can be found.</p>

<p>We introduced discovery briefly in Chapter 1.  Here we examine the
implementation in depth: the registry data structure, the mechanisms for
keeping it current, and the design decisions that make discovery reliable
enough to serve as the foundation for all other service communication.</p>

<h2>Interface</h2>

<p><span class="sidenote"><strong><code>discovery/src/lib.rs</code></strong></span>
The discovery interface is deliberately minimal: two procedures.  Register
allows a server to announce itself, and query allows a client to find a
server.  This simplicity is intentional &mdash; a discovery service that is
complex is a discovery service that fails in complex ways:</p>

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

<h2>The Registry</h2>

<p><span class="sidenote"><strong><code>discovery/src/main.rs</code></strong></span>
The heart of the discovery service is the <code>Registry</code> data structure.
It maintains two maps: one from system names to lists of server addresses,
and another tracking when each address last checked in.  This dual-map
design separates the logical concern (which servers implement which systems)
from the operational concern (which servers are still alive):</p>

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

<p>The <code>register</code> method handles both initial registration and re-registration.
When an address that is already known re-registers, only the timestamp is
updated &mdash; the address is not duplicated in the registry.  This idempotency
is important because servers register periodically, and duplicating entries
would bias the random selection in <code>get_address</code>.</p>

<p>The <code>get_address</code> method uses random selection to distribute load across
all servers implementing a system.  This provides a simple form of load
balancing: if three servers implement the <a href="/chapter/systems" class="sys" style="color:#00B4D8">echo</a> system, each query has
roughly a one-in-three chance of returning each server's address.</p>

<h2>Stale Entry Cleanup</h2>

<p>Servers can fail without deregistering &mdash; a crash, a network partition, or
a hardware failure will leave stale entries in the registry.  The cleanup
mechanism removes addresses that have not sent a heartbeat within a
configurable duration:</p>

<pre class="code-discovery"><code>fn cleanup_stale(&amp;mut self) {
    let now = Instant::now();
    let stale_addresses: HashSet&lt;_&gt; = self.last_ping.iter()
        .filter(|&amp;(_, time)|
            now.duration_since(*time) &gt; CLEANUP_DURATION)
        .map(|(address, _)| address.clone())
        .collect();

    for address in stale_addresses {
        self.last_ping.remove(&amp;address);
        self.registry.retain(|_, v| {
            v.retain(|a| a != &amp;address);
            !v.is_empty()
        });
    }
}</code></pre>

<p>The cleanup runs as a background task, periodically scanning the registry
for entries whose last ping timestamp exceeds the threshold.  When a stale
address is found, it is removed from both maps.  If removing the address
leaves a system name with no servers, the system name itself is removed.</p>

<h2>Registration with Exponential Backoff</h2>

<p>The client side of discovery is equally important.  Each server must
continuously re-register itself to prevent being cleaned up as stale.
The registration function spawns a background task that registers
immediately and then re-registers at a regular interval:</p>

<pre class="code-discovery"><code>pub fn register(name: String, address: String) {
    tokio::spawn(async move {
        let mut retries = 0;
        let max_retries = 10;
        loop {
            // Register with discovery service
            let args = RegisterArgs {
                name: name.clone(),
                address: address.clone(),
            };
            match client::send_request(
                DISCOVERY_ADDRESS,
                Request {
                    procedure_id: REGISTER_PROCEDURE,
                    payload: args.serialize(),
                },
            ).await {
                Ok(_) =&gt; { retries = 0; }
                Err(_) =&gt; {
                    retries += 1;
                    if retries &gt;= max_retries { break; }
                }
            }

            // Re-register periodically
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    });
}</code></pre>

<p>When the discovery service is unavailable, the registration uses exponential
backoff with a maximum delay.  This prevents a thundering herd of
re-registration attempts when the discovery service restarts after an outage.
The delay starts at one second and increases up to sixty seconds, with each
retry doubling the previous delay.</p>

<h2>Design Discussion</h2>

<p>A critical question is how services find the discovery service itself.  Our
implementation uses a well-known address (<code>127.0.0.1:10200</code>).  In
production, there are several alternatives.  DNS can map a well-known hostname
to the discovery service's current address.  Multicast or broadcast protocols
can announce the discovery service on a local network.  Or a small set of
stable addresses can be hardcoded as the discovery service's home.</p>

<p>The random selection in <code>get_address</code> provides basic load balancing but
is not aware of server health or load.  The <a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> service builds on
top of discovery to add health-aware routing and connection pooling.  This
layering &mdash; discovery handles location, routing handles health and efficiency
&mdash; keeps each service focused and simple.</p>

<p>The cleanup duration creates a trade-off between responsiveness and stability.
A short cleanup duration (say, 5 seconds) quickly removes failed servers but
may incorrectly remove servers that are merely slow to re-register.  A longer
duration (say, 60 seconds) is more forgiving but means clients may be
directed to failed servers for up to a minute after they fail.  Our
implementation uses 10 seconds as a compromise.</p>

<p>In larger deployments, the discovery service itself must be replicated for
availability.  Since discovery is essentially a registry of key-value pairs
that must be consistent across replicas, it is a natural candidate for the
<a href="/chapter/consensus" class="sys" style="color:#06D6A0">consensus</a> protocol we examined in the previous chapter.</p>
"##
}

pub fn chapter_routing() -> &'static str {
    r##"
<h1>Chapter 6: Routing</h1>

<p><span class="newthought">Knowing where</span> a server is located is only half the
problem of communicating with it.  The other half is getting requests to
that server efficiently, reliably, and without overwhelming it.  The
<a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> service bridges the gap between discovery (finding servers)
and communication (exchanging data with them), adding connection pooling,
load distribution, and a clean abstraction for clients.</p>

<p>We introduced the routing service in Chapter 1.  Here we examine its
implementation in detail: the connection pool that reuses TCP sockets, the
semaphore that controls concurrency, and the dual-mode architecture that
allows routing to run as either a standalone proxy or an embedded library.</p>

<h2>Interface</h2>

<p><span class="sidenote"><strong><code>routing/src/lib.rs</code></strong></span>
The routing interface consists of a single procedure: route.  A client
specifies the name of the target system, the procedure to invoke, and
the payload.  The routing service handles all the complexity of finding a
server, managing connections, and forwarding the request:</p>

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

<h2>Connection Pool</h2>

<p>Establishing a TCP connection requires a three-way handshake, which adds
latency to every request.  Connection pooling eliminates this overhead by
reusing established connections across multiple requests.  Each system
that a client communicates with has its own pool of connections:</p>

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
}</code></pre>

<p>The pool uses a <code>VecDeque</code> (double-ended queue) to store idle connections
in FIFO order: the oldest idle connection is used first, ensuring that
connections are exercised regularly and stale ones are detected quickly.
The <code>max_size</code> parameter bounds the total number of concurrent connections
to a given system.</p>

<h3>Concurrency Control</h3>

<p>The <code>Semaphore</code> is the key to controlling concurrency.  Before a request
can use a connection, it must acquire a permit from the semaphore.  If all
permits are taken (meaning the maximum number of concurrent requests are
in flight), additional requests block until a permit becomes available:</p>

<pre class="code-routing"><code>pub async fn get(&amp;self) -&gt; Option&lt;TcpStream&gt; {
    self.semaphore.acquire().await
        .expect("Unable to acquire permit").forget();
    let mut pool = self.pool.lock().await;
    let mut conn = pool.pop_front();
    if conn.is_none() {
        let address = discovery::query(
            self.system_name.clone()
        ).await;
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
}</code></pre>

<p>The <code>get</code> method first acquires a semaphore permit, then attempts to
reuse an existing connection from the pool.  If no idle connection is
available, it discovers a server address and establishes a new connection.
The <code>release</code> method returns the connection to the pool (if there is
room) and releases the semaphore permit so another request can proceed.</p>

<p>This design provides natural back-pressure: when a downstream system is
slow, permits accumulate in the semaphore, causing upstream requests to
queue.  This prevents the client from opening an unbounded number of
connections and overwhelming the downstream server.</p>

<h2>Request Forwarding</h2>

<p>With a connection in hand, the pool sends the request in the same format
as a direct RPC call and reads the response:</p>

<pre class="code-routing"><code>pub async fn send_request(
    &amp;self,
    procedure_id: ProcedureId,
    payload: &amp;str,
) -&gt; Result&lt;String, String&gt; {
    if let Some(mut socket) = self.get().await {
        let serialized = format!("{}:{}\n", procedure_id, payload);
        if let Err(e) = socket.write_all(serialized.as_bytes()).await {
            return Err(format!("Failed to send request: {}", e));
        }

        let mut buffer = vec![0u8; 1024];
        let n = socket.read(&amp;mut buffer).await
            .expect("Failed to read from socket");
        let response_data =
            String::from_utf8_lossy(&amp;buffer[..n]).to_string();

        self.release(socket).await;
        Ok(response_data)
    } else {
        Err("Service not available".to_string())
    }
}</code></pre>

<h2>Dual-Mode Architecture</h2>

<p>The routing system can operate in two modes, offering different trade-offs
for different use cases.</p>

<p><strong>Proxy mode</strong> runs routing as a standalone server.  Clients send
their requests to the routing server, which forwards them to the appropriate
backend.  This centralizes connection management and makes it easy to add
cross-cutting concerns like logging, rate limiting, and authentication.
The proxy server listens on a well-known address and dispatches requests
through its own set of connection pools:</p>

<pre class="code-routing"><code>async fn request_handler(
    request: Request,
    shared_state: Arc&lt;Mutex&lt;HashMap&lt;String, ConnectionPool&gt;&gt;&gt;,
) -&gt; Response {
    match request.procedure_id {
        ROUTE_PROCEDURE =&gt;
            handlers::route(&amp;request.payload, shared_state).await,
        _ =&gt; Response {
            payload: "Unknown procedure".to_string(),
        },
    }
}</code></pre>

<p><strong>Library mode</strong> embeds routing directly into the client process.  The
<code>Router</code> struct maintains its own pools and manages connections without
an intermediary.  This eliminates the extra network hop through a proxy
server, reducing latency at the cost of decentralized connection management:</p>

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
            .or_insert_with(||
                ConnectionPool::new(name.clone(), 10));
        route_request(
            RouteArgs { name, procedure_id,
                payload: payload.to_string() },
            pool,
        ).await
    }
}</code></pre>

<h2>Design Discussion</h2>

<p>The choice between proxy and library mode depends on the deployment
environment.  A proxy is simpler for clients (they only need to know the
proxy's address) and provides a single point for observability and policy
enforcement.  A library is faster (no extra hop) and more resilient
(no single point of failure).  Many production systems use both: a
library for latency-sensitive internal communication and a proxy for
external-facing traffic.</p>

<p>The pool size of 10 connections per system is a tuning parameter.  Too few
connections and requests queue unnecessarily; too many and the downstream
server may be overwhelmed.  The optimal pool size depends on the request
rate, the request latency, and the downstream server's capacity.
Little's Law provides a useful guideline: the number of in-flight requests
equals the arrival rate multiplied by the average latency.</p>

<p>The connection pool does not currently perform health checking on idle
connections.  A connection that has been idle for a long time may have
been closed by the server or an intermediary.  Production connection pools
typically include periodic health checks on idle connections and automatic
replacement of broken ones.</p>

<p>Together with <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a>, the <a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> service provides the
communication backbone for the planetary scale computer.  Discovery knows
where servers are; routing knows how to talk to them efficiently.  This
separation of concerns allows each service to evolve independently while
providing a reliable foundation for all inter-service communication.</p>
"##
}

pub fn chapter_monitoring() -> &'static str {
    r##"
<h1>Chapter 14: Monitoring</h1>

<p><span class="newthought">A distributed system</span> that you cannot observe is a
distributed system you cannot operate.  When something goes wrong &mdash; and in a
planetary scale computer, something is <em>always</em> going wrong somewhere
&mdash; operators need to know what is happening, where it is happening, and
ideally why it is happening.  A <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service collects, stores,
and exposes health and performance data from every service in the system.</p>

<p>Monitoring serves two audiences.  For <em>humans</em>, it provides dashboards,
alerts, and diagnostic data to understand system behavior and respond to
incidents.  For <em>machines</em>, it provides health signals that enable
automated actions like load shedding, failover, and auto-scaling.  The
<a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> service, for example, can use health signals from monitoring to
avoid sending traffic to unhealthy servers.</p>

<h2>Interface</h2>

<p><span class="sidenote"><strong><code>monitoring/src/lib.rs</code></strong></span>
The <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service exposes four procedures.  The report procedure
accepts metric data points from services.  The heartbeat procedure accepts
health status updates.  The query procedure retrieves metric time series, and
the health procedure returns the health status of all known services.</p>

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

<p>The <code>ReportArgs</code> structure uses a generic <code>service</code> and
<code>metric</code> pair to identify what is being measured, with an integer
<code>value</code> for the measurement.  This simple schema can represent a wide
variety of metrics: request counts, latencies, queue depths, cache hit rates,
and more.</p>

<h2>Implementation</h2>

<p><span class="sidenote"><strong><code>monitoring/src/main.rs</code></strong></span>
The monitoring server maintains two data structures: a health registry tracking
the status and last heartbeat time of each service, and a metrics store
holding rolling windows of reported values.</p>

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

<p>The heartbeat handler updates the health status and timestamp for a service.
Services are expected to send heartbeats periodically; if a service misses
its heartbeat window, the monitoring system marks it as unhealthy:</p>

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

<p>The report handler stores metric values in a rolling window.  Each metric key
is formed by combining the service name and metric name (e.g.,
<code>storage:latency</code>).  The window holds the most recent 100 values,
which provides enough data for computing statistics like averages and
percentiles without unbounded memory growth:</p>

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

<p>A background task periodically checks for stale services &mdash; those that have
not sent a heartbeat within the timeout period &mdash; and marks them as
unhealthy.  This is the monitoring system's primary mechanism for detecting
failures:</p>

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

<h2>Design Discussion</h2>

<p>The heartbeat pattern is a simple and effective way to detect service failures.
Each service periodically sends a &ldquo;I'm alive&rdquo; message to the monitoring
system.  If the monitoring system doesn't hear from a service within a timeout
period, it assumes the service has failed.  The timeout must be tuned
carefully: too short and healthy services might be marked unhealthy due to
momentary network delays; too long and actual failures take too long to
detect.</p>

<p>The rolling metric window is a compromise between memory efficiency and data
retention.  A fixed window of 100 values provides enough data for basic
statistics while bounding memory usage.  Production monitoring systems like
Prometheus use more sophisticated storage with configurable retention periods
and downsampling for older data.</p>

<p>An important architectural principle is that monitoring should be a
<em>pull</em> or <em>push</em> system, but not both.  Our implementation uses
a push model: services send metrics and heartbeats to the monitoring system.
The alternative, a pull model (used by Prometheus), has the monitoring system
actively scrape metrics from each service.  Push is simpler for services but
makes it harder to detect when a service has disappeared entirely.  Pull makes
failure detection automatic but requires the monitoring system to know about
all services in advance.</p>

<p>The health procedure returns all service statuses in a single response,
making it easy for other systems (like the frontend dashboard) to display a
comprehensive view of system health.  This aggregation is a common pattern in
monitoring systems and forms the basis for status pages and operational
dashboards.</p>
"##
}

// ── Tier 2: Conceptual chapters ─────────────────────────────────────────────

pub fn chapter_implementation() -> &'static str {
    r##"
<h1>Chapter 9: Implementation</h1>

<p><span class="newthought">A design document</span> describes what a system should do.
Implementation is where that description becomes working software.  The
gap between design and code is where subtle bugs are born, where
performance is won or lost, and where a system's true character emerges.
In this chapter, we examine the patterns that appear across all of the
systems we have built and the principles that guide their implementation.</p>

<h2>The Implementation Pattern</h2>

<p>Every service in our planetary scale computer follows the same structural
pattern: a shared library that defines the interface, a server binary that
implements the logic, and shared state that is managed safely across
concurrent requests.</p>

<p>The shared library (<code>lib.rs</code>) defines the procedure identifiers, the
request and response structures, and any client-side helper functions.
This file is the contract.  It is imported by both the server (to implement
the procedures) and by clients (to call them).  Because the library is
shared, changes to it must be made carefully &mdash; a change to a request
structure without a corresponding change to the procedure identifier will
break compatibility between old clients and new servers.</p>

<p>The server binary (<code>main.rs</code>) implements the request handler &mdash; a
function that dispatches incoming requests to the appropriate handler based
on the procedure identifier.  Each handler deserializes the request payload,
performs the operation, and serializes the response.  The server also
initializes shared state, registers with discovery, and starts background
tasks.</p>

<p>Shared state is wrapped in <code>Arc&lt;Mutex&lt;T&gt;&gt;</code> (or <code>Arc&lt;RwLock&lt;T&gt;&gt;</code>
for read-heavy workloads) to allow safe concurrent access from multiple
request handler threads.  This is the standard Rust pattern for shared
mutable state across async tasks.</p>

<h2>Background Tasks</h2>

<p>Most services need work done outside the request-response cycle.  The
<a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a> service runs a background task to clean up expired entries.  The
<a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service checks for stale heartbeats.  The <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a>
service cleans up stale registrations.  The <a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a> service triggers
compaction.</p>

<p>These background tasks follow a common pattern: spawn an async task that
loops with a sleep interval, acquiring the shared state lock, performing
maintenance, and releasing the lock.  The key constraint is that background
tasks must not hold locks for too long, or they will block request handlers.</p>

<h2>Error Handling</h2>

<p>Our implementations take a pragmatic approach to error handling.  Internal
errors (like deserialization failures on well-formed internal traffic) use
<code>expect</code> &mdash; these indicate bugs, not runtime conditions.  External
errors (like network failures when contacting other services) are handled
gracefully, typically by returning an error response to the client or
retrying with backoff.</p>

<p>This distinction matters for operations.  A panic from an <code>expect</code> means
something is fundamentally wrong and the service should restart.  A graceful
error means the service is functioning correctly but encountered a transient
problem in its environment.</p>

<h2>Testing</h2>

<p>The interface-first design pattern naturally supports testing.  Because each
service's interface is defined as typed structures, unit tests can construct
request payloads, call handlers directly, and verify response payloads
without starting a server or making network calls.  Integration tests start
the full server and make RPC calls to verify end-to-end behavior.</p>

<p>The most valuable tests for distributed systems are not unit tests or
integration tests but <em>fault injection</em> tests: what happens when the
discovery service is unavailable?  What happens when a storage write
fails?  What happens when a consensus member crashes mid-replication?
These tests verify the system's resilience, which is ultimately what
matters at planetary scale.</p>
"##
}

pub fn chapter_operation() -> &'static str {
    r##"
<h1>Chapter 10: Operation</h1>

<p><span class="newthought">Building a system</span> is only half the work.  The other half
is keeping it running.  Operation is the practice of deploying, monitoring,
maintaining, and evolving systems in production.  A system that is difficult
to operate will eventually fail, no matter how well it is designed and
implemented.</p>

<h2>Starting Services</h2>

<p>Our planetary scale computer consists of multiple services that must start
in a specific order.  The <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> service must be available before
other services can register.  The <a href="/chapter/configuration" class="sys" style="color:#3A86FF">configuration</a> service should start
next, since other services may read their settings from it.  Then the
remaining services &mdash; <a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a>, <a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a>, <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a>,
<a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a>, and application services &mdash; can start in any order, as
they will retry registration with discovery until they succeed.</p>

<p>The <code>start.sh</code> script encodes this ordering.  It launches each service
as a background process, waits briefly for startup, and proceeds to the next.
This is adequate for development but insufficient for production, where
services should be managed by a process supervisor that handles restarts,
resource limits, and dependency ordering.</p>

<h2>Health Checks</h2>

<p>Once services are running, operators need to know whether they are
<em>healthy</em>.  A running process is not necessarily a healthy process &mdash;
it may be stuck in a deadlock, overwhelmed by traffic, or unable to reach
its dependencies.  Health checks provide a standard way for services to
report their current state.</p>

<p>Our services use the <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service's heartbeat mechanism for health
reporting.  Each service periodically sends a heartbeat with its status.
The monitoring service marks any service that misses its heartbeat window
as unhealthy.  This information feeds into the dashboard, alerting
operators to problems, and into the <a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> service, which avoids
sending traffic to unhealthy servers.</p>

<h2>Observability</h2>

<p>Health checks tell you <em>whether</em> something is wrong.  Observability tells
you <em>what</em> and <em>why</em>.  The three pillars of observability are
metrics (numerical measurements over time), logs (discrete events), and
traces (the path of a request through multiple services).</p>

<p>Our <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service handles metrics.  Each service can report
arbitrary metric values (request counts, latencies, error rates) which
are stored in rolling windows.  For logs, each service writes to standard
output, which can be collected and aggregated by log management systems.
Distributed tracing &mdash; following a single request as it traverses
discovery, routing, and backend services &mdash; is an area where our
system could be extended.</p>

<h2>Operational Runbooks</h2>

<p>When something goes wrong at 3 AM, operators should not need to reason
from first principles.  Operational runbooks document common failure
scenarios and their remediation steps.  For our system, key runbooks
would include: what to do when the discovery service is down (restart it;
other services will re-register automatically), when storage is full
(trigger compaction or expand capacity), and when a consensus ensemble
loses quorum (identify and restart the failed members).</p>

<p>The best runbooks are written by the people who built the system, updated
by the people who operate it, and tested regularly to ensure they still
work.  Over time, the most common runbook steps should be automated,
reducing the burden on human operators.</p>
"##
}

pub fn chapter_scheduling() -> &'static str {
    r##"
<h1>Chapter 11: Scheduling</h1>

<p><span class="newthought">A planetary scale computer</span> consists of thousands or
millions of machines, each capable of running many processes.  Scheduling
is the art and science of deciding <em>where</em> and <em>when</em> to run
work on these machines.  Our
<a href="/dashboard/scheduling" class="sys" style="color:#FF6B35">scheduling</a> service orchestrates the
entire fleet &mdash; spawning service processes, assigning ports, monitoring
health, and reconciling desired state with actual state.</p>

<h2>The Scheduler's Data Model</h2>

<p><span class="sidenote"><strong><code>scheduling/src/main.rs</code></strong></span>
The scheduler maintains two core structures.  A <em>ServiceSpec</em> describes the
desired state: the service name, the Cargo manifest path, an optional binary
name, and the desired replica count.  An <em>Instance</em> describes reality: a
running process with an ID, port, OS process ID, and health status.</p>

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

<p>The gap between desired and actual state drives all scheduling decisions.
When a service spec says three replicas but only two instances are running,
the scheduler spawns one more.  When an instance's health check fails, the
scheduler marks it unhealthy and may replace it.</p>

<h2>Process Spawning</h2>

<p><span class="sidenote"><strong><code>echo/src/bin/server_v1.rs</code></strong> &mdash; this pattern appears in every service</span>
The scheduler spawns each service as an OS process via <code>std::process::Command</code>,
passing the assigned port through the <code>PORT</code> environment variable.  Every
service in our system checks for this variable at startup:</p>

<pre class="code-scheduling"><code>let addr = std::env::var("PORT")
    .map(|p| format!("127.0.0.1:{}", p))
    .unwrap_or_else(|_| SYSTEM_ADDRESS.to_string());</code></pre>

<p>This three-line pattern appears in every service's <code>main.rs</code>.  It means
services can run standalone with their well-known port (for development) or
accept a dynamically assigned port (when managed by the scheduler).  The
spawned process self-registers with
<a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a>, making it immediately
discoverable by other services.</p>

<h2>Fleet Bootstrap</h2>

<p><span class="sidenote"><strong><code>scheduling/src/main.rs</code></strong></span>
On startup, the scheduler bootstraps the entire fleet from a hardcoded
configuration table.  Each entry specifies the service name, Cargo manifest path,
optional binary name, replica count, and base port.  Single-replica services get
their well-known ports.  Multi-replica services get sequential ports from a base.</p>

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
For each entry, the scheduler calls <code>spawn_instance</code>, which builds a
<code>cargo run</code> command with the manifest path and passes the assigned port
through the <code>PORT</code> environment variable.  The child process ID is captured
so the scheduler can later kill the instance if needed.</p>

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

<h2>Health Monitoring</h2>

<p><span class="sidenote"><strong><code>scheduling/src/main.rs</code></strong> &mdash; <code>health_check_loop</code></span>
A background loop runs every five seconds, probing each instance with a TCP
connection attempt.  If the connection succeeds, the instance is marked healthy.
If it fails, the instance is marked unhealthy.  This is the simplest possible
health check &mdash; a production scheduler would also check application-level
health endpoints, resource usage, and response latency.</p>

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

<h2>RPC Interface</h2>

<p><span class="sidenote"><strong><code>scheduling/src/lib.rs</code></strong></span>
The scheduling service exposes five procedures through our
<a href="/chapter/routing" class="sys" style="color:#F4845F">RPC</a> framework.
<code>SCHEDULE_SERVICE</code> registers a new service spec and reconciles it.
<code>LIST_INSTANCES</code> returns all running instances.  <code>SCALE_SERVICE</code>
updates the replica count.  <code>STOP_INSTANCE</code> kills a specific
instance by process ID.  <code>GET_SERVICE</code> returns the spec and instances for
one service.</p>

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

<p>The <a href="/dashboard/scheduling">scheduling dashboard</a> uses
these procedures to display the fleet and allow operators to scale services
or stop individual instances.</p>
"##
}

pub fn chapter_release() -> &'static str {
    r##"
<h1>Chapter 12: Release</h1>

<p><span class="newthought">Software that never changes</span> is software that is never
improved.  The <a href="/dashboard/release" class="sys" style="color:#4CC9F0">release</a> service
manages rolling deployments across the fleet, coordinating with the
<a href="/chapter/scheduling" class="sys" style="color:#FF6B35">scheduler</a> to replace
instances one batch at a time with zero downtime.</p>

<h2>Interface</h2>

<p><span class="sidenote"><strong><code>release/src/lib.rs</code></strong></span>
The release service exposes five procedures.  <code>CREATE_RELEASE</code> starts a new
deployment.  <code>GET_RELEASE</code> and <code>LIST_RELEASES</code> inspect state.
<code>ADVANCE_RELEASE</code> pushes the deployment forward one batch.
<code>ROLLBACK</code> reverts a deployment in progress.</p>

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

<h2>Release Lifecycle</h2>

<p><span class="sidenote"><strong><code>release/src/main.rs</code></strong></span>
A release progresses through a simple state machine: <code>created</code> &rarr;
<code>deploying</code> &rarr; <code>deployed</code> (or <code>rolled_back</code>).
Each state transition is explicit &mdash; an operator advances the release
through the <a href="/dashboard/release">release dashboard</a>, giving them
control over the pace of the rollout.</p>

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

<h2>Rolling Updates</h2>

<p><span class="sidenote"><strong><code>release/src/main.rs</code></strong> &mdash; <code>create_release</code></span>
When a release is created, the service queries the
<a href="/chapter/scheduling" class="sys" style="color:#FF6B35">scheduler</a> for the current
instances of the target service and snapshots them as the &ldquo;old&rdquo; instances.
It calculates the batch size as <code>max(1, total / 10)</code> &mdash; roughly 10%
of the fleet per batch, with a minimum of one instance.</p>

<pre class="code-release"><code>let svc_result = scheduling::get_service(
    SCHEDULER_ADDR, args.service.clone()
).await;
let old_instances: Vec&lt;String&gt; = svc_result.instances
    .split(';').map(|s| s.to_string()).collect();
let total = old_instances.len() as i32;
let batch_size = std::cmp::max(1, total / 10);</code></pre>

<p><span class="sidenote"><strong><code>release/src/main.rs</code></strong> &mdash; <code>advance_release</code></span>
Each call to <code>ADVANCE_RELEASE</code> replaces one batch.  The handler
tells the scheduler to scale up, waits for health, then stops one old instance:</p>

<ol>
<li>Tell the scheduler to spawn new instances for the batch</li>
<li>Wait for the new instances to pass health checks</li>
<li>Tell the scheduler to stop the corresponding old instances</li>
<li>Old instances deregister from
<a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> via stale cleanup</li>
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

<p>This process ensures that at every moment during the rollout, the service
has enough healthy instances to handle traffic.  The
<a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> layer and discovery
service automatically direct traffic away from stopped instances and toward
new ones.</p>

<h2>Rollback</h2>

<p><span class="sidenote"><strong><code>release/src/main.rs</code></strong> &mdash; <code>rollback</code></span>
If a deployment goes wrong, the <code>ROLLBACK</code> procedure marks the
active release as <code>rolled_back</code>.  Because the old instances are only
stopped after new ones are confirmed healthy, a rollback during deployment
simply stops the process &mdash; the remaining old instances continue serving
traffic.  The key to fast rollback is <em>never</em> removing the old before
the new is proven.</p>

<h2>Integration with the Scheduler</h2>

<p><span class="sidenote"><strong><code>release/src/lib.rs</code></strong></span>
The release service does not spawn processes directly.  Instead, it
delegates all process management to the scheduler through RPC calls:
<code>scheduling::scale_service()</code> to add replicas and
<code>scheduling::stop_instance()</code> to remove them.  This separation of
concerns means the scheduler remains the single source of truth for
what is running, while the release service manages the <em>order</em> and
<em>pace</em> of changes.  Visit the
<a href="/dashboard/release">release dashboard</a> to create a release and
step through a rolling deployment.</p>
"##
}

pub fn chapter_security() -> &'static str {
    r##"
<h1>Chapter 13: Security</h1>

<p><span class="newthought">A planetary scale computer</span> is a vast attack surface.
Our <a href="/dashboard/security" class="sys" style="color:#D62828">security</a> service provides
token-based authentication for the system's dashboard, demonstrating the
principles of authentication, authorization, and token management that
protect real distributed systems.</p>

<h2>Interface</h2>

<p><span class="sidenote"><strong><code>security/src/lib.rs</code></strong></span>
The security service exposes four procedures.  <code>CREATE_TOKEN</code> generates a new
token with a name and permission set.  <code>VALIDATE_TOKEN</code> checks whether a
token is valid and returns the associated identity.  <code>REVOKE_TOKEN</code> invalidates
a compromised token.  <code>LIST_TOKENS</code> enumerates all active tokens for the
dashboard.</p>

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

<h2>Token-Based Authentication</h2>

<p><span class="sidenote"><strong><code>security/src/main.rs</code></strong></span>
The security service manages authentication tokens &mdash; opaque strings that
grant access to protected operations.  Each token has a name (identifying
who it belongs to), a set of permissions, and a creation timestamp.  The
frontend checks for an <code>auth_token</code> cookie on every sensitive dashboard
operation (POST requests that modify state) by calling the security
service's <code>VALIDATE_TOKEN</code> procedure.</p>

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

<h2>Token Generation: PRNGs in Distributed Systems</h2>

<p><span class="sidenote"><strong><code>security/src/main.rs</code></strong></span>
Token generation raises an interesting question: where does randomness come
from in a distributed system?  Hardware random number generators are slow.
Cryptographic PRNGs (like <code>/dev/urandom</code>) are better but still involve
system calls.  For our educational implementation, we use xorshift64 &mdash;
a fast, self-contained pseudorandom number generator:</p>

<pre class="code-security"><code>fn xorshift64(&amp;mut self) -&gt; u64 {
    let mut x = self.rng_state;
    x ^= x &lt;&lt; 13;
    x ^= x &gt;&gt; 7;
    x ^= x &lt;&lt; 17;
    self.rng_state = x;
    x
}</code></pre>

<p>The xorshift64 generator produces uniformly distributed 64-bit values with
a period of 2<sup>64</sup>&minus;1.  We concatenate two outputs to form a
128-bit hex token.  In a production system, you would use a cryptographically
secure PRNG, but xorshift64 demonstrates the core concept: deterministic
functions that produce seemingly random output from a seed value.  The seed
is derived from the system clock at startup, making each instance's token
stream unique.</p>

<h2>The Bootstrap Problem</h2>

<p><span class="sidenote"><strong><code>security/src/main.rs</code></strong> &mdash; admin token seeding at startup</span>
Token-based auth creates a chicken-and-egg problem: how do you create the
first token if all mutations require a valid token?  An early approach was a
<em>bootstrap exception</em> &mdash; leaving one route unauthenticated &mdash; but that
creates an attack surface on any public deployment.  Instead, we seed the
admin token from an environment variable at startup:</p>

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

<p>The startup script (<code>start.sh</code>) generates a random token with
<code>openssl rand -hex 16</code>, exports it as <code>ADMIN_TOKEN</code>, and
prints it for the operator.  The operator sets the cookie via browser console
(<code>document.cookie = "auth_token=...;path=/"</code>) and can then use the
dashboard to create additional tokens.  Every POST route &mdash; including
token creation &mdash; requires a valid <code>auth_token</code> cookie.</p>

<p>This pattern is standard in real systems.  Kubernetes creates a bootstrap
token during cluster initialization.  Cloud providers use IAM root
credentials seeded at provisioning time.  The key insight is that the
bootstrap secret lives in the deployment environment, not in an
unauthenticated HTTP endpoint.</p>

<h2>Authorization Middleware</h2>

<p><span class="sidenote"><strong><code>frontend/src/main.rs</code></strong></span>
The frontend implements authorization as middleware &mdash; a function that runs
before each protected route handler:</p>

<pre class="code-security"><code>async fn require_admin(headers: &amp;str) -&gt; bool {
    if let Some(token) = parse_cookie(headers, "auth_token") {
        let result = security::validate_token(SECURITY_ADDR, token).await;
        return result.valid == 1;
    }
    false
}</code></pre>

<p>This pattern &mdash; extracting credentials from the request, validating them
against a central authority, and gating access based on the result &mdash; is
the same pattern used by API gateways, service meshes, and web frameworks
at planetary scale.  The
<a href="/dashboard/security">security dashboard</a> lets you create tokens,
view active tokens, and revoke compromised ones.</p>

<h2>Integrity</h2>

<p><span class="newthought">Authentication controls</span> who can act, but it does
not protect against volume.  A valid user can still overwhelm a system with
requests, and an attacker does not need credentials to consume resources.
<em>Integrity</em> is the practice of ensuring a system remains functional under
hostile conditions &mdash; rate limiting, IP blackholing, and defense in depth.</p>

<h3>Rate Limiting</h3>

<p><span class="sidenote"><strong><code>loadbalancer/src/main.rs</code></strong> &mdash; <code>TokenBucket</code> struct</span>
The load balancer implements per-IP rate limiting using a
<a href="https://en.wikipedia.org/wiki/Token_bucket">token bucket</a> algorithm.
Each IP address gets a bucket that holds up to 30 tokens (the burst capacity)
and refills at 2 tokens per second (sustaining ~120 requests per minute).
Every request consumes one token.  When the bucket is empty, the request is
rejected with <code>429 Too Many Requests</code>:</p>

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

<p>Token buckets are elegant because they allow bursts (a user loading a page
fetches several resources at once) while enforcing a sustained rate.  The
alternative &mdash; fixed-window counters &mdash; creates boundary problems where a
client can send twice the limit by timing requests across the window edge.</p>

<h3>IP Blackholing</h3>

<p><span class="sidenote"><strong><code>loadbalancer/src/main.rs</code></strong> &mdash; <code>record_violation()</code></span>
Rate limiting alone is not sufficient.  An attacker who is repeatedly rejected
still consumes CPU cycles for each rejection.  The load balancer escalates
automatically: if an IP accumulates 10 rejections within 60 seconds, it is
<em>blackholed</em> &mdash; banned for 5 minutes.  Blackholed IPs receive
<code>429 Retry-After: 300</code> immediately, before the request body is even read:</p>

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

<p>A background task sweeps all three maps (rate limit buckets, blacklist entries,
violation counters) every 60 seconds to remove expired state.  This prevents
memory growth from long-running deployments.</p>

<h3>Defense in Depth</h3>

<p>These protections layer together.  The
<a href="/chapter/load-balancing" class="sys" style="color:#2A9D8F">load balancer</a> is the first
line of defense: rate limiting and blackholing happen before requests reach any
backend service.  The frontend is the second line: every dashboard mutation
requires a valid <code>auth_token</code> cookie validated against the security service.
The <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service provides
visibility &mdash; the load balancer reports <code>rate_limited</code> and
<code>blackholed</code> metrics so operators can detect attacks in real time.</p>

<p>Introspection endpoints (<code>/__lb_status</code> and <code>/__lb_strategy</code>)
are restricted to loopback addresses, preventing external users from reading
backend topology or changing the load balancing strategy.  All backend services
bind to <code>127.0.0.1</code> and are unreachable from the internet &mdash; only the
load balancer listens on <code>0.0.0.0</code>.  This is the same architecture used
by production reverse proxies: a single hardened entry point funneling traffic
to internal services.</p>
"##
}

// ── Tier 3: Brief conceptual chapters ───────────────────────────────────────

pub fn chapter_capacity() -> &'static str {
    r##"
<h1>Chapter 15: Capacity</h1>

<p><span class="newthought">Every system</span> has limits.  A server can only handle so many
requests per second.  A disk can only store so many bytes.  A network link
can only carry so much bandwidth.  <em>Capacity</em> is the measurement of these
limits and the practice of ensuring that a system has enough resources to
meet its workload.</p>

<p>Capacity is measured at every level of the stack.  At the hardware level:
CPU cores, memory gigabytes, disk IOPS, network bandwidth.  At the software
level: requests per second, concurrent connections, queue depth, cache hit
rate.  At the service level: the number of users that can be served, the
volume of data that can be stored, the latency that can be achieved.</p>

<p>The relationship between load and performance is rarely linear.  A server that
handles 1,000 requests per second with 10ms latency might handle 2,000 with
15ms, 3,000 with 50ms, and collapse entirely at 3,500.  Understanding these
non-linear relationships &mdash; through load testing, modeling, and experience &mdash;
is essential for capacity planning.</p>

<p>Capacity management is a continuous process.  As traffic grows, new features
are added, and usage patterns change, the capacity requirements of a system
evolve.  The <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service provides the data needed to track
capacity utilization over time and the <a href="/chapter/configuration" class="sys" style="color:#3A86FF">configuration</a> service allows
capacity parameters (like connection pool sizes and cache limits) to be
adjusted without redeployment.</p>
"##
}

pub fn chapter_utilization() -> &'static str {
    r##"
<h1>Chapter 16: Utilization</h1>

<p><span class="newthought">Capacity tells you</span> how much a system <em>can</em> do.
Utilization tells you how much it <em>is</em> doing.  Utilization is expressed
as a percentage of capacity: a server using 70% of its CPU, a disk that is
85% full, a network link carrying 40% of its maximum bandwidth.</p>

<p>High utilization means resources are being used efficiently, but it also
means there is little headroom for spikes in traffic or unexpected failures.
Low utilization means the system is over-provisioned &mdash; resources are being
paid for but not used.  The sweet spot depends on the service's requirements:
a latency-sensitive service might target 50% utilization to leave room for
bursts, while a batch processing system might target 90%.</p>

<p>Utilization must be monitored across all resource dimensions simultaneously.
A server with low CPU utilization but high memory utilization is still at
risk of failure.  The bottleneck resource &mdash; the one closest to capacity &mdash;
determines the system's effective capacity.  Identifying and relieving
bottlenecks is a core operational skill.</p>

<p>Our <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service tracks utilization metrics like cache hit rates,
queue depths, and request rates.  These metrics, combined with system-level
metrics like CPU and memory usage, provide a comprehensive picture of how
effectively the system's resources are being used.</p>
"##
}

pub fn chapter_efficiency() -> &'static str {
    r##"
<h1>Chapter 17: Efficiency</h1>

<p><span class="newthought">Efficiency measures</span> how much useful work a system produces
per unit of resource consumed.  A system that serves 10,000 requests per
second on one server is more efficient than one that requires ten servers
for the same throughput.  At planetary scale, small efficiency improvements
compound into enormous savings &mdash; a 10% improvement across a million
servers frees up 100,000 servers' worth of resources.</p>

<p>Efficiency improvements come from every layer of the stack.  At the
algorithm level: a hash map lookup is O(1) versus O(n) for a linear scan.
At the data structure level: our <a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a> service's LRU eviction keeps
the most useful data in memory.  At the protocol level: connection pooling
in the <a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> service eliminates per-request connection overhead.
At the system level: the <a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a> service's compaction reclaims disk
space from deleted entries.</p>

<p>There is a tension between efficiency and other qualities like simplicity,
reliability, and development velocity.  Premature optimization often
produces complex code that is hard to maintain and debug.  The most
effective approach is to build simple systems first, measure their
performance in production, identify the actual bottlenecks (which are
often not where you expect), and optimize surgically.</p>
"##
}

pub fn chapter_load_testing() -> &'static str {
    r##"
<h1>Chapter 18: Load Testing</h1>

<p><span class="newthought">You cannot manage</span> what you cannot measure, and you cannot
trust a measurement you have not validated.  Load testing subjects a system
to controlled, artificial traffic to measure its performance characteristics
under various conditions.  It answers questions like: how many requests per
second can this system handle?  At what point does latency become
unacceptable?  What happens when a dependency fails under load?</p>

<p>Load testing comes in several forms.  <em>Baseline tests</em> measure
performance under normal expected traffic.  <em>Stress tests</em> push the
system beyond its expected limits to find breaking points.  <em>Soak tests</em>
run at moderate load for extended periods to detect slow resource leaks.
<em>Spike tests</em> simulate sudden bursts of traffic to verify the system's
ability to absorb surges.</p>

<p>The most common mistake in load testing is testing a system in isolation
rather than in the context of its dependencies.  Our <a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a> service's
performance depends on the <a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a> service's hit rate, which depends
on the traffic pattern, which depends on the load test's design.  Realistic
load testing must exercise the full dependency chain with realistic data and
access patterns.</p>

<p>Load test results should be compared against capacity models and SLOs
(Service Level Objectives) to determine whether the system meets its
requirements.  The results should also be archived so that performance
regressions from code changes can be detected by comparing against
previous test runs.</p>
"##
}

pub fn chapter_planning() -> &'static str {
    r##"
<h1>Chapter 19: Planning</h1>

<p><span class="newthought">Capacity planning</span> is the practice of predicting future
resource needs and ensuring that infrastructure is available before demand
arrives.  Running out of capacity is a crisis; having too much capacity
is waste.  Good planning threads the needle between these extremes.</p>

<p>Planning starts with data: historical utilization trends from the
<a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service, growth projections from product teams, and
knowledge of upcoming launches or events that might cause traffic spikes.
Simple trend extrapolation (if storage is growing 5% per month, we'll need
another server in six months) works for steady growth.  Step functions (a
product launch that doubles traffic overnight) require more explicit
planning.</p>

<p>Lead times are critical.  If ordering new hardware takes three months and
traffic is growing 10% per month, you must order hardware when you are at
70% capacity &mdash; not when you run out.  Cloud infrastructure shortens lead
times (new servers in minutes instead of months) but introduces its own
planning challenges around cost management and reserved capacity.</p>

<p>The best capacity plans include a <em>buffer</em> for the unexpected: a
traffic spike from a viral event, a hardware failure that reduces effective
capacity, or a dependency that becomes slower and backs up queues.  A
common rule of thumb is to keep 20-30% headroom above expected peak
utilization.</p>
"##
}

pub fn chapter_degradation() -> &'static str {
    r##"
<h1>Chapter 20: Degradation</h1>

<p><span class="newthought">When demand exceeds capacity,</span> a system has two choices:
fail completely or degrade gracefully.  Graceful degradation means
continuing to provide reduced functionality rather than returning errors
to all users.  It is the difference between a slow website and an
unreachable one.</p>

<p>Degradation strategies include <em>load shedding</em> (rejecting a fraction
of requests to protect the rest), <em>feature reduction</em> (disabling
expensive features to reduce resource consumption), and <em>priority
queuing</em> (serving high-priority requests before low-priority ones).
Each strategy trades some functionality for continued availability.</p>

<p>In our system, the <a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a> service provides a natural degradation
path for the <a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a> service.  If the storage service becomes
overloaded, the caching service can serve stale data rather than letting
requests fail entirely.  The <a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> service can stop sending traffic
to overloaded backends, spreading load to healthier instances.</p>

<p>Degradation must be tested.  A degradation strategy that has never been
exercised is a degradation strategy that does not work.  Chaos engineering
&mdash; deliberately injecting failures in production &mdash; is the practice of
verifying that degradation mechanisms work as designed before they are
needed in earnest.</p>
"##
}

pub fn chapter_load_balancing() -> &'static str {
    r##"
<h1>Chapter 21: Load Balancing</h1>

<p><span class="newthought">When multiple servers</span> can handle the same request, a
load balancer decides which server should receive each request.  Effective
load balancing ensures that no single server becomes a bottleneck while
others sit idle.  It is a fundamental technique for achieving both
scalability and reliability in distributed systems.</p>

<h2>Gateway Load Balancer</h2>

<p>Our system's entry point is a gateway load balancer that sits in front of
multiple frontend instances.  It maintains a pool of backends, each tracked
with its health status and active connection count:</p>

<span class="sidenote">See <code>loadbalancer/src/main.rs</code> for the full implementation.</span>

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

<p>The load balancer discovers its backends dynamically.  Every five seconds, a
background task calls <code>discovery::list("frontend")</code> to refresh the backend
list.  New backends are added automatically; backends that disappear from
<a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> are removed.</p>

<h2>Balancing Strategies</h2>

<p>The gateway supports four strategies, selectable at runtime via the
<code>/__lb_strategy</code> endpoint or the <code>STRATEGY</code> environment variable:</p>

<p><em>Round-robin</em> distributes requests sequentially, skipping unhealthy backends.
It works well when servers and requests are homogeneous:</p>

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

<p><em>Least-connections</em> sends each request to the healthy backend with the
fewest active connections.  This naturally accounts for heterogeneous request
costs &mdash; slow requests keep a connection open longer, directing traffic
elsewhere:</p>

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

<p><em>Random</em> selects a healthy backend uniformly at random.  Simple and
stateless, but it can produce uneven distribution with small backend counts.</p>

<p><em>Power of two random choices</em> (pick-2) is a particularly elegant
algorithm: pick two random healthy backends and choose the one with fewer
active connections.  Research shows this achieves exponentially better load
distribution than pure random selection, with minimal coordination.</p>

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

<h2>Health Checking</h2>

<p>A background loop probes each backend every three seconds with a TCP
connection attempt.  Backends that fail the probe are marked unhealthy and
excluded from selection.  When they recover, they are automatically
re-included:</p>

<pre class="code-loadbalancer"><code>// Health check loop: probe each backend every 3 seconds
let health_lb = Arc::clone(&amp;lb);
tokio::spawn(async move {
    loop {
        sleep(HEALTH_CHECK_INTERVAL).await;
        let mut lb = health_lb.lock().await;
        for backend in lb.backends.iter_mut() {
            let was_healthy = backend.healthy;
            backend.healthy =
                TcpStream::connect(&amp;backend.address).await.is_ok();
            if was_healthy != backend.healthy {
                println!("Backend {} is now {}",
                    backend.address,
                    if backend.healthy { "healthy" } else { "unhealthy" });
            }
        }
    }
});</code></pre>

<p>The health check is deliberately simple: a TCP connection attempt.  If
the connection succeeds, the backend is healthy.  If it fails, it is marked
unhealthy and excluded from <code>select_backend</code> until the next successful
probe.  The <code>/__lb_status</code> endpoint exposes the full state of the
backend pool as JSON for the <a href="/dashboard/loadbalancer">dashboard</a>.</p>

<h2>Backend Discovery</h2>

<p><span class="sidenote"><strong><code>loadbalancer/src/main.rs</code></strong></span>
The load balancer does not use a static configuration file.  Instead, a
background task calls <code>discovery::list("frontend")</code> every five seconds
to discover which frontend instances are currently registered.  New backends
are added automatically; backends that have deregistered are removed:</p>

<pre class="code-loadbalancer"><code>fn refresh_backends(&amp;mut self, addresses: &amp;[String]) {
    // Add new backends
    for addr in addresses {
        if !self.backends.iter().any(|b| &amp;b.address == addr) {
            self.backends.push(Backend {
                address: addr.clone(),
                healthy: true,
                active_connections: 0,
            });
        }
    }

    // Remove stale backends
    self.backends.retain(|b| addresses.contains(&amp;b.address));
}</code></pre>

<p>This dynamic discovery means the load balancer automatically adapts as
the <a href="/chapter/scheduling" class="sys" style="color:#FF6B35">scheduler</a> scales
the frontend fleet up or down.  No restarts or configuration changes are
needed.</p>

<h2>Active Connection Tracking</h2>

<p>The load balancer tracks active connections per backend to support
the least-connections and pick-2 strategies.  When a backend is selected,
its <code>active_connections</code> counter is incremented.  When the proxied
request completes (whether successfully or not), the counter is decremented:</p>

<pre class="code-loadbalancer"><code>// Select backend and increment active connections
let (backend_addr, backend_idx) = {
    let mut lb = lb.lock().await;
    match lb.select_backend() {
        Some(idx) =&gt; {
            lb.backends[idx].active_connections += 1;
            (lb.backends[idx].address.clone(), idx)
        }
        None =&gt; { /* return 503 */ }
    }
};

// ... proxy the request ...

// Decrement active connections when done
lb.lock().await.backends[backend_idx]
    .active_connections = lb.lock().await.backends[backend_idx]
    .active_connections.saturating_sub(1);</code></pre>

<p>The <code>saturating_sub</code> prevents underflow if a backend is removed from
the pool while a request is in flight.  This bookkeeping is the foundation
that makes load-aware strategies like least-connections and pick-2
effective.</p>

<h2>Service-Level Load Balancing</h2>

<p>Load balancing also happens inside the system.  The
<a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> service maintains a
multi-backend connection pool for each service it routes to.  When a
request arrives for a service like "storage", routing calls
<code>discovery::list("storage")</code> to get all registered backends, then selects
one using the same strategy set (round-robin, least-connections, random,
or pick-2).</p>

<span class="sidenote">See <code>routing/src/lib.rs</code> for the <code>ConnectionPool</code> implementation.</span>

<pre class="code-routing"><code>struct BackendPool {
    address: String,
    connections: VecDeque&lt;TcpStream&gt;,
}

struct ConnectionPool {
    system_name: String,
    backends: Vec&lt;BackendPool&gt;,
    strategy: String,
    next_index: usize,
    max_per_backend: usize,
}</code></pre>

<p>The routing pool refreshes backends from discovery every ten seconds,
adding new backends and removing stale ones.  Each backend maintains its
own connection queue, so connections are reused efficiently and directed
to the correct backend on release.</p>

<p>This two-layer approach &mdash; gateway balancing at the edge, service-level
balancing internally &mdash; provides defense in depth.  Even if the gateway
uses simple round-robin, the routing layer can independently optimize
traffic to individual services based on their characteristics.</p>

<h2>Design Discussion</h2>

<p>The choice of balancing strategy depends on the workload.  Round-robin
works well when requests are roughly equal in cost and servers are
homogeneous.  Least-connections adapts naturally to heterogeneous request
costs: a slow request occupies a connection longer, directing subsequent
traffic to less-loaded servers.  Pick-2 strikes a balance, providing
most of the benefit of least-connections with the simplicity of random
selection.</p>

<p>A production load balancer would add several features beyond what we
have implemented.  <em>Weighted backends</em> allow servers of different
capacities to receive proportional traffic.  <em>Session affinity</em> (sticky
sessions) ensures that requests from the same client reach the same backend,
important for stateful applications.  <em>Circuit breaking</em> removes
backends that repeatedly fail rather than probing them indefinitely.
<em>Graceful draining</em> stops sending new requests to a backend being
decommissioned while allowing existing connections to complete.</p>

<p>The two-layer architecture &mdash; gateway balancing at the edge,
service-level balancing inside the <a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> layer &mdash; provides
defense in depth.  Even if the gateway uses simple round-robin, the
routing layer can independently optimize traffic to individual backend
services.  This separation also means the gateway can be replaced (for
example, with nginx or HAProxy) without affecting internal routing.</p>
"##
}

pub fn chapter_consistency() -> &'static str {
    r##"
<h1>Chapter 22: Consistency</h1>

<p><span class="newthought">When data is replicated</span> across multiple servers, a
fundamental question arises: what guarantees do readers have about the
data they see?  <em>Consistency</em> is the set of rules that govern the
relationship between writes and subsequent reads in a distributed system.</p>

<h2>Storage Quorum Replication</h2>

<p>Our <a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a> service runs as three
replicas (N=3), each maintaining its own WAL and snapshot.  Every value is
tagged with a monotonically increasing version number:</p>

<span class="sidenote">See <code>storage/src/engine.rs</code> for versioning.</span>

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

<h3>Replication Interface</h3>

<p><span class="sidenote"><strong><code>storage/src/lib.rs</code></strong></span>
The replication interface uses separate procedures that bypass the quorum
logic, preventing cascading replication.  A <code>GET_PEERS</code> procedure
exposes the current replication topology:</p>

<pre class="code-storage"><code>pub const REPLICATE_PUT_PROCEDURE: ProcedureId = 5;
pub const REPLICATE_DELETE_PROCEDURE: ProcedureId = 6;
pub const GET_PEERS_PROCEDURE: ProcedureId = 7;

#[derive(Debug, Serializable, Deserializable)]
pub struct ReplicatePutArgs {
    pub key: String,
    pub value: String,
    pub version: i32,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct GetPeersResult {
    pub peer_count: i32,
    pub quorum_w: i32,
    pub quorum_r: i32,
}</code></pre>

<p>Each storage replica maintains its own data directory, derived from its
port: <code>storage_data_10600</code>, <code>storage_data_10601</code>,
<code>storage_data_10602</code>.  This ensures replicas do not share WAL or
snapshot files, which would corrupt the versioning guarantees.  The
<a href="/chapter/scheduling" class="sys" style="color:#FF6B35">scheduler</a> assigns each
instance a unique port, and each instance self-registers with
<a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a>, making all
three replicas discoverable via <code>discovery::list("storage")</code>.</p>

<p>Quorum writes require W acks before returning to the client.  With W=2
and N=3, the writing node performs a local write (which counts as one ack),
then replicates to peers via <code>discovery::list("storage")</code> and waits for
one additional peer ack:</p>

<span class="sidenote">See <code>storage/src/main.rs</code> for the quorum write path.</span>

<pre class="code-storage"><code>// Local write counts as 1
let version = engine.put(key.clone(), value.clone());

// Replicate to W-1 peers
let peers = discovery::list("storage").await;
let mut acks = 0;
for peer in &amp;peers {
    if acks &gt;= W - 1 { break; }
    let result = replicate_put(peer, key, value, version).await;
    if result == "OK" { acks += 1; }
}</code></pre>

<p>Quorum reads work symmetrically: with R=2, the reading node reads locally
and from one peer, returning the value with the highest version.  The key
invariant is <strong>W + R &gt; N</strong> (2 + 2 &gt; 3), which guarantees that any read
quorum overlaps with any write quorum &mdash; at least one node in the read
set has the latest write.</p>

<h3>Version Gating</h3>

<p><span class="sidenote"><strong><code>storage/src/engine.rs</code></strong></span>
When a replication message arrives, the receiving node must decide whether
to accept it.  The <code>put_versioned</code> method implements last-writer-wins:
a write is accepted only if its version is at least as recent as the
current value.  This prevents old replication messages (delayed by the
network) from overwriting newer data:</p>

<pre class="code-storage"><code>pub fn put_versioned(
    &amp;mut self, key: String, value: String, version: u64,
) -&gt; bool {
    if let Some(current) = self.data.get(&amp;key) {
        if version &lt; current.version {
            return false; // Reject stale write
        }
    }
    self.append_wal(&amp;format!("VPUT {}={}@{}", key, value, version));
    self.data.insert(key, VersionedValue { value, version });
    true
}</code></pre>

<p>This version check is the core of conflict resolution.  Because versions
are monotonically increasing and assigned by the originating node, two
concurrent writes to different nodes will have different versions, and the
higher version always wins.  This is a practical form of
<em>last-writer-wins</em> that resolves conflicts without coordination.</p>

<h3>Quorum Read Path</h3>

<p><span class="sidenote"><strong><code>storage/src/main.rs</code></strong></span>
A quorum read contacts R&minus;1 peers (since the local read counts as one)
and returns the value with the highest version found.  The code filters
out the reading node itself using <code>discovery::list</code>:</p>

<pre class="code-storage"><code>async fn get_peers(own_addr: &amp;str) -&gt; Vec&lt;String&gt; {
    let result = discovery::list(SYSTEM_NAME.to_string()).await;
    result.addresses.split(';')
        .filter(|s| !s.is_empty() &amp;&amp; *s != own_addr)
        .map(|s| s.to_string())
        .collect()
}

// In the GET handler:
let local = engine.get_versioned(&amp;key);
let mut best_value = local.map(|v| v.value.clone()).unwrap_or_default();
let mut best_found = if local.is_some() { 1 } else { 0 };

let peers = get_peers(&amp;own_addr).await;
let needed = (quorum_r - 1) as usize;
let mut acks = 0;
for peer in &amp;peers {
    if acks &gt;= needed { break; }
    let result = storage::remote_get(peer, key.clone()).await;
    acks += 1;
    if result.found == 1 &amp;&amp; best_found == 0 {
        best_value = result.value.clone();
        best_found = 1;
    }
}</code></pre>

<p>The <code>get_peers</code> pattern &mdash; calling <code>discovery::list</code> and
filtering out self &mdash; appears in both storage and caching.  Each replica
is both a server (handling client requests with quorum logic) and a peer
(accepting replication requests directly).</p>

<h2>Cache Consistency Modes</h2>

<p>While storage uses fixed quorum parameters, our
<a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a> service supports three
configurable consistency modes, switchable at runtime:</p>

<span class="sidenote">See <code>caching/src/main.rs</code> for mode implementation.</span>

<h3>Eventual Consistency</h3>

<p>The default mode.  Writes go to the local cache and are replicated
asynchronously in a background task.  Reads serve only from the local cache.
This provides the lowest latency but may serve stale data if a write has
reached some replicas but not others.</p>

<pre class="code-caching"><code>// Eventual: fire-and-forget replication
tokio::spawn(async move {
    let peers = get_peers(&amp;own_addr).await;
    for peer in &amp;peers {
        let _ = replicate_set(peer, key, value, ttl, version).await;
    }
});</code></pre>

<h3>Quorum Consistency</h3>

<p>Writes wait for one peer ack (W=2 of N=3) before returning.  Reads fetch
from one peer in addition to the local cache (R=2 of N=3), returning the
highest-versioned value.  Since W + R &gt; N, quorum mode guarantees that
reads always see the latest completed write:</p>

<pre class="code-caching"><code>// Quorum: wait for 1 peer ack (W=2 of N=3)
let peers = get_peers(&amp;own_addr).await;
let mut acks = 0;
for peer in &amp;peers {
    if acks &gt;= 1 { break; }
    let result = caching::replicate_set(
        peer, key.clone(), value.clone(), ttl_secs, version,
    ).await;
    if !result.starts_with("ERROR") {
        acks += 1;
    }
}</code></pre>

<h3>Strong Consistency</h3>

<p>Writes wait for <em>all</em> peer acks (W=N) before returning.  Reads query all
peers (R=N).  This provides the strongest guarantees but the highest
latency, since every operation must contact every replica.  A single slow
or unavailable replica blocks the entire operation:</p>

<pre class="code-caching"><code>// Strong: wait for ALL peer acks (W=N)
let peers = get_peers(&amp;own_addr).await;
for peer in &amp;peers {
    let _ = caching::replicate_set(
        peer, key.clone(), value.clone(), ttl_secs, version,
    ).await;
}</code></pre>

<h3>Runtime Mode Switching</h3>

<p><span class="sidenote"><strong><code>caching/src/lib.rs</code></strong></span>
The consistency mode can be changed at runtime via the <code>MODE_PROCEDURE</code>
(id=7).  Sending an empty mode string returns the current mode; sending a
mode name switches to it.  This allows operators to dynamically trade off
between latency and consistency without restarting the service:</p>

<pre class="code-caching"><code>pub const MODE_PROCEDURE: ProcedureId = 7;

#[derive(Debug, Serializable, Deserializable)]
pub struct ModeArgs {
    pub mode: String,
}

#[derive(Debug, Serializable, Deserializable)]
pub struct ModeResult {
    pub mode: String,
}</code></pre>

<p>The <a href="/dashboard/consistency">consistency dashboard</a> uses this procedure
to display and control the mode.  In practice, an operator might switch to
strong consistency during a data migration, then return to eventual mode
for normal operation.</p>

<h2>Trade-offs</h2>

<p>The CAP theorem formalizes the fundamental tension: a distributed system can
provide at most two of three properties &mdash; Consistency, Availability, and
Partition tolerance.  Since network partitions are unavoidable, the practical
choice is between consistency (reject requests during partitions) and
availability (serve potentially stale data during partitions).</p>

<p>Our system makes this trade-off configurable at two levels.  Storage
defaults to quorum consistency (W=2, R=2) because durable data demands
stronger guarantees.  Caching defaults to eventual consistency because
stale cache entries are merely a performance issue, not a correctness
issue &mdash; the backing store remains the source of truth.</p>

<p>The quorum parameters W and R can be tuned independently.  Setting R=1
(read from local only) maximizes read speed at the cost of potentially
stale reads.  Setting W=3 (write to all replicas) maximizes write
durability but makes writes vulnerable to a single slow replica.  The
invariant W + R &gt; N must hold for reads to always see the latest write.</p>

<p>Version-based conflict resolution has an important limitation: it
resolves conflicts by picking the highest version, which means one write
always &ldquo;wins&rdquo; and the other is silently discarded.  For simple
key-value stores this is acceptable, but applications that need to merge
concurrent updates (like collaborative editors) require more sophisticated
techniques like CRDTs (Conflict-free Replicated Data Types) or
application-level merge functions.</p>

<p>Visit the <a href="/dashboard/consistency">consistency dashboard</a> to change
modes and observe the effect on latency and behavior.</p>

<p>Cross-region latency makes these consistency trade-offs even more consequential.
When peers span continents, strong consistency may add hundreds of milliseconds
to every operation.  <a href="/chapter/global-distribution">Chapter 24: Global
Distribution</a> examines how our system uses local quorum for fast writes and
asynchronous replication for cross-region consistency.</p>
"##
}

pub fn chapter_placement() -> &'static str {
    r##"
<h1>Chapter 23: Placement</h1>

<p><span class="newthought">Placement determines</span> where in the physical infrastructure
a piece of data or a computation should live.  Good placement reduces
latency (by putting data close to its consumers), improves reliability
(by spreading replicas across failure domains), and optimizes resource
usage (by balancing load across machines).</p>

<p>Data placement strategies include <em>hashing</em> (using a hash function to
deterministically assign data to servers), <em>range partitioning</em>
(assigning contiguous key ranges to servers), and <em>directory-based</em>
placement (consulting a lookup service for each key).  Our
<a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> service acts as a simple directory for service placement,
though it operates at the service level rather than the data level.</p>

<p>Consistent hashing is a particularly important technique for data
placement.  It assigns both servers and keys to positions on a virtual
ring, and each key is stored on the next server clockwise on the ring.
When a server is added or removed, only a fraction of keys need to be
reassigned, minimizing data movement.</p>

<p>Placement must account for failure domains: the physical or logical
boundaries within which failures are correlated.  A rack failure affects
all servers in the rack; a power failure affects all racks in a power
domain; a building failure affects all power domains in the building.
Placing all replicas of critical data in the same rack defeats the
purpose of replication.</p>

<p>The region is the largest failure domain.  Distributing across geographic
regions protects against disasters that affect an entire datacenter.
<a href="/chapter/global-distribution">Chapter 24: Global Distribution</a>
describes how our system runs a full stack in each region, with federated
discovery and WAL-based replication bridging the gap.</p>
"##
}

pub fn chapter_global_distribution() -> &'static str {
    r##"
<h1>Chapter 24: Global Distribution</h1>

<p><span class="newthought">A system that runs</span> in a single datacenter is a system with a
single point of failure.  No matter how many replicas you run, no matter how
carefully you design your failover, a backhoe through the fiber, a power grid
outage, or a cooling system failure can take everything offline at once.
Global distribution &mdash; running the full system in multiple geographic
regions &mdash; is how planetary scale systems survive regional failures and
serve users with low latency worldwide.</p>

<h2>Why Distribute</h2>

<p>Three forces drive global distribution.  <em>Latency</em>: light takes 74ms to
travel from San Francisco to New York and back, and 165ms to Amsterdam.  Users
notice.  Running a copy of the system in each region means most requests are
served locally.  <em>Availability</em>: independent infrastructure in separate
regions means a failure in one region does not affect the others.  If the SFO
region goes dark, NYC and AMS continue serving traffic.  <em>Data sovereignty</em>:
some jurisdictions require that data about their citizens remain within their
borders.  A multi-region architecture makes compliance possible.</p>

<h2>Full Stack Per Region</h2>

<p>Our approach is simple: each region runs the complete system.
<a href="/chapter/discovery" class="sys" style="color:#F7B731">Discovery</a>,
<a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a>,
<a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a>,
<a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a>,
<a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a>,
<a href="/chapter/scheduling" class="sys" style="color:#FF6B35">scheduling</a> &mdash;
every service runs in every region.  Local requests never leave the region.
Only two things cross regional boundaries: storage replication (so data
written in one region eventually appears in others) and cache invalidation
(so stale entries are purged everywhere).</p>

<span class="sidenote">This is sometimes called an &ldquo;active-active&rdquo; multi-region
deployment: every region can handle reads and writes independently, with
asynchronous replication keeping them in sync.</span>

<h2>The WireGuard Mesh</h2>

<p>The regions communicate over a private WireGuard mesh network.  Each region
has a WireGuard IP on a shared <code>10.0.0.0/24</code> subnet:</p>

<pre class="code-block"><code>SFO   10.0.0.1
NYC   10.0.0.2
AMS   10.0.0.3</code></pre>

<span class="sidenote">WireGuard provides encrypted, authenticated tunnels with
minimal overhead.  The mesh topology means every region can reach every other
region directly, without routing through a hub.  See
<a href="/chapter/network">Chapter 31: Network</a> for more on network
overlays.</span>

<p>This private overlay network means services can bind to <code>0.0.0.0</code>
and accept connections from both local services (via <code>127.0.0.1</code>)
and remote regions (via the WireGuard interface).  The <code>BIND_HOST</code>
environment variable controls the listen address, and <code>region.env</code>
configures the per-region settings:</p>

<pre class="code-block"><code># region.env (per droplet)
REGION=sfo
DISCOVERY_PEERS=10.0.0.2:10200,10.0.0.3:10200</code></pre>

<h2>Federated Discovery</h2>

<p>The key insight of our multi-region architecture is that
<a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a>
itself becomes federated.  Each region's discovery instance maintains two
registries: a <em>local</em> registry of services that registered directly
(the same registry from <a href="/chapter/discovery">Chapter 5</a>), and a
<em>federated</em> registry of services forwarded from peer discovery instances
in other regions.</p>

<p>A background task runs every five seconds.  It collects all locally-registered
services, rewrites their <code>127.0.0.1</code> addresses to the region's
WireGuard IP, and forwards these registrations to each peer discovery instance
using a <code>FEDERATED_REGISTER</code> RPC.  The remote discovery stores these
in its federated registry with the same staleness-based expiry used for local
entries.</p>

<span class="sidenote">This is a gossip-like protocol: each discovery instance
tells its peers about the services it knows.  Stale entries are cleaned up
automatically, so if a region goes offline its services disappear from all
registries within seconds.</span>

<p>The result is two complementary views.  <code>discovery::list("storage")</code>
returns all storage instances across all regions &mdash; useful for cache
invalidation, which must propagate globally.
<code>discovery::list_local("storage")</code> returns only the current region's
instances &mdash; useful for quorum operations, where cross-region latency would
make consensus impractically slow.</p>

<h2>WAL Tailer</h2>

<p>Cross-region storage replication is handled by the <em>WAL tailer</em>, a
lightweight service that reads the write-ahead log files produced by local
<a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a>
instances and replays them to remote storage instances.  It runs one instance
per region.</p>

<p>The tailer discovers local storage instances via
<code>discovery::list_local("storage")</code> and remote instances by
subtracting the local set from <code>discovery::list("storage")</code>.  For
each local instance, it maintains a byte offset into the WAL file and
periodically reads new entries:</p>

<pre class="code-block"><code>// WAL entry format (same as storage engine)
VPUT key=value@version
VDEL key@version</code></pre>

<p>Each new entry is sent to every remote storage instance via
<code>storage::replicate_put()</code> or <code>storage::replicate_delete()</code>.
The versioned replication protocol ensures idempotency: if an entry has already
been applied (because its version is older than the current version for that
key), the remote instance simply ignores it.</p>

<span class="sidenote">When storage compaction occurs (the WAL is truncated and
a snapshot is written), the tailer detects the file size decrease, reads the
snapshot entries, and resends them.  This is safe because replicated writes are
idempotent.</span>

<h2>Cache Invalidation Across Regions</h2>

<p>The <a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a>
service already supports multiple consistency modes: eventual, quorum, and
strong (see <a href="/chapter/consistency">Chapter 22</a>).  With federated
discovery, <code>discovery::list("caching")</code> now returns cache instances
from all regions.  The existing replication logic &mdash; which propagates
<code>SET</code> and <code>DELETE</code> operations to peers &mdash; automatically
extends across regions.</p>

<p>In eventual mode, cache writes are replicated asynchronously to all peers
including those in remote regions.  In quorum mode, the write waits for one
peer acknowledgment, which will typically come from a local peer (since
local responses arrive first).  In strong mode, the write waits for all peers,
including remote ones &mdash; which means it pays the full cross-region latency
penalty.</p>

<h2>Region Configuration</h2>

<p>Each region is configured via a <code>region.env</code> file that is sourced
at startup.  The only required setting is <code>DISCOVERY_PEERS</code> &mdash;
the WireGuard addresses of the other regions' discovery instances.  This single
variable bootstraps the entire cross-region topology:</p>

<pre class="code-block"><code># SFO (10.0.0.1) — peers are NYC and AMS
REGION=sfo
DISCOVERY_PEERS=10.0.0.2:10200,10.0.0.3:10200

# NYC (10.0.0.2) — peers are SFO and AMS
REGION=nyc
DISCOVERY_PEERS=10.0.0.1:10200,10.0.0.3:10200

# AMS (10.0.0.3) — peers are SFO and NYC
REGION=ams
DISCOVERY_PEERS=10.0.0.1:10200,10.0.0.2:10200</code></pre>

<span class="sidenote">Compare this with the previous approach, which required
separate <code>STORAGE_PEERS</code> and <code>CACHE_PEERS</code> environment
variables listing every remote instance of every service.  Federated discovery
reduces the configuration surface to a single variable per region.</span>

<h2>Latency Trade-offs</h2>

<p>The inter-region latencies in our three-region deployment are roughly:</p>

<pre class="code-block"><code>SFO ↔ NYC    ~74ms RTT
SFO ↔ AMS   ~165ms RTT
NYC ↔ AMS    ~92ms RTT</code></pre>

<p>These latencies make cross-region quorum consensus impractical for most
workloads.  A storage write with <code>W=2</code> quorum that includes a
remote peer would add 74&ndash;165ms to every write.  Instead, our architecture
uses local quorum for consistency within a region (fast, sub-millisecond) and
asynchronous replication across regions (eventually consistent, but no latency
penalty on the write path).</p>

<p>This is the fundamental trade-off of global distribution: you can have
strong consistency across regions or low-latency writes, but not both.  Our
system chooses low-latency writes with eventual cross-region consistency.
For workloads that require stronger guarantees, the caching layer's strong
consistency mode is available &mdash; at the cost of cross-region latency on
every operation.</p>
"##
}

pub fn chapter_traffic() -> &'static str {
    r##"
<h1>Chapter 25: Traffic</h1>

<p><span class="newthought">Traffic is the lifeblood</span> of a distributed system.
Understanding traffic patterns &mdash; when requests arrive, where they come
from, and what they ask for &mdash; is essential for capacity planning,
performance optimization, and anomaly detection.</p>

<p>Traffic patterns exhibit several common characteristics.  <em>Diurnal
patterns</em> follow the daily rhythm of human activity, with peaks during
waking hours and valleys overnight.  <em>Weekly patterns</em> distinguish
weekdays from weekends.  <em>Seasonal patterns</em> reflect events like
holidays, product launches, and marketing campaigns.  Understanding
these patterns allows operators to pre-provision capacity and schedule
maintenance during low-traffic periods.</p>

<p>Traffic management techniques include <em>rate limiting</em> (capping the
number of requests a client can make per unit of time), <em>throttling</em>
(slowing down requests rather than rejecting them), and <em>admission
control</em> (rejecting requests at the edge before they consume backend
resources).  These techniques protect the system from overload, whether
from organic growth, flash crowds, or malicious attacks.</p>

<p>The <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service plays a crucial role in traffic management
by providing real-time visibility into request rates, error rates, and
latency distributions.  Sudden changes in these metrics may indicate a
traffic surge, a dependency failure, or a bug in a newly deployed version.</p>
"##
}

pub fn chapter_faults() -> &'static str {
    r##"
<h1>Chapter 26: Faults</h1>

<p><span class="newthought">In a planetary scale computer,</span> faults are not exceptional
events &mdash; they are the norm.  With millions of components, something is
always failing somewhere: a disk is developing bad sectors, a network
switch is dropping packets, a server is running out of memory, a software
bug is causing a crash.  The question is not whether faults will occur
but how the system responds when they do.</p>

<p>Faults can be classified by their scope and duration.  <em>Transient faults</em>
are brief: a momentary network glitch, a garbage collection pause, a
brief CPU spike.  These are best handled with retries, as the fault
typically resolves itself.  <em>Intermittent faults</em> recur unpredictably:
a flaky disk, a network link with packet loss, a service with a memory
leak.  These require detection and remediation.  <em>Permanent faults</em>
are lasting: a dead disk, a failed server, a corrupted dataset.  These
require replacement or recovery.</p>

<p>Fault tolerance is built through redundancy at every level.  Data is
replicated across multiple servers (as in our <a href="/chapter/consensus" class="sys" style="color:#06D6A0">consensus</a> system).
Services run on multiple machines (as managed by our <a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a>
service).  Network paths are duplicated.  Power supplies have backup.
The goal is to ensure that no single fault &mdash; and ideally no combination
of two concurrent faults &mdash; causes a user-visible failure.</p>

<p>Detection is as important as tolerance.  Our <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service
detects faults through heartbeat timeouts and metric anomalies.  The
faster a fault is detected, the faster it can be mitigated &mdash; whether
by automated failover or human intervention.</p>
"##
}

pub fn chapter_outages() -> &'static str {
    r##"
<h1>Chapter 27: Outages</h1>

<p><span class="newthought">An outage</span> is the visible consequence of faults that
overwhelm the system's fault tolerance.  When enough components fail
simultaneously, or when a cascading failure propagates through
dependencies, the system can no longer serve its users.  Outages are the
most consequential events in the life of a distributed system.</p>

<p>Outages have many causes.  <em>Hardware failures</em> take down individual
servers or entire racks.  <em>Software bugs</em> can cause all instances of a
service to crash simultaneously.  <em>Configuration errors</em> can misconfigure
routing, security, or capacity parameters.  <em>Dependency failures</em> can
cascade when a failed service causes its dependents to queue up and
eventually fail.  <em>Overload</em> can occur when traffic exceeds the
system's capacity.</p>

<p>The impact of an outage depends on its <em>scope</em> (how many users are
affected), <em>duration</em> (how long it lasts), and <em>severity</em> (whether
data is lost or merely unavailable).  A one-minute partial outage
affecting 1% of users is very different from a one-hour complete outage
with data loss.  Incident classification systems help organizations
triage and respond to outages appropriately.</p>

<p>The most important lesson from outages is that they should be studied,
not just survived.  Post-incident reviews (blameless postmortems) identify
the root causes, contributing factors, and remediation actions that will
prevent similar outages in the future.  The insights from these reviews,
accumulated over time, become the institutional knowledge that makes a
system more resilient.</p>
"##
}

pub fn chapter_resources() -> &'static str {
    r##"
<h1>Chapter 28: Resources</h1>

<p><span class="newthought">The planetary scale computer</span> runs on physical resources:
compute (processors that execute instructions), memory (fast storage for
active data), persistent storage (slow storage for durable data), and
network (links that connect everything together).  Understanding these
resources &mdash; their characteristics, limits, and costs &mdash; is essential
for designing efficient systems.</p>

<p>Each resource type has a hierarchy of speed and capacity.  For compute:
registers, L1 cache, L2 cache, L3 cache, main memory, and disk, each
step being roughly an order of magnitude slower and larger than the
previous.  For storage: NVMe SSDs, SATA SSDs, spinning disks, and tape,
spanning from microseconds to seconds in access time.  For network:
loopback, local network, data center network, wide area network, and
intercontinental links, spanning from microseconds to hundreds of
milliseconds in latency.</p>

<p>Resource costs differ by orders of magnitude.  A CPU cycle costs
effectively nothing; a disk I/O costs milliseconds; a cross-continent
network round trip costs hundreds of milliseconds.  Our <a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a>
service exploits this hierarchy by keeping frequently accessed data in
memory (nanoseconds) rather than fetching it from storage (milliseconds)
or a remote service (tens of milliseconds).</p>
"##
}

pub fn chapter_servers() -> &'static str {
    r##"
<h1>Chapter 29: Servers</h1>

<p><span class="newthought">A server</span> is the basic unit of compute in a planetary
scale computer.  Modern servers pack enormous capability into a compact
form factor: dozens of CPU cores, hundreds of gigabytes of memory,
terabytes of storage, and network links capable of hundreds of gigabits
per second.  Understanding server architecture is important for sizing
workloads and understanding performance characteristics.</p>

<p>Servers in a data center are typically rack-mounted in standard 19-inch
racks.  A single rack might hold 40 or more servers, along with network
switches, power distribution, and cable management.  The density of a
rack &mdash; both in compute power and in power consumption &mdash; is a key
constraint on data center design.</p>

<p>Server selection involves balancing many dimensions: CPU cores versus
clock speed, memory capacity versus bandwidth, storage capacity versus
IOPS, and network bandwidth versus latency.  Different workloads have
different profiles: a <a href="/chapter/caching" class="sys" style="color:#7209B7">caching</a> service needs large memory and fast
network but little storage; a <a href="/chapter/storage" class="sys" style="color:#5E60CE">storage</a> service needs large disks
and durable writes but fewer CPU cores; a compute-intensive service
needs many fast cores.</p>

<p>The trend toward heterogeneous compute &mdash; adding GPUs, FPGAs, and
custom accelerators alongside CPUs &mdash; adds another dimension to server
selection.  Workloads like machine learning training benefit enormously
from GPU acceleration, while cryptographic operations can be offloaded
to dedicated hardware.</p>
"##
}

pub fn chapter_buildings() -> &'static str {
    r##"
<h1>Chapter 30: Buildings</h1>

<p><span class="newthought">Data centers</span> are the physical homes of planetary scale
computers.  A modern data center is an engineering marvel: a building
designed from the ground up to house, power, cool, and connect thousands
of servers operating at high utilization around the clock.</p>

<p>The design of a data center is driven by several constraints.  Power is
the primary cost and the primary limitation &mdash; a large data center may
consume as much electricity as a small city.  Cooling must dissipate the
heat generated by thousands of servers, using techniques ranging from
traditional air conditioning to liquid cooling and free-air cooling in
cold climates.  Network connectivity must provide high-bandwidth, low-latency
links to other data centers and to the broader Internet.</p>

<p>Data center location is a strategic decision.  Factors include proximity
to users (for low latency), proximity to renewable energy (for
sustainability), risk of natural disasters (earthquakes, floods,
hurricanes), political stability, and real estate costs.  Major cloud
providers operate data centers on every inhabited continent, with each
<em>region</em> typically containing multiple data centers (called
<em>availability zones</em>) for redundancy.</p>
"##
}

pub fn chapter_network() -> &'static str {
    r##"
<h1>Chapter 31: Network</h1>

<p><span class="newthought">The network</span> is the nervous system of the planetary scale
computer.  It connects servers within a rack, racks within a data center,
data centers within a region, and regions across the globe.  The
characteristics of the network &mdash; bandwidth, latency, reliability, and
cost &mdash; fundamentally shape the design of distributed systems.</p>

<p>Within a data center, networks are typically structured as a multi-tier
hierarchy or a spine-leaf topology.  Top-of-rack switches connect servers
within a rack.  Aggregation switches connect racks within a cluster.
Core switches connect clusters within a data center.  At each tier,
bandwidth is aggregated and the failure domain widens.</p>

<p>Between data centers, network connections range from dedicated fiber links
(for data centers in the same metropolitan area) to leased circuits and
the public Internet (for intercontinental connectivity).  Latency between
data centers in the same region is typically 1-5 milliseconds; between
continents, 50-200 milliseconds.  These latency constraints directly
affect system design: strong consistency across continents is expensive
because every write must wait for a round trip.</p>

<p>Network failures are a major source of outages in distributed systems.
Unlike server failures (which are typically independent), network failures
can partition large groups of servers simultaneously, creating split-brain
scenarios that consensus protocols like our <a href="/chapter/consensus" class="sys" style="color:#06D6A0">consensus</a> service are
designed to handle.</p>

<p>Our three-region deployment uses a WireGuard mesh to create a private overlay
network connecting SFO, NYC, and AMS.
<a href="/chapter/global-distribution">Chapter 24: Global Distribution</a>
describes how this mesh enables federated discovery and cross-region storage
replication.</p>
"##
}

pub fn chapter_power() -> &'static str {
    r##"
<h1>Chapter 32: Power</h1>

<p><span class="newthought">Power is the ultimate</span> resource constraint of a planetary
scale computer.  Everything &mdash; computation, storage, networking, and
cooling &mdash; requires electricity.  The amount of power a data center can
deliver to its servers determines the maximum compute capacity of the
facility.</p>

<p>Power delivery in a data center involves multiple levels of redundancy.
Utility power from the electrical grid is the primary source.  Uninterruptible
power supplies (UPS) provide battery backup for the seconds or minutes it
takes for diesel generators to start during a grid outage.  Generators
can power the facility for days or weeks, limited only by fuel supply.</p>

<p>Power Usage Effectiveness (PUE) measures the ratio of total data center
power to the power consumed by computing equipment.  A PUE of 2.0 means
that for every watt used for computing, another watt is used for cooling,
lighting, and other overhead.  Modern data centers achieve PUEs of 1.1 to
1.2, meaning that the vast majority of power goes directly to computation.
Techniques like hot/cold aisle containment, free-air cooling, and liquid
cooling contribute to these improvements.</p>

<p>The environmental impact of data center power consumption is increasingly
important.  Major operators are investing in renewable energy sources and
carbon offsets.  Locating data centers near renewable energy sources &mdash;
hydroelectric dams, wind farms, solar installations &mdash; is becoming a key
factor in site selection.</p>
"##
}

pub fn chapter_infra_management() -> &'static str {
    r##"
<h1>Chapter 33: Management</h1>

<p><span class="newthought">Managing the physical infrastructure</span> of a planetary scale
computer is an enormous operational challenge.  With thousands of servers
across multiple facilities, hardware failures are a daily occurrence.
Disks fail, memory develops errors, network cards malfunction, and
entire servers become unresponsive.  Effective infrastructure management
requires automation at every step: detection, diagnosis, remediation,
and replacement.</p>

<p>Automated hardware management systems track the inventory, health, and
lifecycle of every component.  When a disk shows signs of impending failure
(through SMART metrics), the system automatically drains traffic from the
affected server, schedules a replacement, and migrates data to healthy
replicas.  When a server becomes unresponsive, the system power-cycles it
and, if it fails to recover, marks it for physical repair.</p>

<p>Firmware and BIOS updates must be rolled out across thousands of servers
with minimal disruption.  This requires coordination with the scheduling
system to drain work from servers before updating them, and validation
that the update did not introduce regressions.  The scale of these
operations makes manual management impossible &mdash; everything must be
automated and auditable.</p>
"##
}

pub fn chapter_maintenance() -> &'static str {
    r##"
<h1>Chapter 34: Maintenance</h1>

<p><span class="newthought">Maintenance is the ongoing work</span> required to keep the
planetary scale computer healthy.  Unlike a personal computer that can be
taken offline for maintenance, a planetary scale computer must be
maintained while it continues to serve users.  This requires careful
coordination between maintenance activities and the services running on
the infrastructure.</p>

<p>Planned maintenance includes hardware replacement (swapping failed
components), software updates (operating system patches, firmware updates),
and capacity expansion (adding new servers and racks).  Each maintenance
activity must be scheduled to minimize impact on running services, using
the scheduling and placement systems to move work away from servers that
need maintenance.</p>

<p>Unplanned maintenance &mdash; responding to unexpected failures &mdash; is the more
challenging scenario.  When a server fails unexpectedly, the
<a href="/chapter/discovery" class="sys" style="color:#F7B731">discovery</a> and <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> services detect the failure, the
<a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> service stops sending traffic to it, and the <a href="/chapter/consensus" class="sys" style="color:#06D6A0">consensus</a>
system elects a new leader if the failed server was one.  The failed
server is then repaired or replaced as part of the normal maintenance
cycle.</p>
"##
}

pub fn chapter_edges() -> &'static str {
    r##"
<h1>Chapter 35: Edges</h1>

<p><span class="newthought">The edge of the network</span> is where the planetary scale
computer meets its users.  Edge computing moves computation and data
closer to users &mdash; into local data centers, points of presence (PoPs),
cell towers, and even end-user devices.  This reduces latency, decreases
backbone traffic, and enables applications that require real-time
responsiveness.</p>

<p>Content Delivery Networks (CDNs) are the most common form of edge
infrastructure.  A CDN caches static content (images, videos, scripts)
at hundreds of edge locations around the world, serving users from the
nearest location.  This transforms a 200-millisecond cross-continent
fetch into a 5-millisecond local cache hit.</p>

<p>Beyond caching, edge computing can run application logic close to users.
Edge functions (similar to cloud functions but running at edge locations)
can handle authentication, personalization, and request routing without
a round trip to the origin data center.  This blurs the line between the
edge and the data center, creating a continuum of compute locations
from the user's device to the central facility.</p>

<p>The challenge of edge computing is managing consistency across many
small locations.  Each edge location is a potential source of stale data
or conflicting state.  The techniques we have studied &mdash; caching with
TTLs, eventual consistency, and consensus for critical data &mdash; all apply
at the edge, but the trade-offs shift toward availability and low latency.</p>

<p>Our three-region deployment (SFO, NYC, AMS) is a practical form of edge-like
distribution &mdash; not hundreds of locations, but enough to cover major
population centers with low-latency access.
<a href="/chapter/global-distribution">Chapter 24: Global Distribution</a>
shows how the full stack runs in each region with asynchronous cross-region
replication.</p>
"##
}

pub fn chapter_site_events() -> &'static str {
    r##"
<h1>Chapter 36: Site Events</h1>

<p><span class="newthought">A site event</span> is a significant incident that affects the
availability, performance, or correctness of the planetary scale computer.
Site events range from minor (a brief latency spike affecting a single
service) to major (a complete data center outage lasting hours).  How
an organization detects, responds to, and learns from site events
determines the long-term reliability of its systems.</p>

<p>The lifecycle of a site event has distinct phases: detection, triage,
mitigation, resolution, and post-incident review.  Detection should be
automated through the <a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service &mdash; alerts fire when health
metrics exceed thresholds or when anomalies are detected in traffic
patterns.  The goal is to detect incidents before users report them.</p>

<p>Triage determines the severity and scope of the incident.  Is it
affecting all users or a subset?  Is data at risk?  Is the incident
spreading?  The answers to these questions determine the response: who
is paged, what communication goes out, and what immediate actions are
taken.  Clear severity definitions and escalation procedures prevent
confusion during high-stress incidents.</p>

<p>Mitigation focuses on restoring service as quickly as possible, even if
the root cause is not yet understood.  Common mitigation actions include
rolling back a recent deployment, failing over to a healthy replica,
shedding load to reduce pressure on overloaded components, or disabling
a misbehaving feature.  Root cause analysis comes later, during the
post-incident review.</p>
"##
}

pub fn chapter_detection() -> &'static str {
    r##"
<h1>Chapter 37: Detection</h1>

<p><span class="newthought">The first step</span> in managing any incident is knowing
that something is wrong.  Detection is the bridge between a silent failure
and an active response.  The faster an organization detects an incident,
the smaller the blast radius and the shorter the recovery time.  Detection
latency &mdash; the time between a problem starting and someone being alerted
&mdash; is one of the most important reliability metrics a team can track.</p>

<p>Automated detection relies on the signals collected by the
<a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service: heartbeats, latency percentiles, error rates,
saturation metrics, and business-level indicators like order completion
rates.  Alert thresholds define when these signals cross from normal
variation into actionable territory.  Setting thresholds too low produces
alert fatigue; setting them too high means incidents go unnoticed.
Anomaly detection &mdash; statistical models that learn normal patterns and
flag deviations &mdash; can complement static thresholds by catching novel
failure modes.</p>

<p>Not all incidents are caught by automated systems.  User reports,
support tickets, and social media mentions are valuable detection
channels, especially for problems that affect the user experience in
ways that internal metrics do not capture.  A robust detection strategy
combines automated monitoring with human observation, ensuring that no
category of failure goes unnoticed for long.</p>

<p>On-call engineers are the human link in the detection chain.  When an
alert fires, the on-call responder must acknowledge it, assess whether
it represents a real incident, and decide on next steps.  The
<a href="/chapter/routing" class="sys" style="color:#2A9D8F">routing</a> service can automatically shift traffic away from
unhealthy backends while the responder investigates, buying time without
requiring immediate human intervention.</p>
"##
}

pub fn chapter_escalation() -> &'static str {
    r##"
<h1>Chapter 38: Escalation</h1>

<p><span class="newthought">Once an incident is detected,</span> the next critical
decision is how urgently to respond and who needs to be involved.
Escalation is the process of raising an incident to the right people
at the right time.  Under-escalation leaves a serious problem in the
hands of too few responders; over-escalation wastes attention and
breeds cynicism about the severity system.</p>

<p>Severity levels provide a shared vocabulary for incident urgency.  A
common scheme uses four tiers: SEV1 for total service outages affecting
all users, SEV2 for major degradation affecting a large subset, SEV3
for partial issues with limited user impact, and SEV4 for minor problems
with no immediate user impact.  Each severity level maps to a specific
response: who gets paged, how quickly they must respond, and what
communication is expected.  Clear definitions prevent debates about
severity during the stress of an active incident.</p>

<p>The incident commander role is the cornerstone of escalation.  This
person owns the incident response: they coordinate responders, delegate
investigation tasks, decide when to escalate further, and ensure that
communication flows to stakeholders.  The incident commander does not
need to be the most senior engineer &mdash; they need to be someone who can
organize a response calmly under pressure.</p>

<p>Multi-service incidents, where a failure in one system cascades through
others, require cross-team coordination.  The
<a href="/chapter/monitoring" class="sys" style="color:#B5179E">monitoring</a> service can reveal which services are affected, and
the <a href="/chapter/site-events" class="sys" style="color:#555">site event</a> framework provides the structure for bringing
the right teams together.  Paging policies should account for time zones,
on-call rotations, and backup responders to avoid single points of failure
in the human response chain.</p>
"##
}

pub fn chapter_root_causes() -> &'static str {
    r##"
<h1>Chapter 39: Root Causes</h1>

<p><span class="newthought">After an incident is mitigated,</span> the most important
work begins: understanding why it happened.  Root cause analysis goes
beyond identifying what broke to understanding the deeper systemic
conditions that allowed the failure to occur.  A server running out of
disk space is a proximate cause; the root cause might be that no capacity
alerting existed, or that log rotation was never configured for a new
service.</p>

<p>The "five whys" technique is a simple but effective method: ask why the
failure occurred, then ask why that condition existed, and repeat until
you reach a systemic cause.  The answer is rarely a single root cause.
Most incidents have multiple contributing factors &mdash; a latent bug, a
configuration gap, a monitoring blind spot &mdash; that align to produce a
failure.  Identifying all contributing factors is more valuable than
pinpointing one "root cause" to blame.</p>

<p>Blame is the enemy of learning.  If engineers fear punishment for
mistakes, they will hide information, and the organization loses its
ability to learn from failures.  Blameless post-incident reviews focus
on the system, not the individual.  The question is never "who caused
this?" but "what conditions allowed this to happen, and how do we change
the system so it cannot happen again?"  This mirrors the philosophy
behind the <a href="/chapter/faults" class="sys" style="color:#555">faults</a> and
<a href="/chapter/outages" class="sys" style="color:#555">outages</a> chapters: failures are
inevitable, and resilience comes from how we respond to them.</p>

<p>Documenting findings is essential.  A well-written post-incident review
captures the timeline, the contributing factors, the impact, and the
corrective actions.  These documents become part of the organization's
institutional memory, allowing future engineers to learn from past
incidents without having to experience them firsthand.</p>
"##
}

pub fn chapter_remediation() -> &'static str {
    r##"
<h1>Chapter 40: Remediation</h1>

<p><span class="newthought">Remediation is the work</span> of restoring a system to
full health after an incident.  It operates on three timescales:
immediate mitigation to stop the bleeding, short-term fixes to stabilize
the system, and long-term corrective actions to address root causes.
Each timescale requires different trade-offs between speed and
thoroughness.</p>

<p>Immediate mitigation prioritizes availability over perfection.  Rolling
back a bad deployment, failing over to a healthy replica, shedding
non-critical load, or disabling a misbehaving feature with a feature
flag &mdash; these actions restore service quickly even when the root cause
is not yet understood.  The techniques of
<a href="/chapter/degradation" class="sys" style="color:#555">degradation</a> and
<a href="/chapter/load-balancing" class="sys" style="color:#555">load balancing</a> are essential tools in
the mitigation toolkit.  A well-practiced team can mitigate most
incidents in minutes.</p>

<p>Short-term fixes address the immediate technical cause.  If a memory
leak crashed a service, the short-term fix patches the leak.  If a
configuration change caused a cascading failure, the short-term fix
reverts the configuration and adds validation.  These fixes are deployed
through the normal <a href="/chapter/release" class="sys" style="color:#555">release</a> process, with extra scrutiny
given the recent incident.</p>

<p>Long-term corrective actions come from the post-incident review and
target the systemic conditions that allowed the incident to occur.  These
might include adding monitoring for a previously unobserved failure mode,
improving capacity planning, or redesigning a component to eliminate a
class of failures.  Action items must be tracked to completion &mdash; an
action item that is assigned but never finished provides no protection
against recurrence.  Verifying that repairs actually work, through testing
and monitoring, closes the remediation loop.</p>
"##
}

pub fn chapter_prevention() -> &'static str {
    r##"
<h1>Chapter 41: Prevention</h1>

<p><span class="newthought">The best incident</span> is the one that never happens.
Prevention shifts the focus from reactive response to proactive
resilience.  While no system can prevent all failures, deliberate
engineering practices can eliminate entire classes of incidents and
reduce the severity of those that do occur.</p>

<p>Chaos engineering is the practice of deliberately injecting failures
into production systems to discover weaknesses before they cause real
incidents.  Game days &mdash; scheduled exercises where teams simulate
large-scale failures &mdash; build both technical resilience and human
readiness.  Pre-mortems invert the post-incident review: before
launching a new system, the team imagines it has already failed
catastrophically and works backward to identify what could go wrong.
These practices, combined with thorough design reviews, catch problems
that testing alone cannot reveal.</p>

<p>Automation of manual toil is a powerful preventive measure.  Every
manual step in a runbook is an opportunity for human error under stress.
Automating routine operational tasks &mdash; certificate rotation, capacity
scaling, failover procedures &mdash; removes these error-prone steps and
frees engineers to focus on novel problems.  Defense in depth ensures
that no single failure can cascade into a site-wide outage: redundant
components, circuit breakers, bulkheads, and graceful degradation all
contribute to a system that bends rather than breaks.</p>

<p>Prevention is ultimately a cultural practice.  Organizations that
invest in <a href="/chapter/security" class="sys" style="color:#555">security</a> reviews,
<a href="/chapter/load-testing" class="sys" style="color:#555">load testing</a>, and blameless
post-incident processes build a culture where reliability is everyone's
responsibility.  Tracking incident recurrence classes &mdash; ensuring that
the same type of incident does not happen twice &mdash; is the strongest
signal that an organization is learning from its failures rather than
merely surviving them.</p>
"##
}

pub fn chapter_communication() -> &'static str {
    r##"
<h1>Chapter 42: Communication</h1>

<p><span class="newthought">During a site event,</span> communication is as important as
technical response.  Users need to know that a problem exists, that it
is being worked on, and when it is expected to be resolved.  Internal
teams need to coordinate their efforts, share findings, and avoid
duplicating work.  Effective communication can be the difference between
a well-managed incident and a chaotic one.</p>

<p>Internal communication during an incident typically uses a dedicated
channel (a chat room, a bridge call, or both) where all responders can
share observations and coordinate actions.  An <em>incident commander</em>
leads the response, delegating tasks, tracking progress, and making
decisions.  A <em>scribe</em> documents the timeline of events, actions taken,
and their outcomes, creating the raw material for the post-incident
review.</p>

<p>External communication requires balancing transparency with accuracy.
Premature statements about root causes can be wrong and erode trust.
Status page updates should state what is known (the scope and impact of
the incident), what is being done (the mitigation actions underway), and
when the next update will be (to set expectations).  It is better to say
"we are investigating" than to speculate about causes.</p>

<p>The post-incident review is the most valuable communication artifact.
Written as a blameless document, it describes the timeline, root causes,
contributing factors, and corrective actions.  These reviews, shared
across the organization, build institutional memory and prevent the
same class of incident from recurring.  The best engineering organizations
treat post-incident reviews not as bureaucratic overhead but as one of
their most important learning mechanisms.</p>
"##
}

pub fn afterword() -> &'static str {
    r##"
<h1>Afterword</h1>

<p><span class="newthought">We began this journey</span> with a simple observation: the Internet
has changed what it means to use a computer.  Over the course of forty-two
chapters, we have built &mdash; piece by piece &mdash; the machinery that makes
planetary scale computing possible.  From serialization formats and RPC
protocols to consensus algorithms, from caching layers and storage engines
to the physical infrastructure of servers, buildings, and power systems.</p>

<p>If there is one lesson I hope you take away, it is that these systems are
not magic.  They are the work of engineers making deliberate trade-offs
under constraints.  Every system we studied is a choice: consistency or
availability, latency or throughput, simplicity or flexibility.  The art
of systems engineering is not in finding the perfect answer but in
understanding which trade-off is right for the problem at hand.</p>

<p>The field is still young.  We are building the first generation of truly
planetary scale computers, and many of the hardest problems remain open.
How do we build systems that span jurisdictions with different privacy laws?
How do we make planetary scale computing sustainable in its energy consumption?
How do we ensure that these powerful systems serve all of humanity, not just
those who can afford to build them?</p>

<p>I wrote this book because I believe that understanding how these systems work
should not be a privilege reserved for engineers at the largest companies.
The techniques are not secret &mdash; they are earned through experience, shared
through papers and postmortems, and refined through years of operation.  My
hope is that this book has made some of that knowledge more accessible, and
that you will use it to build systems that are not only reliable and efficient
but also humane.</p>

<p>The planetary scale computer is not a destination.  It is an ongoing project,
built and rebuilt by each generation of engineers who inherit it.  I am glad
to have shared a small part of that project with you.</p>

<p class="attribution">&mdash; <em>Justin J. Meza, San Francisco, 2025</em></p>
"##
}

pub fn colophon() -> &'static str {
    r##"
<h1>Colophon</h1>

<p><span class="newthought">This book</span> was written and typeset using LaTeX with the
<code>tufte-book</code> document class, which provides the wide margins and
sidenote layout inspired by the work of Edward Tufte.  The text is set in
Palatino; code listings use a monospaced font rendered by the
<code>listings</code> package.</p>

<p>All of the services described in this book &mdash; normalization, RPC, discovery,
routing, echo, configuration, caching, storage, and monitoring &mdash; are
implemented in Rust and compile to standalone binaries.  The code is intended
to be read alongside the text.  Simplicity was preferred over production
hardening: the goal was to make each concept legible in a few hundred lines
of code rather than to build software ready for deployment.</p>

<p>The web edition of this book is itself one of the systems it describes.  The
frontend is a raw TCP HTTP server written in Rust with no external web
framework.  It serves the book's chapters, manages user highlights through the
storage and caching services, and presents a live dashboard showing the state
of the running system.  The entire constellation of services launches from a
single shell script.</p>

<p>Large language models were used extensively in the preparation of both the
manuscript and the code.  They served as drafting partners, code reviewers,
and tireless pair programmers.  The author remains responsible for all
errors, omissions, and questionable trade-offs.</p>
"##
}
