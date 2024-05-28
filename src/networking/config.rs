use std::net::{Ipv4Addr, SocketAddr};
use std::time::Duration;

use lightyear::client::config::ClientConfig;
use lightyear::connection::netcode::PRIVATE_KEY_BYTES;
use lightyear::prelude::server::ServerTransport;
use lightyear::prelude::*;
use lightyear::server::config::ServerConfig;

use super::FIXED_UPDATE_HZ;

fn shared_config() -> SharedConfig {
    SharedConfig {
        client_send_interval: Duration::default(),
        server_send_interval: Duration::from_secs_f64(0.1),
        tick: TickConfig {
            tick_duration: Duration::from_secs_f64(1.0 / FIXED_UPDATE_HZ),
        },
        mode: Mode::HostServer
    }
}

pub fn server_config() -> ServerConfig {
    let netcode_config = server::NetcodeConfig::default();

    let conditioner = Some(LinkConditionerConfig {
        incoming_loss: 0.02,
        incoming_latency: Duration::from_millis(20),
        incoming_jitter: Duration::from_millis(5)
    });

    let io_config = server::IoConfig {
        transport: ServerTransport::UdpSocket(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 4000)) ,
        conditioner,
        compression: CompressionConfig::None,
    };

    let net_conf = server::NetConfig::Netcode { 
        config: netcode_config,
        io: io_config
    };

    ServerConfig {
        shared: shared_config(),
        net: vec![
           net_conf 
        ],
        ..Default::default()
    }
}

pub fn empty_server_config() -> ServerConfig {
    ServerConfig {
        shared: shared_config(),
        net: vec![
        ],
        ..Default::default()
    }
}

pub fn local_client_config() -> ClientConfig {
     ClientConfig {
        shared: shared_config(),
        net: client::NetConfig::Local {
            id: 0,
        },
        ..Default::default()
    }
}


pub fn remote_client_config(address: Ipv4Addr) -> ClientConfig {
    let config = client::NetcodeConfig::default();

    let conditioner = LinkConditionerConfig {
        incoming_loss: 0.02,
        incoming_latency: Duration::from_millis(20),
        incoming_jitter: Duration::from_millis(5)
    };

    let io = client::IoConfig::from_transport(client::ClientTransport::UdpSocket(SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 0)))
        .with_conditioner(conditioner);

    let net = client::NetConfig::Netcode { 
        auth: client::Authentication::Manual {
            server_addr: SocketAddr::new(address.into(), 4000),
            client_id: 1,
            private_key: [0; PRIVATE_KEY_BYTES],
            protocol_id: 0
        },
        config,
        io
    };

    let prediction = client::PredictionConfig {
        always_rollback: false,
        input_delay_ticks: 0,
        correction_ticks_factor: 1.5
    };

    ClientConfig {
        shared: shared_config(),
        net,
        prediction,
        ..Default::default()
    }
}
