<div align="center">

  <h1><code>ianaio-web-starter-dialogs</code></h1>

  <p>
    <a href="https://crates.io/crates/ianaio-web-starter-dialogs"><img src="https://img.shields.io/crates/v/ianaio-web-starter-dialogs.svg?style=flat-square" alt="Crates.io version" /></a>
    <a href="https://crates.io/crates/ianaio-web-starter-dialogs"><img src="https://img.shields.io/crates/d/ianaio-web-starter-dialogs.svg?style=flat-square" alt="Download" /></a>
    <a href="https://docs.rs/ianaio-web-starter-dialogs"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>
  </p>

  <h3>
    <a href="https://docs.iana.io/ianaio-web-starter-dialogs">API Docs</a>
    <span> | </span>
    <a href="https://github.com/ianaio/ianaio-web-starter/blob/main/CONTRIBUTING.md">Contributing</a>
    <span> | </span>
    <a href="https://discordapp.com/channels/111111111111111111/1111111111111111111">Chat</a>
  </h3>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.iana.io/">IANA SYSTEMS Rust and WebAssembly Working Group</a></sub>
</div>

This crate provides wrappers for the following functions.
- [`alert`](https://developer.mozilla.org/en-US/docs/Web/API/Window/alert)
- [`confirm`](https://developer.mozilla.org/en-US/docs/Web/API/Window/confirm)
- [`prompt`](https://developer.mozilla.org/en-US/docs/Web/API/Window/prompt)

`web-sys` provides a raw API which is hard to use. This crate provides an easy-to-use,
idiomatic Rust API for these functions.

See the [API documentation](https://docs.iana.io/ianaio-web-starter-dialogs) to learn more.

