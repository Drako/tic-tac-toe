use bevy::prelude::*;
use bevy::render::view::RenderLayers;

use crate::assets::GameAssets;
use crate::states::menu::MainMenu;

pub fn spawn_menu(
    commands: &mut Commands,
    render_layers: RenderLayers,
    assets: &GameAssets,
    is_main_menu: bool,
) {
    commands.spawn((NodeBundle {
        background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.8)),
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                left: Val::Px(0.0),
                top: Val::Px(0.0),
                ..default()
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        ..default()
    }, render_layers, MainMenu)).with_children(|parent| {
        parent.spawn((NodeBundle {
            background_color: BackgroundColor(Color::WHITE),
            style: Style {
                size: Size::new(Val::Px(400.0), Val::Px(400.0)),
                padding: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        }, render_layers)).with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_section("Tic-Tac-Toe", TextStyle {
                    font: assets.font.clone(),
                    font_size: 32.0,
                    color: Color::BLACK,
                }),
                ..default()
            }, render_layers));
        });
    });
}
