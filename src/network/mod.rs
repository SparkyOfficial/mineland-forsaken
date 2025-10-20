// мережа
// author: Андрій Будильников

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::collections::HashMap;

pub mod test_network;

// компонент для мережевого гравця
#[derive(Component)]
pub struct NetworkPlayer {
    pub player_name: String,
}

// структура для мережевого повідомлення
#[derive(Event, Serialize, Deserialize, Debug, Clone)]
pub struct NetworkMessage {
    pub sender: String,
    pub content: String,
}

// ресурс для стану мережі
#[derive(Resource)]
pub struct NetworkState {
    pub is_host: bool,
    pub connected: bool,
    pub player_name: String,
    pub server_address: Option<SocketAddr>,
}

impl Default for NetworkState {
    fn default() -> Self {
        Self {
            is_host: false,
            connected: false,
            player_name: "Player".to_string(),
            server_address: None,
        }
    }
}

// ресурс для стану лобі
#[derive(Resource, Default)]
pub struct LobbyState {
    pub lobbies: Vec<LobbyInfo>,
    pub players: HashMap<String, bool>, // name -> is_ready
    pub current_lobby_id: Option<u32>,
}

// повідомлення для створення лобі
#[derive(Serialize, Deserialize, Debug, Clone, Event)]
pub enum LobbyMessage {
    CreateLobby { player_name: String },
    JoinLobby { player_name: String, lobby_id: u32 },
    LeaveLobby { player_name: String },
    ListLobbies,
    LobbyList { lobbies: Vec<LobbyInfo> },
    PlayerJoined { player_name: String },
    PlayerLeft { player_name: String },
    StartGame,
}

// інформація про лобі
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LobbyInfo {
    pub id: u32,
    pub name: String,
    pub player_count: u32,
    pub max_players: u32,
}

pub fn setup_network(mut commands: Commands) {
    // ініціалізація мережі
    commands.insert_resource(NetworkState::default());
    commands.insert_resource(LobbyState::default());
    println!("network system initialized");
}

pub fn network_system(
    _network_state: ResMut<NetworkState>,
    _lobby_state: ResMut<LobbyState>,
) {
    // логіка мережі
    println!("network system running");
}