use crate::protocol::Message;
use crossbeam_queue::ArrayQueue;

pub fn direct_middleware(input_queue: &ArrayQueue<Message>, output_queue: &ArrayQueue<Message>) {
    loop {
        match input_queue.pop() {
            Ok(message) => output_queue.push(message).unwrap(),
            _ => continue,
        };
    }
}
