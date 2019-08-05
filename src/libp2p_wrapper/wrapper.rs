use clap::ArgMatches;
use futures::prelude::*;
use std::sync::Arc;
use std::cell::RefCell;
use std::time::{Duration, Instant};
use parking_lot::Mutex;
use slog::{crit, debug, info, o, Drain, trace, warn};
use tokio::runtime::TaskExecutor;
use tokio::runtime::Builder;
use tokio::timer::Interval;
use tokio_timer::clock::Clock;
use futures::sync::oneshot;
use futures::Future;
use exit_future::Exit;
use ctrlc;
use super::service::Service;
use eth2_libp2p::NetworkConfig;
use eth2_libp2p::Service as LibP2PService;
use eth2_libp2p::{Libp2pEvent, PeerId, Topic};
use eth2_libp2p::{PubsubMessage, RPCEvent};

/// The interval between heartbeat events.
pub const HEARTBEAT_INTERVAL_SECONDS: u64 = 15;

/// Create a warning log whenever the peer count is at or below this value.
pub const WARN_PEER_COUNT: usize = 1;

pub fn start_libp2p_service(args: &ArgMatches, log: slog::Logger) {
    info!(log,"Initializing libP2P....");
    let mut runtime = Builder::new()
        .name_prefix("main-")
        .clock(Clock::system())
        .build()
        .map_err(|e| format!("{:?}", e)).unwrap();
    let executor = runtime.executor();
    let mut network_config = NetworkConfig::new();
    network_config.apply_cli_args(args);
    let network_logger = log.new(o!("Service" => "Network"));
    let (network, network_send) = Service::new(
            &network_config,
            &executor,
            network_logger,
    ).unwrap();

    // run service until ctrl-c
    let (ctrlc_send, ctrlc_oneshot) = oneshot::channel();
    let ctrlc_send_c = RefCell::new(Some(ctrlc_send));
    ctrlc::set_handler(move || {
        if let Some(ctrlc_send) = ctrlc_send_c.try_borrow_mut().unwrap().take() {
            ctrlc_send.send(()).expect("Error sending ctrl-c message");
        }
    })
    .map_err(|e| format!("Could not set ctrlc handler: {:?}", e)).unwrap();

    let (exit_signal, exit) = exit_future::signal();

    run(&network, executor, exit, log.new(o!("Service" => "Notifier")));

    runtime
        .block_on(ctrlc_oneshot)
        .map_err(|e| format!("Ctrlc oneshot failed: {:?}", e)).unwrap();
}

pub fn run(
    network: &Service,
    executor: TaskExecutor,
    exit: Exit,
    log: slog::Logger
) {
    let err_log = log.clone();
    // notification heartbeat
    let interval = Interval::new(
        Instant::now(),
        Duration::from_secs(HEARTBEAT_INTERVAL_SECONDS),
    );

    let libp2p = network.libp2p_service();

    let heartbeat = move |_| {
        // Number of libp2p (not discv5) peers connected.
        //
        // Panics if libp2p is poisoned.
        let connected_peer_count = libp2p.lock().swarm.connected_peers();

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







