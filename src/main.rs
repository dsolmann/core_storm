#![allow(dead_code)]
mod dispatcher;
mod handlers;
mod middlewares;
mod protocol;
mod stormer;
mod transports;

use std::option::Option;

use crate::protocol::Addr;
use crate::protocol::UpperProto;
use crate::stormer::CoreStorm;
use crate::transports::sample_transport;
use crate::handlers::meta::meta_handler;

extern crate simplelog;
use simplelog::*;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Debug, Config::default(), TerminalMode::Mixed).unwrap()
        ]
    ).unwrap();

    let addr = Addr::random();
    let mut storm = CoreStorm::new(addr, 1, 1024);
    storm.init_default_handlers();
    storm.start();
    storm.ping(addr, 1);
    loop {}
}
