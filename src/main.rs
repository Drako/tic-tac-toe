use bevy::prelude::*;

use crate::states::{GameState, LoadingPlugin, MenuPlugin, MenuState, SplashPlugin};

pub mod assets;
pub mod states;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_state::<MenuState>()
        .add_plugin(LoadingPlugin)
        .add_plugin(SplashPlugin)
        .add_plugin(MenuPlugin)
        .run();
}
