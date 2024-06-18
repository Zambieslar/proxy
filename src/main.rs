#![allow(unused_imports, dead_code, unused_variables, unused_results)]
use std::time::Duration;
#[allow(unused_imports)]
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::*,
    prelude::*,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use definitions::{Request, StateMachine};
use openssl::{
    pkey::{Private, Public},
    rsa::Rsa,
    x509::X509,
};

use tls::{gen_key, init_tls_wrapper};
use traits::Machine;

mod definitions;
mod evaluation;
mod statemachine;
mod tls;
mod traits;

use crate::statemachine::*;

const ALLOWED_CIPHERS: &str = "ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-SHA384:ECDHE-RSA-AES256-SHA384:DHE-RSA-AES256-SHA256:ECDHE-ECDSA-AES128-SHA256:ECDHE-RSA-AES128-SHA256:DHE-RSA-AES128-SHA256:ECDHE-ECDSA-AES256-SHA:ECDHE-RSA-AES256-SHA:DHE-RSA-AES256-SHA:ECDHE-ECDSA-AES128-SHA:ECDHE-RSA-AES128-SHA:DHE-RSA-AES128-SHA";

fn main() {
    let mut machine = StateMachine::new();
    let (ssl_key, ssl_crt) = gen_key();
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Unable to bind socket");
    for client in listener.incoming() {
        match client {
            Ok(mut connection) => {
                let (tx, rx) = mpsc::channel();
                thread::spawn(move || {
                    let mut buf: Vec<u8> = Vec::new();
                    let mut stream = BufReader::new(&mut connection);
                    stream.read(&mut buf).unwrap();
                    tx.send(buf.clone()).unwrap();
                });
                loop {
                    if let Ok(result) = rx.recv() {
                        let raw: String = result.clone().into_iter().map(|x| x as char).collect();
                        println!("{}", raw);
                        let request = machine.run(result);
                        println!("{:#?}", request);
                        break;
                    } else {
                        continue;
                    };
                }
            }
            Err(error) => {}
        }
    }
}
