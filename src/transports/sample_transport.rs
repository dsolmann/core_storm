use crate::meta_message::MetaMessage;
use crate::protocol::{UpperProto, Message, Addr, MsgType};
use uuid::Uuid;
use std::thread::sleep;
use std::time::Duration;
use crossbeam_queue::SegQueue;

pub fn sample_transport(output_queue: &SegQueue<Message>) {
    loop {
        let d = MetaMessage::ping().encode();
        output_queue.push(
            Message {
                sender: Option::from(Addr(0xDE, 0xAD, 0xBE, 0xEF)),
                radius: None,
                u_proto: UpperProto::MetaProto,
                ttl: 64,
                id: Uuid::new_v4(),
                msg_type: MsgType::UnicastL,
                data: d.clone(),
                to: Addr(0xDE, 0xAD, 0xBE, 0xEF),
                hash: Message::hash_it(d.clone())
            }
        );

        // sleep(Duration::from_secs_f32(0.1));
        // println!("SNT");
    }
}