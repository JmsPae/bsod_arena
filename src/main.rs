use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bsod_arena::GamePlugin;
use lightyear::shared::log::add_log_layer;

fn main() {


    App::new()
        .add_plugins(DefaultPlugins.build().set(LogPlugin {
        level: Level::INFO,
        filter: "wgpu=error,bevy_render=info,bevy_ecs=warn,bevy_app=warn,bevy_xpbd_2d=info".to_string(),
        update_subscriber: Some(add_log_layer)
    }))
        .add_plugins((
            WorldInspectorPlugin::new(),
        ))
        .add_plugins(GamePlugin)
        .run();
}
