use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::window::PrimaryWindow;

use crate::assets::GameAssets;
use crate::states::{GameState, MenuState};
use crate::states::menu::ui::spawn_menu;

mod ui;

#[derive(Component)]
pub struct MainMenu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(enter_main_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(leave_main_menu.in_schedule(OnExit(GameState::MainMenu)))
            .add_system(setup_menu.in_schedule(OnEnter(MenuState::Visible)))
            .add_system(teardown_menu.in_schedule(OnExit(MenuState::Visible)))
            .add_system(resize_menu.in_set(OnUpdate(MenuState::Visible)));
    }
}

fn enter_main_menu(mut next_state: ResMut<NextState<MenuState>>) {
    next_state.set(MenuState::Visible);
}

fn leave_main_menu(mut next_state: ResMut<NextState<MenuState>>) {
    next_state.set(MenuState::Hidden);
}

fn setup_menu(mut commands: Commands, game_state: Res<State<GameState>>, assets: Res<GameAssets>) {
    info!("Showing menu");

    // this way we can share code between main menu and pause menu
    let is_main_menu = game_state.0 == GameState::MainMenu;

    let render_layer = RenderLayers::layer(1);

    commands.spawn((Camera2dBundle {
        camera: Camera {
            order: 1,
            ..default()
        },
        ..default()
    }, MainMenu, render_layer));

    spawn_menu(&mut commands, render_layer, assets.as_ref(), is_main_menu);
}

fn teardown_menu(mut commands: Commands, entities_q: Query<Entity, With<MainMenu>>) {
    for entity in &entities_q {
        commands.entity(entity).despawn_recursive();
    }

    info!("Closed menu");
}

fn resize_menu(
    mut menu_q: Query<&mut Style, With<MainMenu>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
) {
    let mut style = menu_q.single_mut();
    let window = window_q.single();
    style.size.width = Val::Px(window.resolution.width());
    style.size.height = Val::Px(window.resolution.height());
}
