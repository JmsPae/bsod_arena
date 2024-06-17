use bevy::prelude::*;
use bevy_xpbd_3d::components::{Position, Rotation};

use crate::character_controller::LookTarget;
use crate::debug::DebugSettings;
use crate::player::PlayerMarker;


#[derive(Component)]
pub struct GameCursorElement;

#[derive(Component)]
pub struct GameReticleInner;

pub struct GameCursorPlugin;

impl Plugin for GameCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            add_cursor
        );
        app.add_systems(
            PreUpdate, 
            (update_cursor, update_reticle)
        );
    }
}

fn add_cursor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let inner_ret: Handle<Image> = asset_server.load("reticle_inner.png");
    let outer_ret: Handle<Image> = asset_server.load("reticle_outer.png");


    commands.spawn((
        ImageBundle {
            image: UiImage {
                texture: outer_ret,
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..Default::default()
            },
            z_index: ZIndex::Global(100),
            transform: Transform::default(),
            ..Default::default()
        },
        GameCursorElement
    ));


    commands.spawn((
        ImageBundle {
            image: UiImage {
                texture: inner_ret,
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                ..Default::default()
            },
            z_index: ZIndex::Global(99),
            transform: Transform::default(),
            ..Default::default()
        },
        GameReticleInner 
    ));
}

fn update_cursor(
    debug: Res<DebugSettings>,
    mut windows: Query<&mut Window>,
    mut cursor: Query<(&mut Style, &Node), With<GameCursorElement>>
) {
    
    let Ok(mut window) = windows.get_single_mut() else { return };
    let Ok((mut cursor, node)) = cursor.get_single_mut() else { return };

    if let Some(pos) = window.cursor_position() {
        cursor.left = Val::Px(pos.x - node.size().x * 0.5);
        cursor.top = Val::Px(pos.y - node.size().y * 0.5);
        cursor.display = Display::DEFAULT;

        window.cursor.visible = debug.enabled;
    }
    else {
        cursor.display = Display::None;
    }
}

fn update_reticle(
    mut cursor: Query<(&mut Style, &Node), With<GameReticleInner>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    player_q: Query<(&Position, &Rotation, &LookTarget), With<PlayerMarker>>
) {
    let Ok((mut cursor, node)) = cursor.get_single_mut() else { return };

    let Ok((camera, c_transform)) = camera_q.get_single() else { return };
    let Ok((p_position, p_rotation, p_look)) = player_q.get_single() else { return };

    let pos = p_position.0 + (p_rotation * Vec3::Z) * p_look.length();

    if let Some(c_pos) = camera.world_to_viewport(c_transform, pos) {
        cursor.left = Val::Px(c_pos.x - node.size().x * 0.5);
        cursor.top = Val::Px(c_pos.y - node.size().y * 0.5);
    }
} 
