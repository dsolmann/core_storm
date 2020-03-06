use stormer::*;

mod dispatcher;
mod middlewares;
mod protocol;
mod stormer;
mod transports;

use crate::stormer::CoreStorm;
use protocol::Addr;
use crate::protocol::UpperProto;
use crate::transports::sample_transport;

fn main() {
    let addr = Addr::random();
    let mut storm = CoreStorm::new(addr, 3, 1024);
    storm.register_handler(UpperProto::MetaProto, |msg| {
        println!("{:#?}", msg.id)
    });
    storm.start();
    sample_transport(&storm.input_queue, addr);
}
