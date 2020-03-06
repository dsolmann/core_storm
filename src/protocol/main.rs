use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub struct Addr(pub u16, pub u16, pub u16, pub u16);

impl Addr {
    pub fn random() -> Addr {
        Addr(random(), random(), random(), random())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub enum UpperProto {
    OneWay,
    ConnProto,
    RelSavNet,
    MetaProto,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub enum MsgType {
    Broadcast,
    RadiusBroadcast,
    UnicastL,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Message {
    pub sender: Option<Addr>,
    pub radius: Option<u16>,
    pub ttl: u16,
    pub data: Vec<u8>,
    pub u_proto: UpperProto,
    pub msg_type: MsgType,
    pub to: Addr,
    pub hash: u64,
    pub id: Uuid,
}

impl Message {
    pub fn hash_it(hashable: Vec<u8>) -> u64 {
        let mut hasher = DefaultHasher::new();
        hashable.hash(&mut hasher);
        hasher.finish()
    }
    pub fn make_bc_message(data: Vec<u8>, proto: UpperProto) -> Message {
        // let data =  bincode::serialize("Test Data to Transfer; Hello, world!").unwrap();
        Message {
            sender: None,
            radius: None,
            u_proto: proto,
            ttl: 256,
            msg_type: MsgType::Broadcast,
            data: data.clone(),
            to: Addr(0, 0, 0, 0),
            hash: Message::hash_it(data),
            id: Uuid::new_v4(),
        }
    }
    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn from_bytes(data: &[u8]) -> Message {
        bincode::deserialize(data).unwrap()
    }
}
