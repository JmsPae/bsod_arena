use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use serde::{Serialize, Deserialize};

mod movement;

pub use movement::*;

use crate::FixedSet;

/// A marker component indicating that an entity is using a character controller.
#[derive(Component, Serialize, Deserialize, Default, Clone, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct CharacterController;


/// Where the character should be looking
#[derive(Component, Serialize, Deserialize, Default, Clone, Debug, Deref, DerefMut, PartialEq, Reflect)]
#[reflect(Component)]
pub struct LookTarget(pub Vec2);

impl From<Vec2> for LookTarget {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}

/// The rotation accel/decceleration 
#[derive(Component, Serialize, Deserialize, Default, Clone, Debug, Deref, DerefMut, PartialEq, Reflect)]
pub struct LookAcceleration(pub f32);

#[derive(Bundle)]
pub struct LookBundle {
    pub acceleration: LookAcceleration,
    pub look_target: LookTarget
}

impl Default for LookBundle {
    fn default() -> Self {
        Self {
            acceleration: LookAcceleration(0.0),
            look_target: Vec2::X.into()
        }
    }
}

#[derive(Bundle)]
pub struct CharacterControllerBundle {
    pub character_controller: CharacterController,

    pub rigid_body: RigidBody,
    pub collider: Collider,

    pub movement_bundle: MovementBundle,
    pub look_bundle: LookBundle
}

impl Default for CharacterControllerBundle {
    fn default() -> Self {
        let collider = Collider::capsule(0.875, 0.5);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Kinematic,
            collider,
            look_bundle: LookBundle::default(),
            movement_bundle: MovementBundle::default()
        }
    }
}

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CharacterController>();
        app.register_type::<MovementAcceleration>();
        app.register_type::<MovementVelocity>();
        app.register_type::<MovementDampening>();
        app.register_type::<MovementTarget>();
        app.register_type::<LookTarget>();
        app.register_type::<LookAcceleration>();

        app.add_systems(FixedUpdate, 
            (
                update_movement, 
                rotation
            )
                .chain()
                .in_set(FixedSet::Main)
        );
    }
}

fn update_movement(
    time: Res<Time<Fixed>>,
    mut characters: Query<(
        &Position, 
        &MovementTarget, 
        &MovementAcceleration, 
        &MovementDampening, 
        &MovementVelocity, 
        &mut LinearVelocity, 
    ), With<CharacterController>>
) {
                
    let dt = time.delta_seconds();
    for (_pos, target, accel, damp, mv_vel, mut vel) in characters.iter_mut() {
         
        match target {
            MovementTarget::None => {
                vel.0 = vel.0 - vel.0.normalize_or_zero() * (damp.0 * dt).min(vel.0.length());
            },
            MovementTarget::Direction(dir) => { 

                let diff = *dir * mv_vel.0 - vel.xz();

                vel.0 = vel.0 + Vec3::new(diff.x, 0.0, diff.y).normalize_or_zero() * accel.0 * dt; 
                
            },
            MovementTarget::Location(_loc) => {
                todo!("MovementTarget::Location not implemented.");
            },
        }
        if vel.0.length() > mv_vel.0 {
            vel.0 = vel.0 + vel.0.normalize() * (mv_vel.0 - vel.0.length());
        }
    }
}

fn rotation(
    timer: Res<Time<Fixed>>,
    mut characters: Query<(
        &LookAcceleration, 
        &LookTarget,
        &Rotation,
        &mut AngularVelocity 
    ), With<CharacterController>>
) {
    for (look_accel, look_target, rotation, mut ang_vel) in characters.iter_mut() {
        let forward = (rotation * Vec3::Z).xz();

        let angle = look_target.0.angle_between(forward) * look_accel.0;

        ang_vel.y = angle;
    }
}
