use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MetaMessage {
    pub m_type: MetaMethods,
    pub payload: Vec<u8>,
}

impl MetaMessage {
    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn from_bytes(data: &[u8]) -> MetaMessage {
        bincode::deserialize(data).unwrap()
    }

    pub fn ping() -> MetaMessage {
        MetaMessage {
            m_type: MetaMethods::EchoRequest,
            payload: vec![0xF; 16],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub enum MetaMethods {
    EchoRequest,
    EchoResponse,
    GetTime,
    TraceRoute
}
