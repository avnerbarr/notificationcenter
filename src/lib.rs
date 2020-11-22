#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;
use uuid::Uuid;
use std::collections::{HashMap, HashSet};
use std::borrow::BorrowMut;

pub struct ChannelWrapper {
    sender : crossbeam::channel::Sender<String>,
    receiver: crossbeam::channel::Receiver<String>
}
impl ChannelWrapper {
    pub fn new() -> Self {
        let (s,r) = crossbeam::channel::unbounded();
        ChannelWrapper { sender: s, receiver: r }
    }
}
lazy_static! {
    pub static ref CHANNEL_WRAPPER : ChannelWrapper = ChannelWrapper::new();
    static ref OBSERVER_COLLECTION : Mutex<ObserverCollection> = Mutex::new(ObserverCollection::new());
}

pub struct NotificationCenter;

#[derive(Clone, PartialEq, Hash, Eq)]
pub struct Token {
    id : String,
    notification: String
}


impl Token {
    fn new(notification: String) -> Self {
        let my_uuid = Uuid::new_v4();
        Token { id: my_uuid.to_string(), notification }
    }
}

// Box<dyn Fn() + Send + Sync + 'static>
struct ObserverCollection {
    tokens : HashMap<Token, Box<dyn Fn() + Send + Sync + 'static>>,
    callbacks: HashMap<String, HashSet<Token>>
}

impl ObserverCollection {
    pub fn new() -> Self {
        ObserverCollection { tokens: Default::default(), callbacks: Default::default() }
    }
    pub fn add_observer<F>(&mut self, notification: String, f: F) -> Token where F: Fn() + Send + Sync + 'static {
        let t=  Token::new(notification.clone());
        self.tokens.insert(t.clone(), Box::new(f));
        self.callbacks.entry(notification.clone()).or_insert_with(|| HashSet::new()).insert(t.clone());
        t.clone()
    }
    pub fn remove_observer(&mut self, token: Token) {
        self.tokens.remove(&token);

        if let Some(m) = self.callbacks.get_mut(&token.notification) {
            m.remove(&token);
        }
    }
    pub fn observers_for_notification(&self, notification: String) -> Vec<&Box<dyn Fn() + Send + Sync>> {
        let mut ve = Vec::new();
        if let Some(n) = self.callbacks.get(&notification) {

            for t in n {
                if let Some(cb) = self.tokens.get(t) {
                    ve.push(cb);
                }
            }

        }
        return ve
    }
}

impl NotificationCenter {
    pub fn remove_observer(token: Token) {
        OBSERVER_COLLECTION.lock().unwrap().borrow_mut().remove_observer(token);
    }
    pub fn observe<F>(notification: String, f: F ) -> Token where F: Fn() + Send + Sync + 'static {
        OBSERVER_COLLECTION.lock().unwrap().borrow_mut().add_observer(notification, f)
    }
    pub fn post(notification: String) {
        if let a = OBSERVER_COLLECTION.lock().unwrap().observers_for_notification(notification) {
            for c  in a {
                println!("current thread is {:?}", std::thread::current().name());
                (*c)();
            }
        }
    }

    pub fn post_async(notification: String) {
        use std::sync::Once;
        static START: Once = Once::new();

        START.call_once(|| {
            let c = CHANNEL_WRAPPER.receiver.clone();
            std::thread::Builder::new().name("notification center".to_string()).spawn(move || {
                loop {
                    if let Ok(v) = c.recv() {
                        Self::post(v);
                        // std::thread::spawn(|| {
                        //     Self::post(v);
                        // });

                    }
                }

            }).unwrap();
        });


        let _ = CHANNEL_WRAPPER.sender.clone().send(notification);


    }
}


pub trait Observer<M> : Sized  + 'static {
    fn handle(&mut self, notification: String, content: M);
}

pub struct Bus {

}

impl Bus {
    pub fn observe<M>(notification: String, observer: impl Observer<M>) {

    }
}
