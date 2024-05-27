use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionData;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::buttonlike::ButtonState;
use leafwing_input_manager::plugin::InputManagerSystem;
use lightyear::client::prediction::Predicted;
use lightyear::prelude::client::{ClientConnection, NetClient};
use lightyear::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::player::{player_input, PhysicsBundle};
use crate::state::{NetState, State};
use crate::FixedSet;

use super::{player_rot, PlayerActions, PlayerId};


pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, 
            (movement, rotation)
                .chain()
                .in_set(FixedSet::Main)
                .run_if(in_state(State::Game))
                .run_if(in_state(NetState::ClientServer))
                .run_if(in_state(NetState::Client))
        )
        .add_systems(
            PreUpdate,
            update_player_mouse
                .in_set(InputManagerSystem::ManualControl)
                .run_if(in_state(State::Game))
                .run_if(in_state(NetState::ClientServer))
        );

        app.add_systems(Update, add_player_physics);
    }
}

fn update_player_mouse(
    window_query: Query<(&Window, &ActionStateDriver<PlayerActions>)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,

    mut action_state_query: Query<(&Position, &mut ActionState<PlayerActions>), With<Predicted>>
){
    let Ok((window, driver)) = window_query.get_single() else { return };

    let Some(cursor_pos) = window.cursor_position() else { return };
    let Ok((camera, camera_transform)) = camera_query.get_single() else { return };

    let Some(world_curs) = camera.viewport_to_world(camera_transform, cursor_pos) else { return };

    for entity in driver.targets.iter() {
        let Ok((position, mut action_state)) = action_state_query.get_mut(*entity) else { warn!("Couldn't get action state query!"); return };
        
        let dir = (world_curs.origin.xy() - position.xy()).normalize();

        let action_data = action_state.action_data_mut(&driver.action);

        if let Some(data) = action_data {
            data.axis_pair = Some(DualAxisData::from_xy(dir));
            action_state.press(&PlayerActions::Look);
        }
        else {
            action_state.set_action_data(PlayerActions::Look, 
                ActionData {
                    state: ButtonState::Pressed,
                    axis_pair: Some(DualAxisData::from_xy(dir)),
                    ..Default::default()
                }
            );
        }
        

    }
}

fn movement(
    tick_manager: Res<TickManager>,
    mut action_query: Query<
        (
            &mut LinearVelocity,
            &ActionState<PlayerActions>,
        ),
        // if we run in host-server mode, we don't want to apply this system to the local client's entities
        // because they are already moved by the client plugin
        With<Predicted>,
    >,
) {
    let dt = tick_manager.config.tick_duration.as_secs_f32();
    for (velocity, action) in action_query.iter_mut() {
        if !action.get_pressed().is_empty() ||
            velocity.length() > 0.0 {
            // NOTE: be careful to directly pass Mut<PlayerPosition>
            // getting a mutable reference triggers change detection, unless you use `as_deref_mut()`
            player_input(velocity, action, dt);
        }
    }
}

fn rotation(
    mut rot_query: Query<
        (
            &mut AngularVelocity,
            &ActionState<PlayerActions>,
            &Rotation
        ),
        With<Predicted>,
    >
) {
    for (ang_vel, look_target, rotation) in rot_query.iter_mut() {
        player_rot(ang_vel, look_target, rotation);
    }
}

fn add_player_physics(
    connection: Res<ClientConnection>,
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &PlayerId),
        (
            // insert the physics components on the player that is displayed on screen
            Added<Predicted>,
        ),
    >,
) {
    let client_id = (*connection).id();
    for (entity, player_id) in player_query.iter_mut() {
        if player_id.0 == client_id {
            continue;
        }
        info!(?entity, ?player_id, "adding physics to predicted player");
        commands.entity(entity).insert(PhysicsBundle::new(
                RigidBody::Dynamic, 
                Collider::rectangle(16.0, 16.0)
            ));
    }
}
