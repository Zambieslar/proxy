#![allow(unused_imports, dead_code, unused_variables, unused_results)]
use std::{
    io::{BufRead, BufReader, Read, Write},
    sync::{mpsc, Arc, Mutex},
    thread,
};
#[allow(unused_imports)]
use std::{net::*, prelude::*};

use openssl::{
    pkey::{Private, Public},
    rsa::Rsa,
    x509::X509,
};
use tls::{gen_key, init_tls_wrapper};

mod deserialize;
mod tls;

const ALLOWED_CIPHERS: &str = "ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384:DHE-RSA-AES256-GCM-SHA384:ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256:DHE-RSA-AES128-GCM-SHA256:ECDHE-ECDSA-AES256-SHA384:ECDHE-RSA-AES256-SHA384:DHE-RSA-AES256-SHA256:ECDHE-ECDSA-AES128-SHA256:ECDHE-RSA-AES128-SHA256:DHE-RSA-AES128-SHA256:ECDHE-ECDSA-AES256-SHA:ECDHE-RSA-AES256-SHA:DHE-RSA-AES256-SHA:ECDHE-ECDSA-AES128-SHA:ECDHE-RSA-AES128-SHA:DHE-RSA-AES128-SHA";
fn main() {
    let (ssl_key, ssl_crt) = gen_key();
    let serialized_cert: String = ssl_crt
        .public_key()
        .unwrap()
        .public_key_to_pem()
        .unwrap()
        .into_iter()
        .map(|x| x as char)
        .collect();
    let serialized_key: String = ssl_key
        .private_key_to_pem()
        .unwrap()
        .into_iter()
        .map(|x| x as char)
        .collect();
    let key: Arc<Mutex<Rsa<Private>>> = Arc::new(Mutex::new(ssl_key.clone()));
    let crt: Arc<Mutex<X509>> = Arc::new(Mutex::new(ssl_crt.clone()));
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Unable to bind socket");
    for client in listener.incoming() {
        match client {
            Ok(conn_req) => {
                let (tx, rx) = mpsc::channel();
                let r1 = key.try_lock().unwrap();
                let r2 = crt.try_lock().unwrap();
                let acceptor = init_tls_wrapper(r1.clone(), r2.clone());
                thread::spawn(move || {
                    let mut buf: Vec<u8> = Vec::new();
                    match acceptor.accept(conn_req.try_clone().unwrap()) {
                        Ok(mut stream) => {
                            let mut packet = BufReader::new(&mut stream);
                            println!("New connection from {:?}", conn_req.peer_addr());
                            packet.read_until(10, &mut buf);
                            for byte in &buf {}
                        }
                        Err(error) => {
                            println!("{:#?}", error)
                        }
                    }
                });
                loop {
                    if let Ok(result) = rx.recv() {
                        let response: String = result.into_iter().map(|x| x as char).collect();
                        println!("{}", response);
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

//let key = Arc::clone(&key);
//let crt = Arc::clone(&crt);
//let tx = tx.clone();
//thread::spawn(move || loop {
//    let tx = tx.clone();
//    match (r1, r2) {
//        (Ok(r1), Ok(r2)) => {
//            let mut buf: Vec<u8> = Vec::new();
//            acceptor
//                .accept(stream.try_clone().unwrap())
//                .unwrap()
//                .read(&mut buf);
//            tx.send(buf.clone());
//        }
//        _ => continue,
//    }
