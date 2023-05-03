use bevy::asset::LoadState;
use bevy::prelude::*;

use crate::assets::GameAssets;
use crate::states::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(start_loading.in_schedule(OnEnter(GameState::Loading)))
            .add_system(check_loading_state.run_if(in_state(GameState::Loading)));
    }
}

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let assets = GameAssets {
        font: asset_server.load("Kenney Future.ttf"),
        logo: asset_server.load("logo.jpg"),
    };

    commands.insert_resource(assets);
}

fn check_loading_state(
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
) {
    match asset_server.get_group_load_state([assets.font.id(), assets.logo.id()]) {
        LoadState::Loading => { /* still loading, maybe add some loading screen */ }
        LoadState::Loaded => {
            next_state.set(GameState::Splash);
            info!("Finished loading assets");
        }
        LoadState::Failed => {
            panic!("Loading one or more assets failed");
        }
        state => { debug!("Got unexpected asset loading state: {:?}", state); }
    }
}
