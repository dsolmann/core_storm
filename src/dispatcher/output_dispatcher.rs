use crate::protocol::{Addr, Message, MsgType, UpperProto};
use crate::transports::sample_looping_transport;
use crossbeam_queue::{ArrayQueue, PopError, PushError};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use uuid::Uuid;
use std::option::Option;
use std::sync::Arc;
use log::{info, debug};

#[derive(Clone)]
pub struct OutDispatcher {
    addr: Addr,
    counter: u64,
}

impl OutDispatcher {
    pub fn new(addr: Addr) -> OutDispatcher {
        OutDispatcher {
            addr,
            counter: 0
        }
    }

    pub fn dispatch(
        &mut self,
        out_dispatcher_queue: &ArrayQueue<Message>,
        loopback_queue: &ArrayQueue<Message>,
    ) {
        loop {
            // println!("{:#?}", receiver);
            let mut msg: Message = match out_dispatcher_queue.pop() {
                Ok(message) => message,
                _ => continue,
            };
            debug!("New message in out_dispatcher queue.");
            let _m_id = &msg.id;
            self.counter += 1;
            if msg.to == self.addr {
                debug!("Message sent back to ourselves.");
                loopback_queue.push(msg).unwrap();
            } else if msg.to == Addr(0, 0, 0, 1) {
                // just doing nothing... yet.
                continue;
            } else {
                // TODO: Routing
            }
        }
    }
}
