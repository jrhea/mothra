pub mod mothra_api;
extern crate getopts;
use std::sync::mpsc::{channel,Sender,Receiver};
use std::{thread, time};
use std::ffi::{CStr,CString};
use std::os::raw::{c_char,c_int};
use std::io::Cursor;
use std::convert::TryFrom;
use std::mem;
use std::ops::Range;
use std::cell::RefCell;
use slog::{info, debug, warn, o, Drain};
use libp2p_wrapper::Message;
use mothra_api::api;

thread_local!(static SEND: RefCell<Option<Sender<Message>>> = RefCell::new(None));

extern {
    fn receive_gossip(message_c_char: *mut c_char);
}

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
    let (mut tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    let nlog = log.clone();
    thread::spawn(move || {
        api::start(args, &tx2, &rx1, nlog.new(o!("API" => "start()")));
    });
    
    SEND.with(|tx_cell| {
        tx_cell.swap(&RefCell::new(Some(tx1)));
    });

    ///Listen for messages rcvd from the network
    thread::spawn(move || {
        loop{
            match rx2.recv(){
                Ok(network_message) => {
                    if network_message.command == "GOSSIP".to_string() {
                        match CString::new(network_message.value){ //adds null terminator
                            Ok(message_c_str) => {
                                let mut message_ptr = message_c_str.as_ptr();
                                unsafe {
                                    receive_gossip(message_ptr as *mut i8);
                                }
                            }
                            Err(_) => {
                                std::println!("Rcv Thread Error: CString::new(message) ");
                            }
                        }
                    }
                }
                Err(_) => {
                    std::println!("Rcv Thread Error: rx2.recv().unwrap()");
                }
            }
        }     
    });

}

#[no_mangle]
pub extern fn libp2p_send_gossip(message_c_char: *mut c_char) {
    let message_cstr = unsafe { CStr::from_ptr(message_c_char) };
    let mut message_bytes: Vec::<u8> = message_cstr.to_bytes().to_vec();
    SEND.with(|tx_cell| {
        let message = Message {
            command: "GOSSIP".to_string(), 
            value: message_bytes
        };
        tx_cell.borrow_mut().as_mut().unwrap().send(message);
    });
}