// Демонстрація мережевої функціональності
// author: Андрій Будильников

use serde::{Deserialize, Serialize};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::io::{Read, Write, BufReader, BufWriter};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// Повідомлення для мережевої комунікації
#[derive(Serialize, Deserialize, Debug, Clone)]
enum NetworkMessage {
    // Базові повідомлення
    Ping,
    Pong,
    
    // Повідомлення для роботи з лобі
    CreateLobby { player_name: String, lobby_name: String },
    JoinLobby { player_name: String, lobby_id: u32 },
    LeaveLobby { player_name: String },
    ListLobbies,
    LobbyList { lobbies: Vec<LobbyInfo> },
    PlayerJoined { player_name: String, lobby_id: u32 },
    PlayerLeft { player_name: String, lobby_id: u32 },
    
    // Повідомлення для гри
    StartGame,
    GameData { data: Vec<u8> },
}

// Інформація про лобі
#[derive(Serialize, Deserialize, Debug, Clone)]
struct LobbyInfo {
    id: u32,
    name: String,
    player_count: u32,
    max_players: u32,
    players: Vec<String>,
}

// Стан сервера
struct ServerState {
    lobbies: HashMap<u32, Lobby>,
    next_lobby_id: u32,
    clients: Vec<TcpStream>,
}

// Лобі на сервері
struct Lobby {
    id: u32,
    name: String,
    players: Vec<String>,
    max_players: u32,
}

impl Lobby {
    fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            players: Vec::new(),
            max_players: 8,
        }
    }
    
    fn add_player(&mut self, player_name: String) -> bool {
        if self.players.len() < self.max_players as usize {
            self.players.push(player_name);
            true
        } else {
            false
        }
    }
    
    fn remove_player(&mut self, player_name: &str) {
        self.players.retain(|name| name != player_name);
    }
    
    fn player_count(&self) -> u32 {
        self.players.len() as u32
    }
    
    fn info(&self) -> LobbyInfo {
        LobbyInfo {
            id: self.id,
            name: self.name.clone(),
            player_count: self.player_count(),
            max_players: self.max_players,
            players: self.players.clone(),
        }
    }
}

impl ServerState {
    fn new() -> Self {
        Self {
            lobbies: HashMap::new(),
            next_lobby_id: 1,
            clients: Vec::new(),
        }
    }
    
    fn create_lobby(&mut self, player_name: String, lobby_name: String) -> u32 {
        let id = self.next_lobby_id;
        self.next_lobby_id += 1;
        
        let mut lobby = Lobby::new(id, lobby_name);
        lobby.add_player(player_name);
        
        self.lobbies.insert(id, lobby);
        println!("Created lobby {} with ID: {}", lobby_name, id);
        id
    }
    
    fn join_lobby(&mut self, player_name: String, lobby_id: u32) -> bool {
        if let Some(lobby) = self.lobbies.get_mut(&lobby_id) {
            let result = lobby.add_player(player_name.clone());
            if result {
                println!("Player {} joined lobby {}", player_name, lobby_id);
            } else {
                println!("Player {} failed to join lobby {} (full)", player_name, lobby_id);
            }
            result
        } else {
            println!("Player {} tried to join non-existent lobby {}", player_name, lobby_id);
            false
        }
    }
    
    fn leave_lobby(&mut self, player_name: String) {
        for (_, lobby) in self.lobbies.iter_mut() {
            if lobby.players.contains(&player_name) {
                lobby.remove_player(&player_name);
                println!("Player {} left lobby {}", player_name, lobby.id);
                
                // Якщо лобі порожнє, видаляємо його
                if lobby.player_count() == 0 {
                    let lobby_id = lobby.id;
                    println!("Lobby {} is empty, removing", lobby_id);
                }
                break;
            }
        }
    }
    
    fn list_lobbies(&self) -> Vec<LobbyInfo> {
        self.lobbies.values().map(|lobby| lobby.info()).collect()
    }
}

