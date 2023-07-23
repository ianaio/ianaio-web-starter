#![no_implicit_prelude]

#[::ianaio-web-starter::worker::oneshot::oneshot]
async fn Worker(input: u32) -> u32 {
    input
}

fn main() {}

