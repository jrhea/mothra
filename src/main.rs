extern crate getopts;
use std::sync::mpsc;
use std::{thread, time};
use slog::{info, o, Drain};
use eth2_libp2p::Message;
use mothra::api_libp2p::api;

fn main() {

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let slog = slog::Logger::root(drain, o!());
    let log = slog.new(o!("API" => "API"));

    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let nlog = log.clone();
    thread::spawn(move || {
        api::start(&tx2, &rx1, nlog.new(o!("API" => "start()")));
    });
    
    let dur = time::Duration::from_millis(500);
    loop{
        thread::sleep(dur);
        let message = Message {
            command: "GOSSIP".to_string(), 
            value: "Blah".as_bytes().to_vec()
        };
        tx1.send(message).unwrap();

        let network_message = rx2.recv().unwrap();
        if network_message.command == "GOSSIP".to_string() {
            info!(log,"receieved a network message!!!!!!!!{:?}",String::from_utf8(network_message.value))
        }
    }

}

