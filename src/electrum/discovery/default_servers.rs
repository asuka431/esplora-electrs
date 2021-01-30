use crate::chain::Network;
use crate::electrum::discovery::{DiscoveryManager, Service};

pub fn add_default_servers(discovery: &DiscoveryManager, network: Network) {
    match network {
        #[cfg(not(feature = "liquid"))]
        Network::Fujicoin => {
            discovery
                .add_default_server(
                    "electrumx1.fujicoin.org".into(),
                    vec![Service::Tcp(50001), Service::Ssl(50002)],
                )
                .ok();
            discovery
                .add_default_server(
                    "electrumx2.fujicoin.org".into(),
                    vec![Service::Tcp(50001), Service::Ssl(50002)],
                )
                .ok();
            discovery
                .add_default_server(
                    "electrumx3.fujicoin.org".into(),
                    vec![Service::Tcp(50001), Service::Ssl(50002)],
                )
                .ok();
        }
        #[cfg(not(feature = "liquid"))]
        Network::Testnet => {
            
        }

        _ => (),
    }
}
