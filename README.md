# Core Storm

## Running & Building

To build: 

```bash
~> cargo build --release
```

To run (in debug): 

```bash
~> cargo run
```

## Description

Core Storm is a protocol aggregation system. You give it a message & address - it will handle delivery for you to make sure that your message was received.

## Idea

Under the hood Core Storm has several message queues to maximize efficiency on multicore systems.

### Receiving

Since message got received by some of `Transports`, `Transport` will immediately put this message in first queue. Then, this Message will be taken by optional `Middleware`, which can be used to filter packages or do something else. After that, `Middleware` will put (or not) message into the next queue, where `Dispatcher` will take it & make sure that it'll be retranslated or sent to some internal listener.

## Usage

### Example receiving program: 

``````rust
use core_storm::stormer::CoreStorm;
use core_storm::protocol::Addr;
use core_storm::protocol::UpperProto;
use core_storm::transports::sample_transport;

fn main() {
    let addr = Addr::random(); // Setting up our CS address.
    let mut storm = CoreStorm::new(addr, 3, 1024); // Creating instance of our daemon.
    storm.register_handler(UpperProto::MetaProto, |msg| {
        println!("{:#?}", msg.id)
    }); // Registering handler (in this case, it'll just print messages, which match `MetaProto` protocol)
    storm.start(); // Note: it'll start in different threads.
    sample_transport(&storm.input_queue, addr); // Registering some transport
}
``````

