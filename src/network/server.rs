// серверна частина мережі
// author: Андрій Будильников

use bevy::prelude::*;
use std::collections::HashMap;
use super::{LobbyMessage, LobbyInfo};

// ресурс для серверного стану
#[derive(Resource)]
pub struct ServerState {
    pub lobbies: HashMap<u32, Lobby>,
    pub next_lobby_id: u32,
}

impl Default for ServerState {
    fn default() -> Self {
        Self {
            lobbies: HashMap::new(),
            next_lobby_id: 1,
        }
    }
}

// лобі на сервері
#[derive(Debug)]
pub struct Lobby {
    pub id: u32,
    pub name: String,
    pub players: Vec<String>,
    pub max_players: u32,
}

impl Lobby {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            players: Vec::new(),
            max_players: 8,
        }
    }
    
    pub fn player_count(&self) -> u32 {
        self.players.len() as u32
    }
    
    pub fn add_player(&mut self, player_name: String) {
        self.players.push(player_name);
    }
    
    pub fn remove_player(&mut self, player_name: &str) {
        self.players.retain(|name| name != player_name);
    }
    
    pub fn info(&self) -> LobbyInfo {
        LobbyInfo {
            id: self.id,
            name: self.name.clone(),
            player_count: self.player_count(),
            max_players: self.max_players,
        }
    }
}

pub fn server_system(
    mut server_state: ResMut<ServerState>,
) {
    // проста логіка сервера
    println!("server running with {} lobbies", server_state.lobbies.len());
}

fn create_lobby(server_state: &mut ServerState, player_name: String) {
    let lobby_id = server_state.next_lobby_id;
    server_state.next_lobby_id += 1;
    
    let lobby_name = format!("Lobby #{}", lobby_id);
    let mut lobby = Lobby::new(lobby_id, lobby_name.clone());
    
    // Додаємо гравця до лобі
    lobby.add_player(player_name);
    
    server_state.lobbies.insert(lobby_id, lobby);
    println!("created lobby {} with player", lobby_name);
}

fn join_lobby(server_state: &mut ServerState, player_name: String, lobby_id: u32) {
    if let Some(lobby) = server_state.lobbies.get_mut(&lobby_id) {
        // Додаємо гравця до лобі
        lobby.add_player(player_name);
        println!("player joined lobby {}", lobby_id);
    }
}

fn leave_lobby(server_state: &mut ServerState, player_name: &str) {
    // знаходимо лобі гравця
    for (_, lobby) in server_state.lobbies.iter_mut() {
        if lobby.players.contains(&player_name.to_string()) {
            lobby.remove_player(player_name);
            // якщо лобі порожнє, видаляємо його
            if lobby.player_count() == 0 {
                let lobby_id = lobby.id;
                server_state.lobbies.remove(&lobby_id);
                println!("removed empty lobby {}", lobby_id);
            }
            break;
        }
    }
    println!("player {} left lobby", player_name);
}

fn send_lobby_list(server_state: &ServerState) {
    let lobbies: Vec<LobbyInfo> = server_state.lobbies
        .values()
        .map(|lobby| lobby.info())
        .collect();
    
    println!("sending lobby list: {} lobbies", lobbies.len());
}