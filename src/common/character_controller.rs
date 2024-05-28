use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use lightyear::client::components::ComponentSyncMode;
use lightyear::prelude::*;

mod client;
mod server;

/// A marker component indicating that an entity is using a character controller.
#[derive(Component, Serialize, Deserialize, Default, Clone, Debug, PartialEq, Reflect)]
#[reflect(Component)]
pub struct CharacterController;

/// The acceleration used for character movement.
#[derive(Component, Serialize, Deserialize, Default, Clone, Debug, Deref, DerefMut, PartialEq, Reflect)]
#[reflect(Component)]
pub struct MovementAcceleration(pub f32);

/// The damping factor used for slowing down movement.
#[derive(Component, Serialize, Deserialize, Default, Clone, Debug, Deref, DerefMut, PartialEq, Reflect)]
#[reflect(Component)]
pub struct MovementDamping(pub f32);

#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDamping,
}

impl MovementBundle {
    pub fn new(acceleration: MovementAcceleration, damping: MovementDamping) -> Self {
        Self { acceleration, damping }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self {
            acceleration: MovementAcceleration(10.0),
            damping: MovementDamping(5.0)
        }
    }
}


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
    acceleration: LookAcceleration,
    look_target: LookTarget
}

impl LookBundle {
    pub fn new(acceleration: LookAcceleration, look_target: LookTarget) -> Self {
        Self { acceleration, look_target }
    }
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
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    shape_caster: ShapeCaster,

    movement_bundle: MovementBundle,
    look_bundle: LookBundle
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider, shape_caster: ShapeCaster, movement_bundle: MovementBundle, look_bundle: LookBundle) -> Self {
        Self { 
            character_controller: CharacterController, 
            rigid_body: RigidBody::Kinematic, 
            collider, 
            shape_caster, 
            movement_bundle, 
            look_bundle 
        }
    }
}

impl Default for CharacterControllerBundle {
    fn default() -> Self {
        let collider = Collider::circle(1.0);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Kinematic,
            shape_caster: ShapeCaster::new(collider.clone(), Vec2::ZERO, 0.0, Direction2d::X).with_max_time_of_impact(1.0),
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
        app.register_component::<CharacterController>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Once)
            .add_interpolation(ComponentSyncMode::Once);

        app.register_type::<MovementAcceleration>();
        app.register_component::<MovementAcceleration>(ChannelDirection::Bidirectional)
            .add_prediction(ComponentSyncMode::Full);

        app.register_type::<MovementDamping>();
        app.register_component::<MovementDamping>(ChannelDirection::Bidirectional)
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
