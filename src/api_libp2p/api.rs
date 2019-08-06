use clap::ArgMatches;
use futures::prelude::*;
use std::sync::mpsc as sync;
use std::time::{Duration, Instant};
use slog::{debug, info, o, warn};
use tokio::runtime::TaskExecutor;
use tokio::runtime::Builder;
use tokio::timer::Interval;
use tokio_timer::clock::Clock;
use futures::Future;
use eth2_libp2p::{NetworkConfig, TopicBuilder, BEACON_PUBSUB_TOPIC};
use tokio::sync::mpsc;
use super::network::{Network,NetworkMessage};

/// The interval between heartbeat events.
pub const HEARTBEAT_INTERVAL_SECONDS: u64 = 15;

/// Create a warning log whenever the peer count is at or below this value.
pub const WARN_PEER_COUNT: usize = 1;

pub struct Message {
    pub command: String,
    pub value: Vec<u8>
}

pub fn init(args: &ArgMatches, rx: &sync::Receiver<Message>, log: slog::Logger) {
    info!(log,"Initializing libP2P....");
    let mut runtime = Builder::new()
        .name_prefix("init-")
        .clock(Clock::system())
        .build()
        .map_err(|e| format!("{:?}", e)).unwrap();
    let executor = runtime.executor();
    let mut network_config = NetworkConfig::new();
    network_config.apply_cli_args(args);
    let network_logger = log.new(o!("Network" => "Network"));
    let (network, network_send) = Network::new(
            &network_config,
            &executor.clone(),
            network_logger,
    ).unwrap();
    
    run(&network, executor, log.clone());
    
    loop {
        let recv = rx.recv().unwrap();
        if recv.command == "GOSSIP".to_string() {
            gossip(network_send.clone(), recv.value.to_vec(),log.new(o!("Network" => "gossip")));
        }
    }
}

fn run(
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






