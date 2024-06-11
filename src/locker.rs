use openssl::{pkey::Private, rsa::Rsa, x509::X509};
use std::{
    boxed, error,
    sync::{Arc, Mutex, MutexGuard},
};

pub fn locker(
    key: &Arc<Mutex<Rsa<Private>>>,
    crt: &Arc<Mutex<X509>>,
) -> Result<(MutexGuard<Rsa<Private>>, MutexGuard<X509>), Box<dyn error::Error>> {
    let r1 = key.try_lock()?;
    let r2 = crt.try_lock()?;
    Ok((r1, r2))
}
