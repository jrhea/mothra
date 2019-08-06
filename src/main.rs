extern crate getopts;
use std::sync::mpsc;
use std::{thread, time};
use slog::{info, o, Drain};
use clap::{App, Arg};
use libp2p::tokio_codec::{FramedRead, LinesCodec};
use futures::prelude::*;
use exit_future::Exit;
use mothra::libp2p_wrapper::service::{Service,NetworkMessage};
use mothra::libp2p_wrapper::api;

fn main() {
    // Logging
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!());

 
    let matches = App::new("Artemis")
        .version("0.0.1")
        .author("Your Mom")
        .about("Eth 2.0 Client")
        .arg(
            Arg::with_name("datadir")
                .long("datadir")
                .value_name("DIR")
                .help("Data directory for keys and databases.")
                .takes_value(true)
        )
        // network related arguments
        .arg(
            Arg::with_name("listen-address")
                .long("listen-address")
                .value_name("Address")
                .help("The address artemis will listen for UDP and TCP connections. (default 127.0.0.1).")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("maxpeers")
                .long("maxpeers")
                .help("The maximum number of peers (default 10).")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("boot-nodes")
                .long("boot-nodes")
                .allow_hyphen_values(true)
                .value_name("BOOTNODES")
                .help("One or more comma-delimited base64-encoded ENR's to bootstrap the p2p network.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("Artemis Port")
                .help("The TCP/UDP port to listen on. The UDP port can be modified by the --discovery-port flag.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("discovery-port")
                .long("disc-port")
                .value_name("DiscoveryPort")
                .help("The discovery UDP port.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("discovery-address")
                .long("discovery-address")
                .value_name("Address")
                .help("The IP address to broadcast to other peers on how to reach this node.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug-level")
                .long("debug-level")
                .value_name("LEVEL")
                .short("s")
                .help("The title of the spec constants for chain config.")
                .takes_value(true)
                .possible_values(&["info", "debug", "trace", "warn", "error", "crit"])
                .default_value("info"),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Sets the verbosity level")
                .takes_value(true),
        )
        .get_matches();




    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        api::init(&matches, &rx, log.new(o!("Service" => "init")));
    });
    
    let dur = time::Duration::from_millis(500);
    loop{
        thread::sleep(dur);
        let message = api::Message {
            command: "GOSSIP".to_string(), 
            value: "Blah".as_bytes().to_vec()
        };
        tx.send(message);
    }
    
    
    //let stdin = tokio_stdin_stdout::stdin(0);
    //let mut framed_stdin = FramedRead::new(stdin, LinesCodec::new());
    //let glog = log.new(o!("Service" => "gossip"));
    
	// tokio::run(futures::future::poll_fn(move || -> Result<_, ()> {
    //     loop {
    //         match framed_stdin.poll().expect("Error while polling stdin") {
    //             Async::Ready(Some(line)) => api::gossip(network_send.to_owned(),line.as_bytes().to_vec(),glog.to_owned()),
    //             Async::Ready(None) => unreachable!("Stdin closed"),
    //             Async::NotReady => break,
    //         };
    //     }
    //     Ok(Async::NotReady)
    // }));

    //info!(log,"Goodbye.")

}

