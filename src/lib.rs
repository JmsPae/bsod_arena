use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;
use bevy_screen_diagnostics::ScreenDiagnosticsPlugin;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::axislike::AxisType;
use leafwing_input_manager::prelude::*;
use lightyear::client::components::Confirmed;
use lightyear::client::interpolation::plugin::InterpolationSet;
use lightyear::client::prediction::plugin::PredictionSet;
use lightyear::prelude::*;

use crate::player::PlayerActions;

use self::networking::{NetworkingPlugin, FIXED_UPDATE_HZ};
use self::player::{PlayerBundle, PlayerId, PlayerPlugin};
use self::state::{NetState, State};

mod player;
mod state;
mod common;
pub mod networking;

pub struct GamePlugin;


#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum FixedSet {
    // main fixed update systems (handle inputs)
    MainClient,
    MainServer,
    // apply physics steps
    Physics,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {


        app.init_state::<State>()
            .init_state::<NetState>();

        app.add_plugins((
            NetworkingPlugin,
            PhysicsPlugins::new(FixedUpdate),
            ScreenDiagnosticsPlugin::default(),
            PlayerPlugin
        ))
            .insert_resource(Time::new_with(Physics::fixed_once_hz(FIXED_UPDATE_HZ)))
            .insert_resource(Gravity(Vec2::ZERO));

        app.configure_sets(
            FixedUpdate,
            (
                // make sure that any physics simulation happens after the Main SystemSet
                // (where we apply user's actions)
                (
                    PhysicsSet::Prepare,
                    PhysicsSet::StepSimulation,
                    PhysicsSet::Sync,
                )
                    .in_set(FixedSet::Physics),
                (
                    FixedSet::MainClient
                        .run_if(in_state(NetState::Client))
                        .run_if(in_state(NetState::ClientServer))
                        .run_if(in_state(State::Game)), 
                    FixedSet::MainServer
                        .run_if(in_state(NetState::Server))
                        .run_if(in_state(NetState::ClientServer))
                        .run_if(in_state(State::Game)), 
                    FixedSet::Physics
                ).chain(),
            ),
        );

        app.add_systems(Startup, init)
            .add_systems(Update, await_mode
                .run_if(in_state(State::NotInGame))
            )
            .add_systems(PostUpdate, draw_players
                .after(InterpolationSet::Interpolate)
                .after(PredictionSet::VisualCorrection)
            )
            .add_systems(PreUpdate, handle_connection
                .after(MainSet::Receive)
                .before(PredictionSet::SpawnPrediction)
            );
    }
}

fn init(mut commands: Commands) {
    commands.spawn(
        Camera2dBundle {
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedHorizontal(360.0),
                ..Default::default()
            },
            ..Default::default()
        }
    );
}

fn await_mode(
    mut next_state: ResMut<NextState<State>>,
    mut next_net_state: ResMut<NextState<NetState>>,

    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyS) {
        info!("Starting host");

        next_state.set(State::Game);
        next_net_state.set(NetState::Server);
    }
    else if keys.just_pressed(KeyCode::KeyC) {
        info!("Starting client");
    
        next_state.set(State::Game);
        next_net_state.set(NetState::Client);
    }
}


fn handle_connection(
    mut commands: Commands,
    mut connection_event: EventReader<client::ConnectEvent>,
    window: Query<Entity, With<PrimaryWindow>>
) {
    for event in connection_event.read() {
        info!("Spawning {:?}", event.client_id());

        let map = InputMap::default().insert_multiple([
            (PlayerActions::Up,     KeyCode::KeyW),
            (PlayerActions::Down,   KeyCode::KeyS),
            (PlayerActions::Left,   KeyCode::KeyA),
            (PlayerActions::Right,  KeyCode::KeyD),
        ]).to_owned();

        let player = commands.spawn(
            PlayerBundle::new(
                event.client_id(),
                InputManagerBundle::<PlayerActions>::with_map(map)
        )).id();

        commands.entity(window.single()).insert(ActionStateDriver {
            action: PlayerActions::Look,
            targets: player.into()
        });
    }
}

fn draw_players(
    mut gizmos: Gizmos,
    player_query: Query<(&Position, &Rotation), (With<PlayerId>, Without<Confirmed>)>
) {
    for (position, rotation) in player_query.iter() {
        gizmos.rect_2d(position.xy(), rotation.as_radians(), Vec2::ONE * 16.0, Color::WHITE);
    }
}
