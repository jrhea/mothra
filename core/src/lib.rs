pub mod mothra_api;
extern crate getopts;
use std::sync::mpsc::{channel,Sender,Receiver};
use std::{thread, time};
use std::ffi::CStr;
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
    fn libp2p_receive_gossip(message_c_char: *mut c_char);
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


    thread::spawn(move || {
        loop{
            let network_message = rx2.recv().unwrap();
            if network_message.command == "GOSSIP".to_string() {
                let message = String::from_utf8(network_message.value).unwrap();
                unsafe {
                    libp2p_receive_gossip(message.as_ptr() as *mut i8);
                }
            }
        }     
    });

}

#[no_mangle]
pub extern fn libp2p_send_gossip(message_c_char: *mut c_char) {
    let mut message_str: String = "".to_string();
    let message_cstr = unsafe { CStr::from_ptr(message_c_char) };
    match message_cstr.to_str() {
        Ok(s) => {
            message_str = s.to_string();
        }
        Err(_) => {
            std::println!("Invalid libp2p config provided! ");
        }
    }

    SEND.with(|tx_cell| {
        let message = Message {
            command: "GOSSIP".to_string(), 
            value: message_str.as_bytes().to_vec()
        };
        tx_cell.borrow_mut().as_mut().unwrap().send(message);
    });
}