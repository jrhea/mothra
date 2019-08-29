use super::error;
use libp2p_wrapper::NetworkConfig;
use libp2p_wrapper::Service as LibP2PService;
use libp2p_wrapper::{Message,RPC,GOSSIP,DISCOVERY};
use libp2p_wrapper::{Libp2pEvent, PeerId};
use libp2p_wrapper::{RPCEvent,RPCRequest};
use libp2p_wrapper::Topic;
use futures::prelude::*;
use futures::Stream;
use parking_lot::Mutex;
use slog::{warn,debug, info, o};
use std::sync::Arc;
use std::sync::mpsc as sync;
use tokio::runtime::TaskExecutor;
use tokio::sync::{mpsc, oneshot};

pub struct Network {
    libp2p_service: Arc<Mutex<LibP2PService>>,
    _libp2p_exit: oneshot::Sender<()>,
    _network_send: mpsc::UnboundedSender<NetworkMessage>,
}

impl Network {
    pub fn new(
        tx: sync::Sender<Message>,
        config: &NetworkConfig,
        executor: &TaskExecutor,
        log: slog::Logger,
    ) -> error::Result<(Arc<Self>, mpsc::UnboundedSender<NetworkMessage>)> {
        // build the network channel
        let (network_send, network_recv) = mpsc::unbounded_channel::<NetworkMessage>();
        // launch libp2p Network
        let libp2p_log = log.new(o!("Network" => "Libp2p"));
        let libp2p_service = Arc::new(Mutex::new(LibP2PService::new(config.clone(), std::sync::Mutex::new(tx.clone()), libp2p_log)?));
        let libp2p_exit = spawn_service(
            libp2p_service.clone(),
            network_recv,
            network_send.clone(),
            std::sync::Mutex::new(tx),
            executor,
            log,
        )?;
        let network_service = Network {
            libp2p_service,
            _libp2p_exit: libp2p_exit,
            _network_send: network_send.clone(),
        };

        Ok((Arc::new(network_service), network_send))
    }

    pub fn libp2p_service(&self) -> Arc<Mutex<LibP2PService>> {
        self.libp2p_service.clone()
    }
}

fn spawn_service(
    libp2p_service: Arc<Mutex<LibP2PService>>,
    network_recv: mpsc::UnboundedReceiver<NetworkMessage>,
    network_send: mpsc::UnboundedSender<NetworkMessage>,
    tx: std::sync::Mutex<sync::Sender<Message>>,
    executor: &TaskExecutor,
    log: slog::Logger,
) -> error::Result<tokio::sync::oneshot::Sender<()>> {
    let (network_exit, exit_rx) = tokio::sync::oneshot::channel();

    // spawn on the current executor
    executor.spawn(
        network_service(
            libp2p_service,
            network_recv,
            network_send,
            tx,
            log.clone(),
        )
        // allow for manual termination
        .select(exit_rx.then(|_| Ok(())))
        .then(move |_| {
            info!(log.clone(), "Network shutdown");
            Ok(())
        }),
    );

    Ok(network_exit)
}

fn network_service(
    libp2p_service: Arc<Mutex<LibP2PService>>,
    mut network_recv: mpsc::UnboundedReceiver<NetworkMessage>,
    _network_send: mpsc::UnboundedSender<NetworkMessage>,
    tx: std::sync::Mutex<sync::Sender<Message>>,
    log: slog::Logger,
) -> impl futures::Future<Item = (), Error = libp2p_wrapper::error::Error> {
    futures::future::poll_fn(move || -> Result<_, libp2p_wrapper::error::Error> {
        loop {
            // poll the network channel
            match network_recv.poll() {
                Ok(Async::Ready(Some(message))) => match message {
                    NetworkMessage::Send(peer_id, outgoing_message) => match outgoing_message {
                        OutgoingMessage::RPC(rpc_event) => {
                            //debug!(log, "Sending RPC Event: {:?}", rpc_event);
                            libp2p_service.lock().swarm.send_rpc(peer_id, rpc_event);
                        }
                    },
                    NetworkMessage::Publish { topics, message } => {
                        //debug!(log, "Sending pubsub message"; "topics" => format!("{:?}",topics));
                        libp2p_service.lock().swarm.publish(topics, message);
                    }
                },
                Ok(Async::NotReady) => break,
                Ok(Async::Ready(None)) => {
                    return Err(libp2p_wrapper::error::Error::from("Network channel closed"));
                }
                Err(_) => {
                    return Err(libp2p_wrapper::error::Error::from("Network channel error"));
                }
            }
        }
        loop {
            // poll the swarm
            match libp2p_service.lock().poll() {
                Ok(Async::Ready(Some(event))) => match event {
                    Libp2pEvent::RPC(_peer_id, rpc_event) => {
                        //debug!(log, "RPC Event: RPC message received: {:?}", rpc_event);
                         match rpc_event {
                            RPCEvent::Request(_, request) => {
                                match request {
                                    RPCRequest::Message(data) => {
                                        tx.lock().unwrap().send(Message {
                                            category: RPC.to_string(),
                                            command: "HELLO".to_string(),      //TODO: need to fix this when i properly package the payload
                                            peer: _peer_id.to_string(),
                                            value: data
                                        }).unwrap();
                                    }
                                }
                            },
                            RPCEvent::Response(id,_) => {
                                // ?????
                            },
                            RPCEvent::Error(_,_) =>{
                                warn!(log,"RPCEvent Error");
                            }
                        }
                    }
                    Libp2pEvent::PeerDialed(_peer_id) => {
                        tx.lock().unwrap().send(Message {
                            category: DISCOVERY.to_string(),
                            command: Default::default(),
                            peer: _peer_id.to_string(),
                            value: Default::default()
                        }).unwrap();
                    }
                    Libp2pEvent::PeerDisconnected(peer_id) => {
                        debug!(log, "Peer Disconnected: {:?}", peer_id);
                    }
                    Libp2pEvent::PubsubMessage {
                        source: _, message: _, ..
                    } => {

                    } 
                },
                Ok(Async::Ready(None)) => unreachable!("Stream never ends"),
                Ok(Async::NotReady) => break,
                Err(_) => break,
            }
        }
        Ok(Async::NotReady)
    })
}

/// Types of messages that the network Network can receive.
#[derive(Debug)]
pub enum NetworkMessage {
    /// Send a message to libp2p Network.
    //TODO: Define typing for messages across the wire
    Send(PeerId, OutgoingMessage),
    /// Publish a message to pubsub mechanism.
    Publish {
        topics: Vec<Topic>,
        message: Vec<u8>,
    },
}

/// Type of outgoing messages that can be sent through the network Network.
#[derive(Debug)]
pub enum OutgoingMessage {
    /// Send an RPC request/response.
    RPC(RPCEvent),
}
