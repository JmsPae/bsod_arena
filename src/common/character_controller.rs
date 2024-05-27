use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use lightyear::client::components::ComponentSyncMode;
use lightyear::prelude::*;

mod client;
mod server;

/// A marker component indicating that an entity is using a character controller.
#[derive(Component, Serialize, Deserialize, Default, Clone, Debug, PartialEq, Reflect)]
pub struct CharacterController;

/// The acceleration used for character movement.
#[derive(Component, Serialize, Deserialize, Default, Clone, Debug, Deref, DerefMut, PartialEq, Reflect)]
pub struct MovementAcceleration(pub f32);

/// The damping factor used for slowing down movement.
#[derive(Component, Serialize, Deserialize, Default, Clone, Debug, Deref, DerefMut, PartialEq, Reflect)]
pub struct MovementDampingFactor(pub f32);


/// Where the character should be looking
#[derive(Component, Serialize, Deserialize, Default, Clone, Debug, Deref, DerefMut, PartialEq, Reflect)]
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
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
}

impl MovementBundle {
    pub fn new(acceleration: MovementAcceleration, damping: MovementDampingFactor) -> Self {
        Self { acceleration, damping }
    }
}

#[derive(Bundle)]
pub struct LookBundle {
    acceleration: LookAcceleration,
    look_target: LookTarget
}

impl LookBundle {
    pub fn new(acceleration: LookAcceleration, look_target: LookTarget) -> Self {
        Self { acceleration, look_target }
    }
}

#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    shape_caster: ShapeCaster,

    movement_bundle: MovementBundle,
    look_bundle: LookBundle
}

impl CharacterControllerBundle {
    pub fn new(rigid_body: RigidBody, collider: Collider, shape_caster: ShapeCaster, movement_bundle: MovementBundle, look_bundle: LookBundle) -> Self {
        Self { character_controller: CharacterController, rigid_body, collider, shape_caster, movement_bundle, look_bundle }
    }
}

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CharacterController>();
        app.register_component::<CharacterController>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_type::<MovementAcceleration>();
        app.register_component::<MovementAcceleration>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full);

        app.register_type::<MovementDampingFactor>();
        app.register_component::<MovementDampingFactor>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full);

        app.register_type::<LookTarget>();
        app.register_component::<LookTarget>(ChannelDirection::Bidirectional)
            .add_interpolation(ComponentSyncMode::Full)
            .add_prediction(ComponentSyncMode::Full);

        app.register_type::<LookAcceleration>();
        app.register_component::<LookAcceleration>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full);

        app.add_plugins((server::ServerPlugin, client::ClientPlugin));
    }
}
