#![allow(dead_code)]
mod dispatcher;
mod handlers;
mod middlewares;
mod protocol;
mod stormer;
mod transports;

use crate::protocol::Addr;
use crate::protocol::UpperProto;
use crate::stormer::CoreStorm;
use crate::transports::sample_transport;

fn main() {
    let addr = Addr::random();
    let mut storm = CoreStorm::new(addr, 3, 1024);
    storm.register_handler(UpperProto::MetaProto, |msg| println!("{:#?}", msg.id));
    storm.start();
    sample_transport(&storm.input_queue, addr);
}
