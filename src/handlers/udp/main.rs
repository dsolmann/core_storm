use std::net::UdpSocket;
use crate::protocol::Message;
use crossbeam_queue::ArrayQueue;

fn udp_send_handler(_m_queue: &ArrayQueue<Message>) {
    // loop {}
}