// Функція для запуску сервера
fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server started on 127.0.0.1:8080");
    
    let mut server_state = ServerState::new();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected");
                let mut server_state_clone = server_state.clone();
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream, &mut server_state_clone) {
                        eprintln!("Client error: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }
    
    Ok(())
}

// Функція для обробки клієнта
fn handle_client(mut stream: TcpStream, server_state: &mut ServerState) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream);
    
    let mut buffer = vec![0u8; 1024];
    
    loop {
        // Читаємо повідомлення від клієнта
        let bytes_read = match reader.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(n) => n,
            Err(e) => {
                eprintln!("Read error: {}", e);
                break;
            }
        };
        
        // Десеріалізуємо повідомлення
        let message: NetworkMessage = match bincode::deserialize(&buffer[..bytes_read]) {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("Deserialize error: {}", e);
                continue;
            }
        };
        
        // Обробляємо повідомлення
        let response = match message {
            NetworkMessage::Ping => {
                println!("Received Ping");
                NetworkMessage::Pong
            }
            NetworkMessage::CreateLobby { player_name, lobby_name } => {
                let lobby_id = server_state.create_lobby(player_name, lobby_name);
                // Повертаємо список лобі для оновлення
                NetworkMessage::LobbyList { lobbies: server_state.list_lobbies() }
            }
            NetworkMessage::JoinLobby { player_name, lobby_id } => {
                let success = server_state.join_lobby(player_name.clone(), lobby_id);
                if success {
                    // Повертаємо оновлений список лобі
                    NetworkMessage::LobbyList { lobbies: server_state.list_lobbies() }
                } else {
                    // Повертаємо той самий список
                    NetworkMessage::LobbyList { lobbies: server_state.list_lobbies() }
                }
            }
            NetworkMessage::LeaveLobby { player_name } => {
                server_state.leave_lobby(player_name);
                NetworkMessage::LobbyList { lobbies: server_state.list_lobbies() }
            }
            NetworkMessage::ListLobbies => {
                NetworkMessage::LobbyList { lobbies: server_state.list_lobbies() }
            }
            _ => NetworkMessage::Pong,
        };
        
        // Серіалізуємо та відправляємо відповідь
        let response_data = bincode::serialize(&response)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        
        writer.write_all(&response_data)?;
        writer.flush()?;
    }
    
    Ok(())
}

// Функція для запуску клієнта
fn run_client(client_name: &str) -> std::io::Result<()> {
    println!("Client {} connecting to server...", client_name);
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Client {} connected", client_name);
    
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream);
    
    // Відправляємо Ping для тестування
    let ping_message = NetworkMessage::Ping;
    let ping_data = bincode::serialize(&ping_message)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    writer.write_all(&ping_data)?;
    writer.flush()?;
    
    // Читаємо відповідь
    let mut buffer = vec![0u8; 1024];
    let bytes_read = reader.read(&mut buffer)?;
    let response: NetworkMessage = bincode::deserialize(&buffer[..bytes_read])
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    match response {
        NetworkMessage::Pong => println!("Client {} received Pong", client_name),
        _ => println!("Client {} received unexpected response: {:?}", client_name, response),
    }
    
    // Створюємо лобі
    let create_message = NetworkMessage::CreateLobby {
        player_name: client_name.to_string(),
        lobby_name: format!("{}'s Lobby", client_name),
    };
    let create_data = bincode::serialize(&create_message)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    writer.write_all(&create_data)?;
    writer.flush()?;
    
    // Читаємо відповідь
    let bytes_read = reader.read(&mut buffer)?;
    let response: NetworkMessage = bincode::deserialize(&buffer[..bytes_read])
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    match response {
        NetworkMessage::LobbyList { lobbies } => {
            println!("Client {} received lobby list:", client_name);
            for lobby in lobbies {
                println!("  Lobby {}: {} ({} players)", lobby.id, lobby.name, lobby.player_count);
            }
        }
        _ => println!("Client {} received unexpected response: {:?}", client_name, response),
    }
    
    println!("Client {} test completed", client_name);
    Ok(())
}

fn main() {
    println!("Network Demo - Mineland Forsaken");
    println!("================================");
    
    // Запускаємо сервер у окремому потоці
    let server_handle = thread::spawn(|| {
        if let Err(e) = run_server() {
            eprintln!("Server error: {}", e);
        }
    });
    
    // Зачекаємо трохи, щоб сервер запустився
    thread::sleep(Duration::from_secs(1));
    
    // Запускаємо кілька клієнтів
    let client1_handle = thread::spawn(|| {
        if let Err(e) = run_client("Player1") {
            eprintln!("Client1 error: {}", e);
        }
    });
    
    let client2_handle = thread::spawn(|| {
        // Зачекаємо трохи перед запуском другого клієнта
        thread::sleep(Duration::from_secs(1));
        if let Err(e) = run_client("Player2") {
            eprintln!("Client2 error: {}", e);
        }
    });
    
    // Чекаємо завершення клієнтів
    client1_handle.join().unwrap();
    client2_handle.join().unwrap();
    
    // Зупиняємо сервер (в реальній програмі сервер би працював постійно)
    println!("Demo completed");
}