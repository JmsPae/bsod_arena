use std::net::Ipv4Addr;

use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy::window::PrimaryWindow;
use bevy_screen_diagnostics::ScreenDiagnosticsPlugin;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;
use lightyear::client::components::Confirmed;
use lightyear::client::config::ClientConfig;
use lightyear::client::interpolation::plugin::InterpolationSet;
use lightyear::client::prediction::plugin::PredictionSet;
use lightyear::prelude::client::ClientCommands;
use lightyear::prelude::server::ServerCommands;
use lightyear::prelude::*;

use crate::player::PlayerActions;

use self::networking::{NetworkingPlugin, FIXED_UPDATE_HZ, config::remote_client_config};
use self::player::{PlayerBundle, PlayerId, PlayerPlugin};
use self::state::{ClientState, ServerState, State};

mod player;
mod state;
mod common;
//pub mod scripting;
pub mod networking;

pub struct GamePlugin;


#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum FixedSet {
    // main fixed update systems (handle inputs)
    MainClient,
    MainServer,
    MainScript,
    // apply physics steps
    PrePhysics,
    Physics,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {

        app.init_state::<State>()
            .init_state::<ClientState>()
            .init_state::<ServerState>();

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
                    FixedSet::MainClient.run_if(in_state(ClientState::Running)),
                    FixedSet::MainServer.run_if(in_state(ServerState::Running)),
                    FixedSet::MainScript,
                    FixedSet::PrePhysics,
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
    mut commands: Commands,
    mut next_state: ResMut<NextState<State>>,
    mut next_server_state: ResMut<NextState<ServerState>>,
    mut next_client_state: ResMut<NextState<ClientState>>,
    mut client_conf: ResMut<ClientConfig>,

    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyS) {
        info!("Starting host");

        commands.start_server();
        commands.connect_client();

        next_state.set(State::Game);
        next_server_state.set(ServerState::Running);
        next_client_state.set(ClientState::Running);
    }
    else if keys.just_pressed(KeyCode::KeyC) {
        info!("Starting client");
    
        *client_conf = remote_client_config(Ipv4Addr::new(127, 0, 0, 1));
        commands.connect_client();
        
        next_state.set(State::Game);
        next_client_state.set(ClientState::Running);
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
