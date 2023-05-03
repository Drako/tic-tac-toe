use bevy::prelude::*;

use crate::states::{GameState, LoadingPlugin, SplashPlugin};

pub mod assets;
pub mod states;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_plugin(LoadingPlugin)
        .add_plugin(SplashPlugin)
        .run();
}
