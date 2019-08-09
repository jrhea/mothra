pub mod mothra_api;
extern crate getopts;
use std::sync::mpsc;
use std::{thread, time};
use std::ffi::CStr;
use std::os::raw::{c_char,c_int};
use std::io::Cursor;
use std::convert::TryFrom;
use std::mem;
use std::ops::Range;
use slog::{info, debug, warn, o, Drain};
use libp2p_wrapper::Message;
use mothra_api::api;

#[no_mangle]
pub extern fn libp2p_start(args_c_char: *mut *mut c_char, length: c_int) {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let slog = slog::Logger::root(drain, o!());
    let log = slog.new(o!("API" => "init()"));
    let mut args_vec = Vec::<String>::new();
    let mut idx: isize = 0;
    for i in 0..length {
        let args_cstr = unsafe { CStr::from_ptr(*args_c_char.offset(idx)) };
        match args_cstr.to_str() {
            Ok(s) => {
            args_vec.push(s.to_string());
            }
            Err(_) => {
            warn!(log,"Invalid libp2p config provided! ")
            }
        }
        idx += 1;
    }
    let args = api::config(args_vec);
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    let nlog = log.clone();
    thread::spawn(move || {
        api::start(args, &tx2, &rx1, nlog.new(o!("API" => "start()")));
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
            info!(log,"Receieved the following message from the network {:?}",String::from_utf8(network_message.value))
        }
    }
}