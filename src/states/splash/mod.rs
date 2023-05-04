use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::window::PrimaryWindow;

use crate::assets::GameAssets;
use crate::states::GameState;

#[derive(Component)]
struct SplashScreen;

#[derive(Component)]
struct Logo;

#[derive(Component)]
struct LogoFader {
    time: Stopwatch,
}

// Logo should take 10 seconds
const LOGO_DURATION: f32 = 10.0;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup_splash_screen.in_schedule(OnEnter(GameState::Splash)))
            .add_system(teardown_splash_screen.in_schedule(OnExit(GameState::Splash)))
            .add_system(resize_splash_screen.run_if(in_state(GameState::Splash)))
            .add_system(fade_splash_screen.run_if(in_state(GameState::Splash)))
            .add_system(skip_if_requested.run_if(in_state(GameState::Splash)));
    }
}

fn setup_splash_screen(mut commands: Commands, assets: Res<GameAssets>) {
    info!("Starting splash screen");

    commands.spawn((Camera2dBundle::default(), SplashScreen));

    commands.spawn((SpriteBundle {
        texture: assets.logo.clone(),
        ..default()
    }, Logo, SplashScreen));

    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.0),
            ..default()
        },
        ..default()
    }, LogoFader { time: Stopwatch::new() }, SplashScreen));
}

fn teardown_splash_screen(mut commands: Commands, entities_q: Query<Entity, With<SplashScreen>>) {
    for entity in &entities_q {
        commands.entity(entity).despawn();
    }

    info!("Finished splash screen");
}

fn resize_splash_screen(
    window_q: Query<&Window, With<PrimaryWindow>>,
    assets: Res<GameAssets>,
    images: Res<Assets<Image>>,
    mut fader_q: Query<&mut Sprite, With<LogoFader>>,
    mut logo_q: Query<&mut Transform, With<Logo>>,
) {
    let window = window_q.single();
    let (win_w, win_h) = (window.resolution.width(), window.resolution.height());
    let win_aspect = win_w / win_h;

    let mut fader = fader_q.single_mut();
    fader.custom_size = Some(Vec2::new(win_w, win_h));

    let mut logo_transform = logo_q.single_mut();
    let logo_image = images.get(&assets.logo).unwrap();
    let logo_size = logo_image.size();
    let (logo_w, logo_h) = (logo_size.x, logo_size.y);
    let logo_aspect = logo_w / logo_h;

    let scale = if win_aspect > logo_aspect {
        win_h / logo_h
    } else {
        win_w / logo_w
    };

    logo_transform.scale.x = scale;
    logo_transform.scale.y = scale;
}

fn fade_splash_screen(
    mut fader_q: Query<(&mut Sprite, &mut LogoFader)>,
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    let (mut sprite, mut fader) = fader_q.single_mut();
    let timer = &mut fader.time;
    timer.tick(time.delta());

    let progress = timer.elapsed_secs();

    if progress > LOGO_DURATION {
        sprite.color = Color::rgba(0.0, 0.0, 0.0, 1.0);
        next_state.set(GameState::MainMenu);
        return;
    }

    if progress < LOGO_DURATION * 0.3 {
        sprite.color = Color::rgba(0.0, 0.0, 0.0, (progress * std::f32::consts::PI / (LOGO_DURATION * 0.6)).cos());
    } else if progress > LOGO_DURATION * 0.7 {
        sprite.color = Color::rgba(0.0, 0.0, 0.0, ((LOGO_DURATION - progress) * std::f32::consts::PI / (LOGO_DURATION * 0.6)).cos());
    } else {
        sprite.color = Color::rgba(0.0, 0.0, 0.0, 0.0);
    }
}

fn skip_if_requested(
    mut keys: ResMut<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.any_just_pressed([KeyCode::Space, KeyCode::Escape, KeyCode::Return]) {
        keys.clear();
        next_state.set(GameState::MainMenu);
        info!("Skipping splash screen on behalf of user");
    }
}
