
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::client::components::ComponentSyncMode;
use lightyear::prelude::client::Replicate;
use lightyear::prelude::*;
use lightyear::utils::bevy_xpbd_2d::{position, rotation};

use crate::networking::REPLICATION_GROUP;

mod server;
mod client;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerId>();
        app.register_component::<PlayerId>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_component::<Position>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(position::lerp)
            .add_correction_fn(position::lerp);

        app.register_component::<Rotation>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full)
            .add_interpolation(ComponentSyncMode::Full)
            .add_interpolation_fn(rotation::lerp)
            .add_correction_fn(rotation::lerp);

        app.register_component::<LinearVelocity>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full);

        app.register_component::<AngularVelocity>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full);

        app.add_plugins((
            LeafwingInputPlugin::<PlayerActions>::default(), 
            server::ServerPlugin, 
            client::ClientPlugin
        ));
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct PlayerId(pub ClientId);

#[derive(Bundle)]
pub struct PlayerBundle {
    player_id: PlayerId,

    position: Position,
    physics: PhysicsBundle,

    inputs: InputManagerBundle<PlayerActions>,

    replicate: Replicate,
    pre_predicted: PrePredicted
}

impl PlayerBundle {
    pub fn new(player_id: ClientId, inputs: InputManagerBundle<PlayerActions>) -> PlayerBundle {
        PlayerBundle {
            player_id: PlayerId(player_id),

            position: Position::new(Vec2::ZERO),
            physics: PhysicsBundle::new(
                RigidBody::Dynamic, 
                Collider::rectangle(16.0, 16.0)
            ),
            inputs,
            replicate: Replicate { 
                group: REPLICATION_GROUP,
                ..Default::default()
            },
            pre_predicted: PrePredicted::default()
        }
    }
}

#[derive(Bundle)]
pub struct PhysicsBundle {
    rigidbody: RigidBody,
    collider: Collider,
}

impl PhysicsBundle {
    fn new(rigidbody: RigidBody, collider: Collider) -> PhysicsBundle {
        PhysicsBundle {
            rigidbody,
            collider
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash, Reflect, Actionlike)]
pub enum PlayerActions {
    Up,
    Down,
    Left,
    Right,

    Look
}


fn player_input(
    mut velocity: Mut<LinearVelocity>,

    action: &ActionState<PlayerActions>,
    dt: f32
) {
    const SPEED: f32 = 500.0;

    let mut accum = Vec2::ZERO;
    for act in action.get_pressed() {
        match act {
            PlayerActions::Up =>    { accum.y += 1.0 },
            PlayerActions::Down =>  { accum.y -= 1.0 },
            PlayerActions::Left =>  { accum.x -= 1.0 },
            PlayerActions::Right => { accum.x += 1.0 },
            PlayerActions::Look => {
                
            },
        }
    }

    if accum.length() > 0.0 {
        **velocity += accum.normalize() * SPEED * dt;
    }
    else if velocity.length() > 0.0 {
        let len = (SPEED * dt).min(velocity.length());
        *velocity = (velocity.xy() - velocity.normalize() * len).into();
    }

    *velocity = velocity.clamp_length_max(100.0).into();
}

fn player_rot(
    mut ang_vel: Mut<AngularVelocity>,

    action: &ActionState<PlayerActions>,
    rotation: &Rotation
) {
    
    let rot = rotation.as_radians();
    let Some(target) = action.axis_pair(&PlayerActions::Look) else { return };

    ang_vel.0 = (target.y().atan2(target.x()) - rot) * 10.0;
}

