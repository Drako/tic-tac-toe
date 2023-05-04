use bevy::prelude::*;

pub use loading::LoadingPlugin;
pub use menu::MenuPlugin;
pub use splash::SplashPlugin;

mod loading;
mod splash;
mod menu;

#[derive(States, Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Splash,
    MainMenu,
    Game,
    Credits,
}

#[derive(States, Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MenuState {
    #[default]
    Hidden,
    Visible,
}
