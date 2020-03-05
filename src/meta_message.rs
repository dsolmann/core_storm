use serde::{Serialize, Deserialize};
use std::hash::{Hash, Hasher};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MetaMessage {
    m_type: MetaMethods,
    payload: Vec<u8>
}
impl MetaMessage {
    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn from_vec_u8(data: &Vec<u8>) -> MetaMessage {
        bincode::deserialize(data).unwrap()
    }

    pub fn ping() -> MetaMessage {
        MetaMessage {
            m_type: MetaMethods::Ping,
            payload: vec![]
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub enum MetaMethods {
    Ping,
    GetTime,
    NeighbourSearch,
    AprDistance,
    TraceRoute
}
