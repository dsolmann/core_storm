use bincode;
use std::thread;
use std::option;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

use crossbeam_queue::{PopError, SegQueue};

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;

mod transports;
mod meta_message;
mod middlewares;
mod protocol;

use crate::protocol::{Message, Addr, UpperProto, MsgType};
use uuid::Uuid;
use crate::meta_message::MetaMessage;
use crate::transports::{sample_transport};
use crate::middlewares::{direct_middleware};

#[derive(Clone)]
struct InDispatcher {
    passed_pkt: Vec<Uuid>,
    addr: Addr,
    counter: u64,
    registered_callbacks: HashMap<UpperProto, fn(&Message)>
}
impl InDispatcher {
    pub fn new() -> InDispatcher {
        InDispatcher {
            passed_pkt: vec![],
            addr: Addr(0xDE, 0xAD, 0xBE, 0xEF),
            counter: 0,
            registered_callbacks: Default::default(),
        }
    }
    pub fn register_callback(&mut self, proto:UpperProto, func: fn(&Message)) {
        self.registered_callbacks.insert(proto, func);
    }
    pub fn dispatch(&mut self, input_queue: &SegQueue<Message>) {
        loop {
            // println!("{:#?}", receiver);
            let mut msg = match input_queue.pop() {
                Ok(Message) => Message,
                _ => {
                    continue
                }
            };
            let m_id = &msg.id;
            self.counter += 1;
            msg.ttl -= 1;
            println!("{}", &self.counter);
            if !self.passed_pkt.contains(*&m_id) & (
                (self.addr == msg.to) | (Addr(0, 0, 0, 0) == msg.to)) & (msg.ttl > 0)
            {
                if !self.registered_callbacks.contains_key(&msg.u_proto) {
                    self.passed_pkt.push(*&msg.id);
                    continue // TODO: Make A Handler
                } else {
                    self.passed_pkt.push(*&msg.id);
                    let func = self.registered_callbacks[&msg.u_proto];
                    thread::spawn(move || func(&msg));
                    continue
                }
            }
        }
    }
}

fn main() {
    let input_middleware_q: Arc<SegQueue<Message>> = Arc::new(SegQueue::new());
    let input_dispatcher_q: Arc<SegQueue<Message>> = Arc::new(SegQueue::new());

    // let output_middleware_q = SegQueue::new();
    // let output_dispatcher_q = SegQueue::new();

    let mut in_dispatcher = InDispatcher::new();

    in_dispatcher.register_callback(
        UpperProto::MetaProto,
        |message| {
            // println!("RCVD");
        }
    );

    for _ in 0..8 {
        let mut d = in_dispatcher.clone();
        let dis_q= Arc::clone(&input_dispatcher_q);
        thread::spawn( move || d.dispatch(&dis_q));
    }

    for _ in 0..8 {
        let i_m_q = Arc::clone(&input_middleware_q);
        let i_d_q = Arc::clone(&input_dispatcher_q);
        thread::spawn(
            move || direct_middleware(
                &i_m_q,
                &i_d_q
            )
        ); // Attach I_MW_Q to I_Q
    }

    for _ in 0..8 {
        let i_m_q = Arc::clone(&input_middleware_q);
        thread::spawn(
            move || sample_transport(&i_m_q)
        ); // Start SMP Transport
    }

    loop {}
}
