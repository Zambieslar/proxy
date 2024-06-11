#[allow(unused_imports)]
use std::{net::*, prelude::*};
use std::{
    sync::{Arc, Mutex},
    thread,
};

use openssl::{
    pkey::{Private, Public},
    rsa::Rsa,
    x509::X509,
};
use tls::{gen_key, init_tls_wrapper};

mod locker;
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
    let listener = TcpListener::bind("127.0.0.1:8008").expect("Unable to bind socket");
    let key: Arc<Mutex<Rsa<Private>>> = Arc::new(Mutex::new(ssl_key.clone()));
    let crt: Arc<Mutex<X509>> = Arc::new(Mutex::new(ssl_crt.clone()));
    loop {
        match listener.accept() {
            Ok((_socket, addr)) => {
                thread::spawn(move || {
                    let key = Arc::clone(&key);
                    let crt = Arc::clone(&crt);
                    let acceptor = init_tls_wrapper(ssl_key, ssl_crt);
                });
            }
            Err(error) => {}
        }
    }
}
