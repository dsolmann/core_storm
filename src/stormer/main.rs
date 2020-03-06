use crossbeam_queue::ArrayQueue;
use log::{debug, error, info, trace, warn};
use std::hash::{Hash, Hasher};
use std::option;
use std::sync::Arc;
use std::thread;

use crate::dispatcher::InDispatcher;
use crate::middlewares::direct_middleware;
use crate::protocol::{Addr, Message, MsgType, UpperProto};
use crate::transports::sample_transport;

pub struct CoreStorm {
    pub input_queue: Arc<ArrayQueue<Message>>,
    input_dispatcher_queue: Arc<ArrayQueue<Message>>,
    output_middleware_queue: Arc<ArrayQueue<Message>>,
    output_queue: Arc<ArrayQueue<Message>>,
    // thread_pool: Vec<io::Result<JoinHandle<T>>>,
    max_workers_per_iproc: usize,
    address: Addr,
    pre_in_middleware: fn(&ArrayQueue<Message>, &ArrayQueue<Message>),
    pre_out_middleware: fn(&ArrayQueue<Message>, &ArrayQueue<Message>),
    in_dispatcher: InDispatcher,
}

impl CoreStorm {
    pub fn new(addr: Addr, threads_per_proc: usize, queue_capacity: usize) -> CoreStorm {
        CoreStorm {
            input_queue: Arc::from(ArrayQueue::new(queue_capacity)),
            input_dispatcher_queue: Arc::from(ArrayQueue::new(queue_capacity)),
            output_middleware_queue: Arc::from(ArrayQueue::new(queue_capacity)),
            output_queue: Arc::from(ArrayQueue::new(queue_capacity)),
            // thread_pool: vec![],
            max_workers_per_iproc: threads_per_proc,
            address: addr,
            pre_in_middleware: direct_middleware,
            pre_out_middleware: direct_middleware,
            in_dispatcher: InDispatcher::new(addr),
        }
    }

    pub fn set_input_middleware(&mut self, func: fn(&ArrayQueue<Message>, &ArrayQueue<Message>)) {
        self.pre_in_middleware = func;
        info!("Input middleware set.")
    }

    pub fn set_output_middleware(&mut self, func: fn(&ArrayQueue<Message>, &ArrayQueue<Message>)) {
        self.pre_out_middleware = func;
        info!("Output middleware set.")
    }

    pub fn start(&mut self) {
        info!("Starting CoreStorm's instance.");
        for _ in 0..self.max_workers_per_iproc {
            // Initializing Output Middleware
            let o_m_q = Arc::clone(&self.output_middleware_queue);
            let o_d_q = Arc::clone(&self.output_queue);
            let mw_func = self.pre_out_middleware;
            thread::spawn(move || mw_func(&o_m_q, &o_d_q));
            info!("Intermediate output queue connected to final queue.");

            // Initializing Input Dispatcher
            let mut d = self.in_dispatcher.clone();
            let inp_dis_q = Arc::clone(&self.input_dispatcher_queue);
            let out_mw_q = Arc::clone(&self.output_middleware_queue);

            thread::spawn(move || d.dispatch(&inp_dis_q, &out_mw_q));
            info!("in_dispatcher connected to intermediate output queue.");

            // Initializing Input Middleware
            let i_m_q = Arc::clone(&self.input_queue);
            let i_d_q = Arc::clone(&self.input_dispatcher_queue);
            let mw_func = self.pre_in_middleware;
            thread::spawn(move || mw_func(&i_m_q, &i_d_q));
            info!("Input Queue connected to in_dispatcher input queue.");
        }
    }

    pub fn get_address(&self) -> Addr {
        self.address
    }

    pub fn register_handler(&mut self, protocol: UpperProto, handler_func: fn(&Message)) {
        self.in_dispatcher.register_callback(protocol, handler_func);
        info!("Assigned handler to {:?}.", protocol);
    }

    pub fn accept_message(&mut self, msg: Message) {
        self.input_queue.push(msg).unwrap();
        debug!("Message received.")
    }
}
