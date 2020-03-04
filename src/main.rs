use bincode;
use std::thread;
use std::option;
use std::hash::{Hash, Hasher};
use serde::{Serialize, Deserialize};
use crossbeam_channel::{unbounded, Sender, Receiver, TryRecvError};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use std::error::Error;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash, Copy)]
struct Addr (u16, u16, u16, u16);

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash, Copy)]
enum UpperProto {
    OneWay,
    ConnProto,
    RelSavNet,
    MetaProto
}


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
struct MetaMessage {
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
enum MetaMethods {
    Ping,
    GetTime,
    NeighbourSearch,
    AprDistance,
    TraceRoute
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash, Copy)]
enum MsgType {
    Broadcast,
    RadiusBroadcast,
    UnicastL
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
struct Message {
    sender: Option<Addr>,
    radius: Option<u16>,
    ttl: u16,
    data: Vec<u8>,
    u_proto: UpperProto,
    msg_type: MsgType,
    to: Addr,
    hash: u64,
    id: Uuid
}

impl Message {
    fn hash_it(hashable: Vec<u8>) -> u64{
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
            hash: Message::hash_it(data.clone()),
            id: Uuid::new_v4()
        }
    }
    pub fn encode(&self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn from_vec_u8(data: &Vec<u8>) -> Message {
        bincode::deserialize(data).unwrap()
    }
}

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
    pub fn dispatch(&mut self, receiver: &Receiver<Message>) {
        loop {
            // println!("{:#?}", receiver);
            if receiver.is_empty() {
                continue
            };

            let mut msg = receiver.recv().unwrap();
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
    let mut in_dispatcher = InDispatcher::new();
    let (sender, receiver) = unbounded();
    thread::spawn(move || {
        loop {
            let d = MetaMessage::ping().encode();
            sender.send(Message {
                sender: Option::from(Addr(0xDE, 0xAD, 0xBE, 0xEF)),
                radius: None,
                u_proto: UpperProto::MetaProto,
                ttl: 64,
                id: Uuid::new_v4(),
                msg_type: MsgType::UnicastL,
                data: d.clone(),
                to: Addr(0xDE, 0xAD, 0xBE, 0xEF),
                hash: Message::hash_it(d.clone())
            }).unwrap_or_else(|e| println!("{:#?}", e.to_string()));
            sleep(Duration::from_secs_f32(0.1));
            println!("SNT");
        }
    });

    in_dispatcher.register_callback(
        UpperProto::MetaProto,
        |message| {
            println!("RCVD");
        }
    );

    for i in 0..4 {
        let mut d_2 = in_dispatcher.clone();
        let r = receiver.clone();
        thread::spawn(move || d_2.dispatch(&r));
    }

    loop {}
}
