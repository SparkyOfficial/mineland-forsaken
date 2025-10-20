// тестова мережева функціональність
// author: Андрій Будильников

use serde::{Deserialize, Serialize};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::collections::HashMap;

// повідомлення для тестування
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum TestMessage {
    Ping,
    Pong,
    CreateLobby { name: String },
    JoinLobby { lobby_id: u32 },
    ListLobbies,
    LobbyList { lobbies: Vec<TestLobbyInfo> },
}

// інформація про тестове лобі
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestLobbyInfo {
    pub id: u32,
    pub name: String,
    pub player_count: u32,
}

// тестовий сервер
#[derive(Clone)]
pub struct TestServer {
    lobbies: HashMap<u32, TestLobby>,
    next_lobby_id: u32,
}

// тестове лобі
#[derive(Clone)]
pub struct TestLobby {
    id: u32,
    name: String,
    players: Vec<String>,
}

impl TestLobby {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            players: Vec::new(),
        }
    }
    
    pub fn add_player(&mut self, player_name: String) {
        self.players.push(player_name);
    }
    
    pub fn player_count(&self) -> u32 {
        self.players.len() as u32
    }
    
    pub fn info(&self) -> TestLobbyInfo {
        TestLobbyInfo {
            id: self.id,
            name: self.name.clone(),
            player_count: self.player_count(),
        }
    }
}

impl TestServer {
    pub fn new() -> Self {
        Self {
            lobbies: HashMap::new(),
            next_lobby_id: 1,
        }
    }
    
    pub fn create_lobby(&mut self, name: String) -> u32 {
        let id = self.next_lobby_id;
        self.next_lobby_id += 1;
        
        let lobby = TestLobby::new(id, name);
        self.lobbies.insert(id, lobby);
        
        id
    }
    
    pub fn join_lobby(&mut self, lobby_id: u32, player_name: String) -> bool {
        if let Some(lobby) = self.lobbies.get_mut(&lobby_id) {
            lobby.add_player(player_name);
            true
        } else {
            false
        }
    }
    
    pub fn list_lobbies(&self) -> Vec<TestLobbyInfo> {
        self.lobbies.values().map(|lobby| lobby.info()).collect()
    }
}

// функція для тестування сервера
pub fn run_test_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server running on 127.0.0.1:8080");
    
    let mut server = TestServer::new();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection");
                let server_clone = server.clone();
                thread::spawn(move || {
                    handle_client(stream, server_clone);
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    
    Ok(())
}

// обробка клієнта
fn handle_client(mut stream: TcpStream, mut server: TestServer) {
    let mut buffer = [0; 1024];
    
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                let data = &buffer[..n];
                match bincode::deserialize::<TestMessage>(data) {
                    Ok(message) => {
                        let response = match message {
                            TestMessage::Ping => TestMessage::Pong,
                            TestMessage::CreateLobby { name } => {
                                let id = server.create_lobby(name);
                                println!("Created lobby with ID: {}", id);
                                TestMessage::ListLobbies // Повертаємо список для оновлення
                            }
                            TestMessage::JoinLobby { lobby_id } => {
                                // Для тесту додаємо гравця з ім'ям "Player"
                                server.join_lobby(lobby_id, "Player".to_string());
                                println!("Player joined lobby {}", lobby_id);
                                TestMessage::ListLobbies // Повертаємо список для оновлення
                            }
                            TestMessage::ListLobbies => {
                                let lobbies = server.list_lobbies();
                                TestMessage::LobbyList { lobbies }
                            }
                            _ => TestMessage::Pong,
                        };
                        
                        let response_data = bincode::serialize(&response).unwrap();
                        stream.write_all(&response_data).unwrap();
                    }
                    Err(e) => {
                        println!("Deserialize error: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Read error: {}", e);
                break;
            }
        }
    }
}

// функція для тестування клієнта
pub fn run_test_client() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to server");
    
    // Відправляємо повідомлення Ping
    let message = TestMessage::Ping;
    let data = bincode::serialize(&message).unwrap();
    stream.write_all(&data)?;
    
    // Читаємо відповідь
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    let response: TestMessage = bincode::deserialize(&buffer[..n]).unwrap();
    
    println!("Received: {:?}", response);
    
    Ok(())
}