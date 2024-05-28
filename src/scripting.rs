use std::sync::Mutex;

use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;

use crate::FixedSet;

pub struct GameScriptingPlugin;


#[derive(Clone)]
pub struct EntityLuaArg(pub Entity);

impl<'lua> IntoLua<'lua> for EntityLuaArg {
    fn into_lua(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        self.0.to_lua_proxy(&lua)
    }
}

impl Plugin for GameScriptingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_mod_scripting::core::ScriptingPlugin);

        app.add_script_host_to_set::<LuaScriptHost<EntityLuaArg>>(FixedUpdate, FixedSet::MainScript)
            .add_script_handler_to_set::<LuaScriptHost<EntityLuaArg>, 0, 0>(FixedUpdate, FixedSet::MainScript)
            .add_api_provider::<LuaScriptHost<EntityLuaArg>>(Box::new(LuaCoreBevyAPIProvider))
            .add_systems(Startup, load_scripts);
        
    }
}

fn load_scripts(
    server: Res<AssetServer>, 
    mut commands: Commands,
    mut lua_writer: PriorityEventWriter<LuaEvent<EntityLuaArg>>
) {
    let path = "scripts/test.lua";
    let handle = server.load::<LuaFile>(path);

    commands.spawn(()).insert(ScriptCollection::<LuaFile> {
        scripts: vec![Script::<LuaFile>::new(path.to_string(), handle)],
    });
}
