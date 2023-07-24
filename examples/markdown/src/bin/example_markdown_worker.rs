use example_markdown::MarkdownWorker;

use iana-web-starter::worker::Registrable;

fn main() {
    console_error_panic_hook::set_once();

    MarkdownWorker::registrar().register();
}

