use bevy::prelude::*;
use lightyear::prelude::*;


use self::config::{local_client_config, server_config};

pub mod config;

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
    }
}

