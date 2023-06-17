use std::env;
use std::io::BufReader;
use std::io::BufWriter;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use cursive::event::Key;
use cursive::logger;
use cursive::reexports::log;
use cursive::reexports::log::info;
use cursive::view::Nameable;
use cursive::view::{Resizable, Scrollable};
use cursive::views::Canvas;
use cursive::views::DebugView;
use cursive::views::EditView;
use cursive::views::LinearLayout;
use cursive::{Cursive, CursiveExt};

use crate::conn::persistent;
use crate::conn::ShortConn;
use crate::models::Echo;
use crate::models::PrivateMsg;
use crate::protocol::persistent::Message;
use crate::protocol::persistent::Method;
use crate::protocol::short::Status;
use crate::protocol::Body;
use crate::socket::persistent::read_message;
use crate::socket::persistent::write_message;

pub fn run() {
    logger::init();
    log::set_max_level(log::LevelFilter::Info);
    let mut siv = Cursive::default();

    info!("test");
    siv.add_layer(
        LinearLayout::horizontal()
            .child(DebugView::default().fixed_size((60, 15)))
            .child(chat()),
    );

    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.set_fps(30);

    siv.run();
}

fn chat() -> LinearLayout {
    let mut args = env::args();
    args.next();
    let my_uid: u32 = args.next().unwrap().parse().unwrap();
    let to_uid: u32 = args.next().unwrap().parse().unwrap();
    let mut short_conn = ShortConn::builder("persistent")
        .try_build("127.0.0.1:8080")
        .unwrap();
    let resp = short_conn.send().unwrap();
    assert_eq!(*resp.status(), Status::Ok);
    let socket = short_conn.into_socket();
    let (mut reader, mut writer) = persistent::split(socket);

    let chats = Arc::new(Mutex::new(Vec::<(u32, String)>::with_capacity(50)));

    {
        let chats = chats.clone();
        thread::spawn(move || {
            info!("socket reader thread started");
            while let Ok(msg) = read_message(BufReader::new(&mut reader)) {
                let body = msg.body();

                match msg.method() {
                    Method::Echo => {
                        let echo: Echo = body.deserialize().unwrap();
                        info!("uid={my_uid} content={:?}", &echo.content);
                        chats.lock().unwrap().push((my_uid, echo.content));
                    }
                    Method::Private => {
                        let pmsg: PrivateMsg = body.deserialize().unwrap();
                        info!("uid={to_uid} content={:?}", &pmsg.content);
                        chats.lock().unwrap().push((to_uid, pmsg.content));
                    }
                    _ => unreachable!(),
                }
            }
            info!("socket reader thread overed");
        });
    }

    let room = Canvas::new(chats)
        .with_draw(|chats, printer| {
            for (y, (uid, content)) in chats.lock().unwrap().iter().enumerate() {
                printer.print((0, y), &format!("{uid}: {content}"));
            }
        })
        .fixed_size((40, 15))
        .scrollable();

    let input = EditView::new()
        .on_submit_mut(move |_, s| {
            write_message(
                BufWriter::new(&mut writer),
                Message::new(
                    Method::Private,
                    Body::serialize(&PrivateMsg {
                        uid: to_uid,
                        content: s.to_owned(),
                    }),
                ),
            )
            .unwrap();
        })
        .with_name("input")
        .fixed_width(20);

    LinearLayout::vertical().child(room).child(input)
}
