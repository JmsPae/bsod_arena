use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use serde::{Serialize, Deserialize};

/// The acceleration used for character movement.
#[derive(Component, Serialize, Deserialize, Default, Clone, Debug, Deref, DerefMut, PartialEq, Reflect)]
#[reflect(Component)]
pub struct MovementAcceleration(pub f32);

/// Maximum movement velocity
#[derive(Component, Serialize, Deserialize, Default, Clone, Debug, Deref, DerefMut, PartialEq, Reflect)]
#[reflect(Component)]
pub struct MovementVelocity(pub f32);

/// Rate at which the character is slowed down.
#[derive(Component, Serialize, Deserialize, Default, Clone, Debug, Deref, DerefMut, PartialEq, Reflect)]
#[reflect(Component)]
pub struct MovementDampening(pub f32);


#[derive(Component, Serialize, Deserialize, Default, Clone, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub enum MovementTarget {
    #[default]
    None,
    Direction(Vec2),
    Location(Vec2)
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub acceleration: MovementAcceleration,
    pub velocity: MovementVelocity,
    pub damping: MovementDampening,
    pub target: MovementTarget
}

impl MovementBundle {
    pub fn new(acceleration: MovementAcceleration, damping: MovementDampening) -> Self {
        Self { acceleration, damping, ..Default::default() }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self {
            acceleration: MovementAcceleration(2.5),
            velocity: MovementVelocity(5.0),
            damping: MovementDampening(0.8),
            target: MovementTarget::default()
        }
    }
}
