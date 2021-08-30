use rustls::{ServerConfig, NoClientAuth};
use rustls::internal::pemfile::{certs, pkcs8_private_keys};

pub fn load_ssl() -> ServerConfig {
    use std::io::BufReader;

    const CERT: &'static [u8] = include_bytes!("../cert/192.168.0.7.pem");
    const KEY: &'static [u8] = include_bytes!("../cert/192.168.0.7-key.pem");

    let mut cert = BufReader::new(CERT);
    let mut key = BufReader::new(KEY);

    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_chain = certs(&mut cert).unwrap();
    let mut keys = pkcs8_private_keys(&mut key).unwrap();
    config
        .set_single_cert(cert_chain, keys.remove(0)).unwrap();

    config
}