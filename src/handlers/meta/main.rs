use crate::handlers::meta::{MetaMessage, MetaMethods};
use crate::protocol::{Message, UpperProto, Addr, MsgType};
use std::option::Option;

fn meta_handler(msg: &Message, self_addr: Addr) -> Option<Message> {
    let meta_msg = MetaMessage::from_bytes(&msg.data);
    if meta_msg.m_type == MetaMethods::EchoRequest {
        let meta_resp = MetaMessage { m_type: MetaMethods::EchoResponse, payload: meta_msg.payload }.encode();
        Option::from(
            Message {
                sender: Option::from(self_addr),
                radius: None,
                ttl: msg.ttl,
                data: meta_resp.clone(),
                u_proto: UpperProto::MetaProto,
                msg_type: MsgType::UnicastL,
                to: msg.sender.unwrap_or(Addr(0, 0, 0, 1)),
                hash: Message::hash_it(meta_resp.clone()),
                id: Default::default()
            }
        )
    } else {
        Option::None
    }
}