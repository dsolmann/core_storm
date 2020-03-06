use crate::protocol::Message;
use crossbeam_queue::ArrayQueue;
use log::{debug, error};

pub fn direct_middleware(input_queue: &ArrayQueue<Message>, output_queue: &ArrayQueue<Message>) {
    loop {
        match input_queue.pop() {
            Ok(message) => match output_queue.push(message) {
                Ok => continue,
                Err => {error!("Queue Overflow!"); continue}
            },
            _ => continue,
        };
    }
}
