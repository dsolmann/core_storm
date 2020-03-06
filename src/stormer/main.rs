use bincode;
use crossbeam_queue::ArrayQueue;
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
            input_dispatcher_queue:Arc::from(ArrayQueue::new(queue_capacity)),
            output_middleware_queue: Arc::from(ArrayQueue::new(queue_capacity)),
            output_queue: Arc::from(ArrayQueue::new(queue_capacity)),
            // thread_pool: vec![],
            max_workers_per_iproc: threads_per_proc,
            address: addr,
            pre_in_middleware: direct_middleware,
            pre_out_middleware: direct_middleware,
            in_dispatcher: InDispatcher::new(),
        }
    }

    pub fn set_input_middleware(&mut self, func: fn(&ArrayQueue<Message>, &ArrayQueue<Message>)) {
        self.pre_in_middleware = func;
    }

    pub fn set_output_middleware(&mut self, func: fn(&ArrayQueue<Message>, &ArrayQueue<Message>)) {
        self.pre_out_middleware = func;
    }

    pub fn start(&mut self) {
        for _ in 0..self.max_workers_per_iproc {
            // Initializing Output Middleware
            let o_m_q = Arc::clone(&self.output_middleware_queue);
            let o_d_q = Arc::clone(&self.output_queue);
            let mw_func = self.pre_out_middleware;
            thread::spawn(move || mw_func(&o_m_q, &o_d_q));

            // Initializing Input Dispatcher
            let mut d = self.in_dispatcher.clone();
            let inp_dis_q = Arc::clone(&self.input_dispatcher_queue);
            let out_mw_q = Arc::clone(&self.output_middleware_queue);

            thread::spawn(move || d.dispatch(&inp_dis_q, &out_mw_q));

            // Initializing Input Middleware
            let i_m_q = Arc::clone(&self.input_queue);
            let i_d_q = Arc::clone(&self.input_dispatcher_queue);
            let mw_func = self.pre_in_middleware;
            thread::spawn(move || mw_func(&i_m_q, &i_d_q));
        }
    }

    pub fn get_address(&self) -> Addr {
        self.address.clone()
    }

    pub fn register_handler(&mut self, protocol: UpperProto, handler_func: fn(&Message)) {
        self.in_dispatcher.register_callback(protocol, handler_func)
    }
}
