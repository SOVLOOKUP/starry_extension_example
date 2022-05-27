use std::{thread::sleep, time::Duration};
use chrono::prelude::*;
extern crate chrono;

use starry_extension_interface::{
    EmitContent, ArcEmitSender, Extension, ListenContent, ArcListenSender, ListenType,
};

#[no_mangle]
pub fn new() -> Box<dyn Extension + Send> {
    Box::new(ExtensionSayHello {})
}

pub struct ExtensionSayHello {}

impl Extension for ExtensionSayHello {
    fn id(&self) -> &str {
        "3a90790e"
    }

    fn info(&self) -> &str {
        "{}"
    }

    fn load(&self, emit_sender: &ArcEmitSender, listen_sender: &ArcListenSender) {
        // let emit_sender2 = emit_sender.clone();
        // // 监听 plus 事件
        // listen_sender.send(ListenContent {
        //     event: "plus".to_string(),
        //     content_type: ListenType::Listen(Box::new(move |x| {
        //         let mut y = String::from(x.payload().unwrap());
        //         y.push_str("plustr");
        //         emit_sender2.clone().send(EmitContent {
        //             id: x.id().to_string(),
        //             event: "callback".to_string(),
        //             payload: y,
        //         }).unwrap();
        //     })),
        // }).unwrap();

        // loop {
        //     // 每两秒一次心跳
            emit_sender.send(EmitContent {
                id: "".to_string(),
                event: "heartbeat".to_string(),
                payload: Local::now().to_string(),
            }).unwrap();
        //     sleep(Duration::from_secs(2))
        // }
    }

    fn unload(&self) {
        println!("[{}] Unload instance!", self.id());
    }
}
