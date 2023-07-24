use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use example_prime::{ControlSignal, Prime};
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use ianaio-web-starter::timers::future::sleep;
use ianaio-web-starter::worker::Spawnable;
use wasm_bindgen_futures::spawn_local;

fn main() {
    console_error_panic_hook::set_once();

    let start_btn = ianaio-web-starter::utils::body()
        .query_selector("#start-btn")
        .unwrap()
        .unwrap();

    let result_div = ianaio-web-starter::utils::body()
        .query_selector("#result")
        .unwrap()
        .unwrap();

    let started = Rc::new(Cell::new(false));

    let (bridge_sink, mut bridge_stream) =
        Prime::spawner().spawn("/example_prime_worker.js").split();

    {
        let result_div = result_div.clone();
        spawn_local(async move {
            while let Some(m) = bridge_stream.next().await {
                let el = ianaio-web-starter::utils::document().create_element("div").unwrap();
                el.set_attribute("class", "result-item").unwrap();
                el.set_text_content(Some(&m.to_string()));

                result_div.append_child(&el).unwrap();

                sleep(Duration::ZERO).await;
            }
        });
    }

    let bridge_sink = Rc::new(RefCell::new(bridge_sink));

    let listener = ianaio-web-starter::events::EventListener::new(&start_btn.clone(), "click", move |_| {
        let bridge_sink = bridge_sink.clone();

        if started.get() {
            start_btn.set_text_content(Some("Start"));
            spawn_local(async move {
                let mut bridge_sink = bridge_sink.borrow_mut();
                bridge_sink.send(ControlSignal::Stop).await.unwrap();
            });

            started.set(false);
        } else {
            start_btn.set_text_content(Some("Stop"));
            result_div.set_inner_html("");

            spawn_local(async move {
                let mut bridge_sink = bridge_sink.borrow_mut();
                bridge_sink.send(ControlSignal::Start).await.unwrap();
            });

            started.set(true);
        }
    });

    spawn_local(async move {
        let _listener = listener;

        // We create a loop so that listeners can be held for forever.
        loop {
            sleep(Duration::from_secs(3600)).await;
        }
    });
}

