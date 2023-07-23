<div align="center">

  <h1><code>ianaio-web-starter-net</code></h1>

  <p>
    <a href="https://crates.io/crates/ianaio-web-starter-net"><img src="https://img.shields.io/crates/v/ianaio-web-starter-net.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/ianaio-web-starter-net"><img src="https://img.shields.io/crates/d/ianaio-web-starter-net.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/ianaio-web-starter-net"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>

  <h3>
    <a href="https://docs.iana.io/ianaio-web-starter-net">API Docs</a>
    <span> | </span>
    <a href="https://github.com/ianaio/ianaio-web-starter/blob/main/CONTRIBUTING.md">Contributing</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/11111111111111111111/1111111111111">Chat</a>
  </h3>

<sub>Built with 🦀🕸 by <a href="https://rustwasm.iana.io/">IANA SYSTEMS Rust and WebAssembly Working Group</a></sub>
</div>

HTTP requests library for WASM Apps. It provides idiomatic Rust bindings for the `web_sys` [`fetch`](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API), [`WebSocket`](https://developer.mozilla.org/en-US/docs/Web/API/WebSocket) and [`EventSource`](https://developer.mozilla.org/en-US/docs/Web/API/EventSource) APIs.

## Examples

### HTTP

```rust
let resp = Request::get("/path")
    .send()
    .await
    .unwrap();
assert_eq!(resp.status(), 200);
```

### WebSocket

```rust
use ianaio-web-starter_net::websocket::{Message, futures::WebSocket};
use wasm_bindgen_futures::spawn_local;
use futures::{SinkExt, StreamExt};

let mut ws = WebSocket::open("wss://echo.websocket.org").unwrap();
let (mut write, mut read) = ws.split();

spawn_local(async move {
    write.send(Message::Text(String::from("test"))).await.unwrap();
    write.send(Message::Text(String::from("test 2"))).await.unwrap();
});

spawn_local(async move {
    while let Some(msg) = read.next().await {
        console_log!(format!("1. {:?}", msg))
    }
    console_log!("WebSocket Closed")
})
```

### EventSource

```rust
use ianaio-web-starter_net::eventsource::futures::EventSource;
use wasm_bindgen_futures::spawn_local;
use futures::{stream, StreamExt};

let mut es = EventSource::new("http://api.example.com/ssedemo.php").unwrap();
let stream_1 = es.subscribe("some-event-type").unwrap();
let stream_2 = es.subscribe("another-event-type").unwrap();

spawn_local(async move {
    let mut all_streams = stream::select(stream_1, stream_2);
    while let Some(Ok((event_type, msg))) = all_streams.next().await {
        console_log!(format!("1. {}: {:?}", event_type, msg))
    }
    console_log!("EventSource Closed");
})
```

