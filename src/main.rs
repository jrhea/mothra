
extern crate getopts;
extern crate hobbits_libp2p_relay;

use hobbits_libp2p_relay::libp2p_wrapper::service;

fn main() {
    service::start();
}