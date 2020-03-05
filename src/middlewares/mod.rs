use crossbeam_queue::{SegQueue};
use crate::protocol::Message;

pub fn direct_middleware(input_queue: &SegQueue<Message>, output_queue: &SegQueue<Message>) {
    loop {
        match input_queue.pop() {
            Ok(message) => output_queue.push(message),
            _ => {
                continue
            }
        };
    }
}