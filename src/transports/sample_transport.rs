use crate::protocol::MetaMessage;
use crate::protocol::{Addr, Message, MsgType, UpperProto};
use crossbeam_queue::ArrayQueue;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;

pub fn sample_transport(output_queue: &ArrayQueue<Message>, addr: Addr) {
    loop {
        let d = MetaMessage::ping().encode();
        output_queue
            .push(Message {
                sender: Option::from(Addr(0xDE, 0xAD, 0xBE, 0xEF)),
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

        sleep(Duration::from_secs_f32(1.));
        // println!("SNT");
    }
}
