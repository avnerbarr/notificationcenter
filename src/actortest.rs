use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "i32")]
pub struct Ping {
    pub i: i32
}


pub struct MyActor {
    pub count: i32,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl Handler<Ping> for MyActor {
    type Result = i32;

    fn handle(&mut self, msg: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        self.count += msg.i;

        self.count
    }
}