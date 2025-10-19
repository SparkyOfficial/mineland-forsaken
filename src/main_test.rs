// тестова програма для перевірки мережевої функціональності
// author: Андрій Будильников

use mineland_forsaken::network::test_network::{TestMessage, TestLobbyInfo};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};

fn main() {
    // Запускаємо сервер у окремому потоці
    let server_handle = thread::spawn(|| {
        if let Err(e) = run_server() {
            eprintln!("Server error: {}", e);
        }
    });
    
    // Зачекаємо трохи, щоб сервер запустився
    thread::sleep(Duration::from_secs(1));
    
    // Запускаємо клієнта
    if let Err(e) = run_client() {
        eprintln!("Client error: {}", e);
    }
    
    // Чекаємо завершення сервера
    server_handle.join().unwrap();
}

fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server running on 127.0.0.1:8080");
    
    // Створюємо тестові лобі
    let mut lobbies: Vec<TestLobbyInfo> = Vec::new();
    lobbies.push(TestLobbyInfo {
        id: 1,
        name: "Test Lobby 1".to_string(),
        player_count: 2,
    });
    lobbies.push(TestLobbyInfo {
        id: 2,
        name: "Test Lobby 2".to_string(),
        player_count: 1,
    });
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New client connected");
                let lobbies_clone = lobbies.clone();
                thread::spawn(move || {
                    handle_client(&mut stream, &lobbies_clone);
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
    
    Ok(())
}

fn handle_client(stream: &mut TcpStream, lobbies: &Vec<TestLobbyInfo>) {
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
                            TestMessage::Ping => {
                                println!("Received Ping, sending Pong");
                                TestMessage::Pong
                            }
                            TestMessage::ListLobbies => {
                                println!("Received ListLobbies request");
                                TestMessage::LobbyList { lobbies: lobbies.clone() }
                            }
                            TestMessage::CreateLobby { name } => {
                                println!("Received CreateLobby request: {}", name);
                                TestMessage::ListLobbies // Для спрощення просто повертаємо список
                            }
                            TestMessage::JoinLobby { lobby_id } => {
                                println!("Received JoinLobby request for lobby {}", lobby_id);
                                TestMessage::ListLobbies // Для спрощення просто повертаємо список
                            }
                            _ => TestMessage::Pong,
                        };
                        
                        let response_data = bincode::serialize(&response).unwrap();
                        if let Err(e) = stream.write_all(&response_data) {
                            println!("Error sending response: {}", e);
                            break;
                        }
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

fn run_client() -> std::io::Result<()> {
    println!("Connecting to server...");
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to server");
    
    // Відправляємо Ping
    let ping_message = TestMessage::Ping;
    let ping_data = bincode::serialize(&ping_message).unwrap();
    stream.write_all(&ping_data)?;
    
    // Читаємо відповідь
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    let response: TestMessage = bincode::deserialize(&buffer[..n]).unwrap();
    
    match response {
        TestMessage::Pong => println!("Received Pong from server"),
        _ => println!("Received unexpected response: {:?}", response),
    }
    
    // Відправляємо запит на список лобі
    let list_message = TestMessage::ListLobbies;
    let list_data = bincode::serialize(&list_message).unwrap();
    stream.write_all(&list_data)?;
    
    // Читаємо відповідь
    let n = stream.read(&mut buffer)?;
    let response: TestMessage = bincode::deserialize(&buffer[..n]).unwrap();
    
    match response {
        TestMessage::LobbyList { lobbies } => {
            println!("Received lobby list with {} lobbies:", lobbies.len());
            for lobby in lobbies {
                println!("  Lobby {}: {} ({} players)", lobby.id, lobby.name, lobby.player_count);
            }
        }
        _ => println!("Received unexpected response: {:?}", response),
    }
    
    // Відправляємо запит на створення лобі
    let create_message = TestMessage::CreateLobby { name: "My Lobby".to_string() };
    let create_data = bincode::serialize(&create_message).unwrap();
    stream.write_all(&create_data)?;
    
    // Читаємо відповідь
    let n = stream.read(&mut buffer)?;
    let response: TestMessage = bincode::deserialize(&buffer[..n]).unwrap();
    
    match response {
        TestMessage::ListLobbies => println!("Lobby created successfully"),
        _ => println!("Received unexpected response: {:?}", response),
    }
    
    println!("Client test completed");
    Ok(())
}