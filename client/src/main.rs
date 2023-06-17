mod conn;
mod models;
mod protocol;
mod socket;
mod ui;

// use std::sync::Arc;
// use std::sync::Mutex;
// use std::thread;
// use std::time::Duration;
//
// use conn::ShortConn;

fn main() {
    // let resp_content = Arc::new(Mutex::new(Vec::with_capacity(50)));
    //
    // {
    //     let resp_content = Arc::clone(&resp_content);
    //     thread::spawn(move || {
    //         for _ in 0..50 {
    //             let resp = ShortConn::builder("ping")
    //                 .try_build("127.0.0.1:8080")
    //                 .unwrap()
    //                 .send()
    //                 .unwrap();
    //             let text: String = resp.body().deserialize().unwrap();
    //             resp_content.lock().unwrap().push(text);
    //             thread::sleep(Duration::from_secs(3));
    //         }
    //     });
    // }

    ui::run();
}
