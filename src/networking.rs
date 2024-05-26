use std::net::Ipv4Addr;

use bevy::prelude::*;
use lightyear::client::config::ClientConfig;
use lightyear::prelude::client::ClientCommands;
use lightyear::prelude::server::ServerCommands;
use lightyear::prelude::*;

use crate::state::NetState;

use self::config::{local_client_config, remote_client_config, server_config};

mod config;

pub const FIXED_UPDATE_HZ: f64 = 64.0;

pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(1);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message1(pub usize);

#[derive(Channel)]
pub struct Channel1;


pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            server::ServerPlugins { config: server_config() }, 
            client::ClientPlugins { config: local_client_config() }
        ));

        app.add_message::<Message1>(ChannelDirection::Bidirectional);

        app.add_channel::<Channel1>(ChannelSettings {
            mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
            ..default()
        });

        app.add_systems(OnEnter(NetState::Server), init_host)
            .add_systems(OnEnter(NetState::Client), init_client);
    }
}



fn init_host(mut commands: Commands,
) {
    commands.start_server();
    commands.connect_client();
}

fn init_client(mut commands: Commands,

    mut client_conf: ResMut<ClientConfig>,
) {
    *client_conf = remote_client_config(Ipv4Addr::new(127, 0, 0, 1));
    commands.connect_client();
}
