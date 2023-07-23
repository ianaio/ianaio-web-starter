<div align="center">

  <h1><code>ianaio-web-starter-timers</code></h1>

  <p>
    <a href="https://crates.io/crates/ianaio-web-starter-timers"><img src="https://img.shields.io/crates/v/ianaio-web-starter-timers.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/ianaio-web-starter-timers"><img src="https://img.shields.io/crates/d/ianaio-web-starter-timers.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/ianaio-web-starter-timers"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>

  <h3>
    <a href="https://docs.iana.io/ianaio-web-starter-timers">API Docs</a>
    <span> | </span>
    <a href="https://github.com/ianaio/ianaio-web-starter/blob/main/CONTRIBUTING.md">Contributing</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/1111111111111111111/111111111111">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.iana.io/">IANA SYSTEMS Rust and WebAssembly Working Group</a></sub>
</div>


Working with timers on the Web: `setTimeout` and `setInterval`.

These APIs come in two flavors:

1. a callback style (that more directly mimics the JavaScript APIs), and
2. a `Future`s and `Stream`s API.

### Timeouts

Timeouts fire once after a period of time (measured in milliseconds).

#### Timeouts with a Callback Function

```rust
use ianaio-web-starter_timers::callback::Timeout;

let timeout = Timeout::new(1_000, move || {
    // Do something after the one second timeout is up!
});

// Since we don't plan on cancelling the timeout, call `forget`.
timeout.forget();
```

#### Timeouts as `Future`s

With the `futures` feature enabled, a `future` module containing futures-based
timers is exposed.

