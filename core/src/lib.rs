pub mod mothra_api;
extern crate getopts;
use std::sync::mpsc::{channel,Sender,Receiver};
use std::{thread, time};
use std::ffi::{CStr,CString};
use std::os::raw::{c_uchar,c_char,c_int};
use std::io::Cursor;
use std::convert::TryFrom;
use std::mem;
use std::ops::Range;
use std::cell::RefCell;
use std::cell::RefMut;
use cast::i16;
use cast::i8;
use slog::{info, debug, warn, o, Drain};
use libp2p_wrapper::GossipData;
use mothra_api::api;

#[derive(Debug)]
struct Global {
    tx: RefCell<Option<Sender<GossipData>>>
}

static mut GLOBAL: Global = Global{ tx:  RefCell::new(None) };

impl Global {
    fn set(&mut self, value: &RefCell<Option<Sender<GossipData>>>) {
        self.tx.swap(value);
    }

    fn get(&self) -> RefMut<Option<Sender<GossipData>>> {
        self.tx.borrow_mut()
    }
}

macro_rules! set_tx {
    ($fmt:expr) => (
        unsafe {
            GLOBAL.set($fmt)
        }
    );
}

macro_rules! get_tx {
    () => (
        unsafe {
            GLOBAL.get()
        }
    );
}

extern {
    fn receive_gossip(topic_c_uchar: *mut c_uchar, topic_length: i16, data_c_uchar: *mut c_uchar, data_length: i16);
    //fn receive_rpc(message_c_char: *mut c_uchar, length: i16);
}

#[no_mangle]
pub extern fn libp2p_start(args_c_char: *mut *mut c_char, length: isize) {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let slog = slog::Logger::root(drain, o!());
    let log = slog.new(o!("API" => "init()"));
    let mut args_vec = Vec::<String>::new();
    for idx in 0..length {
        let args_cstr = unsafe { CStr::from_ptr(*args_c_char.offset(idx)) };
        match args_cstr.to_str() {
            Ok(s) => {
            args_vec.push(s.to_string());
            }
            Err(_) => {
                warn!(log,"Invalid libp2p config provided! ")
            }
        }
    }
    let args = api::config(args_vec);
    let (mut tx1, rx1) = channel();
    let (tx2, rx2) = channel();

    let nlog = log.clone();
    thread::spawn(move || {
        api::start(args, &tx2, &rx1, nlog.new(o!("API" => "start()")));
    });

    set_tx!(&RefCell::new(Some(tx1)));

    ///Listen for messages rcvd from the network
    thread::spawn(move || {
        loop{
            match rx2.recv(){
                Ok(mut network_message) => {
                    let topic_length = i16(network_message.topic.len()).unwrap();
                    let topic = network_message.topic.into_bytes().as_mut_ptr();
                    let data_length = i16(network_message.value.len()).unwrap();
                    let data = network_message.value.as_mut_ptr();
                    
                    unsafe {
                        receive_gossip(topic, topic_length, data, data_length);
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
pub extern fn libp2p_send_gossip(topic_c_uchar: *mut c_uchar, topic_length: usize, data_c_uchar: *mut c_uchar, data_length: usize) {
    let mut data = unsafe { std::slice::from_raw_parts_mut(data_c_uchar, data_length) };
    let topic = unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(topic_c_uchar, topic_length)) };
    let gossip_data = GossipData {
        topic: topic.to_string(), 
        value: data.to_vec()
    };
    get_tx!().as_mut().unwrap().send(gossip_data);
}

#[no_mangle]
pub extern fn libp2p_send_rpc(message_c_uchar: *mut c_uchar, length: usize) {
    // let mut message_bytes = unsafe { std::slice::from_raw_parts_mut(message_c_uchar, length) };

    // let message = Message {
    //     command: "RPC".to_string(), 
    //     value: message_bytes.to_vec()
    // };
    // get_tx!().as_mut().unwrap().send(message);
}