mod conn;
mod protocol;
mod socket;
mod ui;

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use crate::protocol::Request;

use cursive::logger;
use cursive::view::{Resizable, Scrollable};
use cursive::views::Canvas;
use cursive::views::Dialog;
use cursive::{Cursive, CursiveExt};

fn main() {
    logger::init();
    let mut siv = Cursive::default();
    let resp_content: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::with_capacity(50)));

    {
        let resp_content = Arc::clone(&resp_content);
        thread::spawn(move || {
            for _ in 0..50 {
                let resp = Request::from_method("ping").send("127.0.0.1:8080").unwrap();
                let text = resp.body::<String>().unwrap();
                resp_content.lock().unwrap().push(text);
                thread::sleep(Duration::from_secs(3));
            }
        });
    }

    siv.add_layer(Dialog::around(
        Canvas::new(resp_content)
            .with_draw(|resp_content, printer| {
                for (y, text) in resp_content.lock().unwrap().iter().enumerate() {
                    printer.print((0, y), text);
                }
            })
            .fixed_size((10, 50))
            .scrollable(),
    ));

    siv.add_global_callback('q', |s| s.quit());
    siv.set_fps(30);

    siv.run();
}
