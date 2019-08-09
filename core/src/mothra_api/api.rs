use clap::ArgMatches;
use futures::prelude::*;
use std::sync::mpsc as sync;
use std::time::{Duration, Instant};
use std::env;
use std::process;
use slog::{debug, info, o, warn};
use tokio::runtime::TaskExecutor;
use tokio::runtime::Builder;
use tokio::timer::Interval;
use tokio_timer::clock::Clock;
use futures::Future;
use clap::{App, Arg, AppSettings};
use libp2p_wrapper::{NetworkConfig, TopicBuilder, BEACON_PUBSUB_TOPIC,Message};
use tokio::sync::mpsc;
use super::network::{Network,NetworkMessage};

/// The interval between heartbeat events.
pub const HEARTBEAT_INTERVAL_SECONDS: u64 = 15;

/// Create a warning log whenever the peer count is at or below this value.
pub const WARN_PEER_COUNT: usize = 1;

pub fn start(args: ArgMatches, local_tx: &sync::Sender<Message>,local_rx: &sync::Receiver<Message>, log: slog::Logger) {
    info!(log,"Initializing libP2P....");
    let runtime = Builder::new()
        .name_prefix("api-")
        .clock(Clock::system())
        .build()
        .map_err(|e| format!("{:?}", e)).unwrap();
    let executor = runtime.executor();
    let mut network_config = NetworkConfig::new();
    network_config.apply_cli_args(&args).unwrap();
    let network_logger = log.new(o!("Network" => "Network"));
    let (network_tx, network_rx) = sync::channel();
    let (network, network_send) = Network::new(
            network_tx,
            &network_config,
            &executor.clone(),
            network_logger,
    ).unwrap();
    
    monitor(&network, executor, log.clone());
    
    loop {
        let local_message = local_rx.recv().unwrap();
        if local_message.command == "GOSSIP".to_string() {
            gossip(network_send.clone(), local_message.value.to_vec(),log.new(o!("API" => "gossip()")));
        }
        let network_message = network_rx.recv().unwrap();
        local_tx.send(network_message).unwrap();
    }
}

fn monitor(
    network: &Network,
    executor: TaskExecutor,
    log: slog::Logger
) {
    let err_log = log.clone();
    let (_exit_signal, exit) = exit_future::signal();
    // notification heartbeat
    let interval = Interval::new(
        Instant::now(),
        Duration::from_secs(HEARTBEAT_INTERVAL_SECONDS),
    );

    let libp2p = network.libp2p_service();

    let heartbeat = move |_| {

        let connected_peer_count = libp2p.lock().swarm.num_connected_peers();

        debug!(log, "libp2p"; "peer_count" => connected_peer_count);

        if connected_peer_count <= WARN_PEER_COUNT {
            warn!(log, "Low libp2p peer count"; "peer_count" => connected_peer_count);
        }

        Ok(())
    };

    // map error and spawn
    let heartbeat_interval = interval
        .map_err(move |e| debug!(err_log, "Timer error {}", e))
        .for_each(heartbeat);
    executor.spawn(exit.until(heartbeat_interval).map(|_| ()));

}

fn gossip( mut network_send: mpsc::UnboundedSender<NetworkMessage>, message: Vec<u8>, log: slog::Logger){
    let topic = TopicBuilder::new(BEACON_PUBSUB_TOPIC).build();
    network_send.try_send(NetworkMessage::Publish {
        topics: vec![topic],
        message: message,
    }).unwrap_or_else(|_| {
        warn!(
            log,
            "Could not send gossip message."
        )
    });
}

pub fn config(mut args: Vec<String>) -> ArgMatches<'static> {
    
    App::new("Artemis")
    .version("0.0.1")
    .author("Your Mom")
    .about("Eth 2.0 Client")
    .setting(AppSettings::TrailingVarArg)
    .setting(AppSettings::DontDelimitTrailingValues)
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
   .get_matches_from_safe(args.iter())
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        })
}