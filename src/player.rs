use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::character_controller::{CharacterController, CharacterControllerBundle, LookAcceleration, LookBundle, LookTarget, MovementAcceleration, MovementBundle, MovementDampening, MovementTarget, MovementVelocity};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
enum Action {
    Move,
}

#[derive(Component)]
pub struct PlayerMarker;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Action>::default());

        app.add_systems(Startup, startup);
        app.add_systems(FixedPreUpdate, (input, look).chain());
    }
}

fn startup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let input_map = InputMap::new([(
        Action::Move,
        VirtualDPad {
            up: InputKind::PhysicalKey(KeyCode::KeyW),
            down: InputKind::PhysicalKey(KeyCode::KeyS),
            left: InputKind::PhysicalKey(KeyCode::KeyA),
            right: InputKind::PhysicalKey(KeyCode::KeyD)
        }
    )]);


    commands.spawn((
        PlayerMarker,
        CharacterControllerBundle {
            movement_bundle: MovementBundle {
                acceleration: MovementAcceleration(5.0),
                velocity: MovementVelocity(2.0),
                damping: MovementDampening(5.0),
                ..Default::default()
            },
            look_bundle: LookBundle {
                acceleration: LookAcceleration(3.14),
                look_target: Vec2::X.into(),
            },
            ..Default::default()
        },
        InputManagerBundle::with_map(input_map),
        SceneBundle {
            scene: asset_server.load("meshes/character.glb#Scene0"),
            ..Default::default()
        }
    ));
}

fn input(
    mut query: Query<(
        &mut MovementTarget, &ActionState<Action>
    ), With<PlayerMarker>>
) {
    let (mut movement, action_state) = query.single_mut();

    if action_state.pressed(&Action::Move) {
        let axis = action_state.axis_pair(&Action::Move).unwrap();
        *movement = MovementTarget::Direction(axis.xy() * Vec2::new(1.0, -1.0));
    }
    else {
        *movement = MovementTarget::None;
    }

}

fn look(
    win_q: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,

    mut player_q: Query<(&mut LookTarget, &Position), With<PlayerMarker>>
) {
    let Ok(window) = win_q.get_single() else { return };
    let (camera, camera_transform) = q_camera.single();

    if let Some(cur_pos) = window.cursor_position() {
        let Ok((mut look_target, position)) = player_q.get_single_mut() else { return };

        let Some(ray) = camera.viewport_to_world(camera_transform, cur_pos) else { return };

        let world_pos = ray.origin + ray.direction * (ray.origin.y / ray.direction.y);

        look_target.0 = world_pos.xz() - position.xz();
        
    }
}
