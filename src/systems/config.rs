use graphics::wgpu::{Backend, Backends};
use log::{LevelFilter, debug};
use rustls::{
    ClientConfig, RootCertStore, ServerConfig,
    client::danger,
    crypto::{CryptoProvider, ring as provider},
    pki_types::{CertificateDer, PrivateKeyDer},
    server::WebPkiClientVerifier,
};
use serde::{Deserialize, Serialize};
use std::{fs, io::BufReader, sync::Arc};

use crate::{Result, renderer::*};

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientLevelFilter {
    /// A level lower than all log levels.
    Off,
    /// Corresponds to the `Error` log level.
    Error,
    /// Corresponds to the `Warn` log level.
    Warn,
    /// Corresponds to the `Info` log level.
    Info,
    /// Corresponds to the `Debug` log level.
    Debug,
    /// Corresponds to the `Trace` log level.
    Trace,
}

impl ClientLevelFilter {
    pub fn parse_enum(&self) -> LevelFilter {
        match self {
            ClientLevelFilter::Off => LevelFilter::Off,
            ClientLevelFilter::Error => LevelFilter::Error,
            ClientLevelFilter::Warn => LevelFilter::Warn,
            ClientLevelFilter::Info => LevelFilter::Info,
            ClientLevelFilter::Debug => LevelFilter::Debug,
            ClientLevelFilter::Trace => LevelFilter::Trace,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: String,
    pub password: String,
    pub save_password: bool,
    pub bgm_volume: u8,
    pub sfx_volume: u8,
    pub reconnect_code: String,
    pub level_filter: ClientLevelFilter,
    pub enable_backtrace: bool,
    pub graphic_backend: String,
    pub show_fps: bool,
    pub show_ping: bool,
    pub show_average_ping: bool,
    pub show_frame_loop: bool,
    pub power_settings: ClientAdapterPowerSettings,
    pub present_mode: ClientPresentMode,
    pub gpu_instance: ClientGPUInstances,
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

    pub fn append_graphic_backend(&self) -> Backends {
        let text: Vec<&str> = self.graphic_backend.split('|').collect();
        let mut backends = Backends::empty();
        for data in text.iter() {
            match *data {
                "OpenGL" => backends |= Backends::GL,
                "DX12" => backends |= Backends::DX12,
                "Vulkan" => backends |= Backends::VULKAN,
                "Metal" => backends |= Backends::METAL,
                _ => {}
            }
        }
        debug!("Backends: {backends:?}",);
        backends
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: None,
            port: None,
            username: String::new(),
            password: String::new(),
            save_password: false,
            bgm_volume: 70,
            sfx_volume: 70,
            reconnect_code: String::new(),
            level_filter: ClientLevelFilter::Info,
            enable_backtrace: false,
            graphic_backend: "OpenGL|DX12|Vulkan|Metal".to_string(),
            show_fps: false,
            show_ping: false,
            show_average_ping: false,
            show_frame_loop: false,
            power_settings: ClientAdapterPowerSettings::HighPower,
            present_mode: ClientPresentMode::AutoVsync,
            gpu_instance: ClientGPUInstances::None,
        }
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

    panic!("no keys found in {filename:?} (encrypted keys not supported)");
}

pub fn build_tls_config() -> Result<Arc<rustls::ClientConfig>> {
    let mut root_store = RootCertStore::empty();
    let ca_cert = load_certs("keys/ca-crt.pem");
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
    .with_no_client_auth();

    Ok(Arc::new(config))
}
