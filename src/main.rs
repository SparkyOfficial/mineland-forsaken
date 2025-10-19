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
    // базова камера і світло
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

// меню
fn setup_menu(mut commands: Commands) {
    // тут буде інтерфейс меню
    println!("setting up menu...");
}

fn cleanup_menu(mut commands: Commands) {
    // очистка меню
    println!("cleaning up menu...");
}

fn menu_system() {
    // логіка меню
    println!("menu system running...");
}

// лоббі
fn setup_lobby(mut commands: Commands) {
    // тут буде інтерфейс лоббі
    println!("setting up lobby...");
}

fn cleanup_lobby(mut commands: Commands) {
    // очистка лоббі
    println!("cleaning up lobby...");
}

fn lobby_system() {
    // логіка лоббі
    println!("lobby system running...");
}

// геймплей
fn setup_game(mut commands: Commands) {
    // тут буде геймплей
    println!("setting up game...");
}

fn cleanup_game(mut commands: Commands) {
    // очистка гри
    println!("cleaning up game...");
}

fn game_system() {
    // логіка гри
    println!("game system running...");
}