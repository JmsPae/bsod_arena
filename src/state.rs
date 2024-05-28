use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
    #[default]
    NotInGame,
    Game
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ServerState {
    #[default]
    None,
    Running
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClientState {
    #[default]
    None,
    Running
}
