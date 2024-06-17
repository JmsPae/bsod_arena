use std::time::{Duration, Instant};

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_xpbd_3d::prelude::*;
use iyes_perf_ui::PerfUiCompleteBundle;

mod debug;
mod cursor;

mod character_controller;
mod player;

use self::character_controller::CharacterControllerPlugin;
use self::cursor::GameCursorPlugin;
use self::debug::DebugPlugin;
use self::player::PlayerPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum FixedSet {
    // main fixed update systems (handle inputs)
    Main,
    Physics,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((DebugPlugin, PhysicsPlugins::new(FixedUpdate), GameCursorPlugin))
            .add_systems(Startup, setup);


        app.configure_sets(FixedUpdate, 
            (
                (
                    PhysicsSet::Prepare,
                    PhysicsSet::StepSimulation,
                    PhysicsSet::Sync,
                )
                    .in_set(FixedSet::Physics),
                (
                    FixedSet::Main,
                    FixedSet::Physics
                ).chain()
            )
        );

        app.insert_resource(FrameLimit::new(60.0));
        app.add_systems(Last, limit_framerate.run_if(resource_exists::<FrameLimit>));

        app.add_plugins((PlayerPlugin, CharacterControllerPlugin));
    }
}

#[derive(Resource)]
struct FrameLimit {
    cap: f64,
    inst: Instant
}

impl FrameLimit {
    fn new(cap: f64) -> Self {
        Self { cap, inst: Instant::now() }
    }
}

fn limit_framerate(mut frame_limit: ResMut<FrameLimit>) { // Limit to 60fps
    spin_sleep::sleep(
        Duration::from_secs_f64(
            (
                (1.0 / frame_limit.cap) - 
                frame_limit.inst.elapsed().as_secs_f64()
            ).max(0.0)
        )
    );

    frame_limit.inst = Instant::now();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    let material = materials.add(StandardMaterial::from(Color::GRAY));
    commands.spawn(PerfUiCompleteBundle::default());

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::default()
                        .looking_to(Vec3::new(-0.5, -1.0, 0.5).normalize(), Vec3::Y),
        ..Default::default()
    });

    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Plane3d::new(Vec3::Y)),
            material,
            transform: Transform::from_scale(Vec3::ONE * 10.0),
            ..Default::default()
        }
    );

    commands.spawn(
        Camera3dBundle {
            projection: Projection::Orthographic(OrthographicProjection {
                near: -1000.0,
                far: 1000.0,
                scaling_mode: ScalingMode::FixedVertical(16.0),

                ..Default::default()
            }),
            camera: Camera {
                hdr: true,
                ..Default::default()
            },
            transform: Transform::default().looking_to(Vec3::NEG_Y, Vec3::NEG_Z),
            ..Default::default()
        }
    );
}
