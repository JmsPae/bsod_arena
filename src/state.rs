use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum State {
    #[default]
    NotInGame,
    Game
}


#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum NetState {
    #[default]
    None,
    Server,
    Client,
    ClientServer
}
