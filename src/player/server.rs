use bevy::prelude::*;
use lightyear::client::components::Confirmed;
use lightyear::client::prediction::Predicted;
use lightyear::prelude::server::*;
use lightyear::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::networking::REPLICATION_GROUP;
use crate::player::player_input;
use crate::player::PhysicsBundle;
use crate::FixedSet;

use super::player_rot;
use super::PlayerId;
use super::PlayerActions;

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {

        app.add_systems(
            PreUpdate,
            replicate_players
                .in_set(ServerReplicationSet::ClientReplication),
        );

        app.add_systems(FixedUpdate, 
            (movement, rotation)
                .chain()
                .in_set(FixedSet::MainServer)
        );
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
        (Without<Confirmed>, Without<Predicted>),
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
        (Without<Confirmed>, Without<Predicted>),
    >
) {
    for (ang_vel, look_target, rotation) in rot_query.iter_mut() {
        player_rot(ang_vel, look_target, rotation);
    }
}


fn replicate_players(
    mut commands: Commands,
    query: Query<(Entity, &Replicated), (Added<Replicated>, With<PlayerId>)>,
) {
    for (entity, replicated) in query.iter() {
        let client_id = replicated.client_id();
        info!("received player spawn event from client {client_id:?}");

        if let Some(mut e) = commands.get_entity(entity) {
            let mut sync_target = SyncTarget::default();

            sync_target.prediction = NetworkTarget::All;
            let replicate = server::Replicate {
                sync: sync_target,
                controlled_by: server::ControlledBy {
                    target: NetworkTarget::Single(client_id),
                },
                group: REPLICATION_GROUP,
                ..default()
            };
            e.insert((
                replicate,
                OverrideTargetComponent::<ActionState<PlayerActions>>::new(
                    NetworkTarget::AllExceptSingle(client_id),
                ),
                OverrideTargetComponent::<PrePredicted>::new(NetworkTarget::Single(client_id)),
                PhysicsBundle::new(
                    RigidBody::Dynamic, 
                    Collider::rectangle(16.0, 16.0)
                )
            ));
        }
    }
}
