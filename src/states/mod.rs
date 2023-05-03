use bevy::prelude::*;

pub use loading::LoadingPlugin;
pub use splash::SplashPlugin;

mod loading;
mod splash;

#[derive(States, Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Splash,
    Menu,
    Game,
}
