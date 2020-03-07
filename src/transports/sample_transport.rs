use crate::handlers::meta::MetaMessage;
use crate::protocol::{Addr, Message, MsgType, UpperProto};
use crossbeam_queue::ArrayQueue;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;

pub fn sample_looping_transport(output_queue: &ArrayQueue<Message>, addr: Addr, interval: f32, sender: Option<Addr>) {
    loop {
        let d = MetaMessage::ping(&[0]).encode();
        output_queue
            .push(Message {
                sender,
                radius: None,
                u_proto: UpperProto::MetaProto,
                ttl: 64,
                id: Uuid::new_v4(),
                msg_type: MsgType::UnicastL,
                data: d.clone(),
                to: addr,
                hash: Message::hash_it(d.clone()),
            })
            .unwrap();

        sleep(Duration::from_secs_f32(interval));
        // println!("SNT");
    }
}

pub fn sample_transport(output_queue: &ArrayQueue<Message>, addr: Addr, sender: Option<Addr>) {
        let d = MetaMessage::ping(&[0]).encode();
        output_queue
            .push(Message {
                sender,
                radius: None,
                u_proto: UpperProto::MetaProto,
                ttl: 64,
                id: Uuid::new_v4(),
                msg_type: MsgType::UnicastL,
                data: d.clone(),
                to: addr,
                hash: Message::hash_it(d.clone()),
            })
            .unwrap();
}
