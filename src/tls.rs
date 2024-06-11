use std::{ops::Deref, time};

use crate::ALLOWED_CIPHERS;
use asn1::Asn1Time;
use openssl::*;
use pkcs7::Pkcs7;
use pkey::{PKey, PKeyRef, Private, Public};
use rsa::Rsa;
use ssl::{SslAcceptor, SslAcceptorBuilder, SslContextBuilder, SslMethod, SslVersion};
use x509::{
    extension::{self, BasicConstraints, ExtendedKeyUsage, SubjectKeyIdentifier},
    X509Builder, X509Extension, X509NameBuilder, X509,
};

pub fn init_tls_wrapper(ssl_key: Rsa<Private>, ssl_crt: X509) -> SslAcceptor {
    let mut acceptor = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls()).unwrap();
    acceptor.set_cipher_list(ALLOWED_CIPHERS).unwrap();
    acceptor.set_security_level(2);
    acceptor
        .set_min_proto_version(Some(SslVersion::TLS1_2))
        .unwrap();
    acceptor
        .set_private_key(&PKey::from_rsa(ssl_key).unwrap())
        .unwrap();
    acceptor.set_certificate(&ssl_crt).unwrap();
    acceptor.build()
}

pub fn gen_key() -> (Rsa<Private>, X509) {
    let priv_key = Rsa::generate(2048).unwrap();
    let pub_key = PKey::from_rsa(priv_key.clone()).unwrap();
    let bc = BasicConstraints::new().pathlen(0).build().unwrap();
    let key_usage = extension::KeyUsage::new()
        .key_agreement()
        .key_encipherment()
        .digital_signature()
        .non_repudiation()
        .build()
        .unwrap();
    let mut x509_name = X509NameBuilder::new().unwrap();
    x509_name.append_entry_by_text("C", "US").unwrap();
    x509_name.append_entry_by_text("ST", "UT").unwrap();
    x509_name.append_entry_by_text("O", "ZambieBam").unwrap();
    x509_name.append_entry_by_text("CN", "172.24.1.6").unwrap();
    let mut x509_builder = X509Builder::new().unwrap();
    x509_builder.append_extension(bc).unwrap();
    x509_builder.append_extension(key_usage).unwrap();
    x509_builder.set_pubkey(&pub_key).unwrap();
    x509_builder.set_not_before(&Asn1Time::days_from_now(0).unwrap());
    x509_builder.set_not_after(&Asn1Time::days_from_now(365).unwrap());
    x509_builder.set_subject_name(&x509_name.build()).unwrap();
    let cert = x509_builder.build();
    (priv_key, cert)
}
