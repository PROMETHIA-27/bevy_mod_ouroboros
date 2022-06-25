use std::fs;

use ouroboros_common::bevy::prelude::{App, Plugin};
use ouroboros_common::quinn::ClientConfig;
use ouroboros_common::rustls::{Certificate, RootCertStore};
use ouroboros_common::CommonPlugin;

mod interface;
mod systems;

pub use interface::ClientInterfaceExt;

use self::systems::run_client;

pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CommonPlugin(run_client));
    }
}

fn client_config() -> ClientConfig {
    let cert = Certificate(fs::read("certificate.der").unwrap());

    let mut store = RootCertStore::empty();
    store.add(&cert).unwrap();

    ClientConfig::with_root_certificates(store)
}