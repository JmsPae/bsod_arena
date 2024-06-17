use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Resource)]
pub struct DebugSettings {
    pub enabled: bool
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WorldInspectorPlugin::default().run_if(is_debug));

        app.insert_resource(DebugSettings {
            enabled: true
        });

        app.add_systems(PreUpdate, toggle_debug.run_if(resource_changed::<ButtonInput<KeyCode>>));
    }
}

fn is_debug(
    deb: Res<DebugSettings>
) -> bool {
    deb.enabled
}

fn toggle_debug(
    mut mutdeb: ResMut<DebugSettings>,
    keys: Res<ButtonInput<KeyCode>>
) {
    if keys.just_pressed(KeyCode::F11) {
        mutdeb.enabled = !mutdeb.enabled;
    }
}
