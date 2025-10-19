// асиметричний хоррор на беві
// author: Андрій Будильников

use bevy::prelude::*;

mod menu;
mod lobby;
mod game;
mod network;

// стани гри
#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
enum GameState {
    #[default]
    Menu,
    Lobby,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Menu), menu::setup_menu)
        .add_systems(OnExit(GameState::Menu), menu::cleanup_menu)
        .add_systems(OnEnter(GameState::Lobby), lobby::setup_lobby)
        .add_systems(OnExit(GameState::Lobby), lobby::cleanup_lobby)
        .add_systems(OnEnter(GameState::Game), game::setup_game)
        .add_systems(OnExit(GameState::Game), game::cleanup_game)
        .add_systems(Update, menu::menu_system.run_if(in_state(GameState::Menu)))
        .add_systems(Update, lobby::lobby_system.run_if(in_state(GameState::Lobby)))
        .add_systems(Update, game::game_system.run_if(in_state(GameState::Game)))
        .run();
}

fn setup(mut commands: Commands) {
    // Create a single camera that will be used throughout the game
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}