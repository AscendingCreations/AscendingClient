use rustls::{
    client::danger,
    crypto::{ring as provider, CryptoProvider},
    pki_types::{CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
    ClientConfig, RootCertStore, ServerConfig,
};
use serde::{Deserialize, Serialize};
use std::{fs, io::BufReader, sync::Arc};

use crate::Result;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: String,
    pub password: String,
    pub save_password: bool,
}

impl Config {
    pub fn read_config(path: &str) -> Self {
        match fs::read_to_string(path) {
            Ok(data) => toml::from_str(&data).unwrap(),
            Err(_) => {
                let config = Config::default();
                config.save_config(path);
                config
            }
        }
    }

    pub fn save_config(&self, path: &str) {
        let toml_data = toml::to_string(self).unwrap();
        fs::write(path, toml_data).unwrap();
    }
}

fn load_certs(filename: &str) -> Vec<CertificateDer<'static>> {
    let certfile =
        fs::File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader)
        .map(|result| result.unwrap())
        .collect()
}

fn load_private_key(filename: &str) -> PrivateKeyDer<'static> {
    let keyfile =
        fs::File::open(filename).expect("cannot open private key file");
    let mut reader = BufReader::new(keyfile);

    loop {
        match rustls_pemfile::read_one(&mut reader)
            .expect("cannot parse private key .pem file")
        {
            Some(rustls_pemfile::Item::Pkcs1Key(key)) => return key.into(),
            Some(rustls_pemfile::Item::Pkcs8Key(key)) => return key.into(),
            Some(rustls_pemfile::Item::Sec1Key(key)) => return key.into(),
            None => break,
            _ => {}
        }
    }

    panic!(
        "no keys found in {:?} (encrypted keys not supported)",
        filename
    );
}

pub fn build_tls_config(certs_path: &str) -> Result<Arc<rustls::ClientConfig>> {
    let mut root_store = RootCertStore::empty();
    let ca_cert = load_certs("keys/ca-crt.pem");
    let certs = load_certs("keys/client.crt");
    let private_key = load_private_key("keys/client-key.pem");
    root_store.add_parsable_certificates(ca_cert);

    let config = ClientConfig::builder_with_provider(
        CryptoProvider {
            cipher_suites: provider::DEFAULT_CIPHER_SUITES.to_vec(),
            ..provider::default_provider()
        }
        .into(),
    )
    .with_protocol_versions(rustls::DEFAULT_VERSIONS)
    .expect("inconsistent cipher-suite/versions selected")
    .with_root_certificates(root_store)
    .with_client_auth_cert(certs, private_key)
    .unwrap();

    Ok(Arc::new(config))
}
