mod actortest;
use crate::actortest::{MyActor, Ping};
use actix::prelude::*;
use notificationcenter::{NotificationCenter, Token};
use std::sync::Mutex;
use std::borrow::Borrow;

#[actix_rt::main]
async fn main() {
    // start new actor
    let addr = MyActor { count: 10 }.start();

    // send message and get future for result
    let res = addr.send(Ping{ i: 1 }).await;

    // handle() returns tokio handle
    println!("RESULT: {}", res.unwrap() == 20);

    // stop system and exit
    System::current().stop();

    println!("Hello, world!");

    NotificationCenter::observe("notification".to_string(), || {
        println!("got notified");
    });

    NotificationCenter::observe("notification".to_string(), || {
        println!("got notified here too");
    });

    let mut str = SomeStruct { this_thing: 1, token: Mutex::new(None) };
    str.do_some_work();
    NotificationCenter::post("notification".to_string());
    NotificationCenter::post_async("notification".to_string());
    NotificationCenter::post_async("notification".to_string());
    NotificationCenter::post_async("notification".to_string());
    NotificationCenter::post_async("notification".to_string());
    NotificationCenter::post_async("notification".to_string());
    std::thread::spawn(|| {
        for i in 0..10 {
            NotificationCenter::post_async("notificationblah".to_string());
        }
    });
    std::thread::park();

}

struct SomeStruct {
    this_thing : i32,
    token : Mutex<Option<Token>>
}

impl SomeStruct {
    fn do_some_work(&mut self) {
        // self.token = Mutex::new(Some(NotificationCenter::observe("notification".to_string(), || {
        //     println!("got here");
        // })));
        // use weak_self::WeakSelf;
        // use std::sync::{Arc, Weak};
        // let weak_self = WeakSelf::new();
        // let a = Arc::new(&self);
        // weak_self.init(&a);
        //
        // NotificationCenter::observe("notificationblah".to_string(), move || {
        //     println!("got notificationblah");
        //     weak_self;
        //     // if let Some(to) = (*self.token.lock().unwrap()).borrow() {
        //     //     NotificationCenter::remove_observer(to.clone());
        //     // }
        // });
    }
}
