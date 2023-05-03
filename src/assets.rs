use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub font: Handle<Font>,
    pub logo: Handle<Image>,
}
