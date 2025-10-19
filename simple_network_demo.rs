// Проста демонстрація мережевої функціональності
// author: Андрій Будильников

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader, BufWriter};
use std::thread;
use std::time::Duration;

// Функція для запуску сервера
fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server started on 127.0.0.1:8080");
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected");
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
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
fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream);
    
    let mut buffer = [0u8; 1024];
    
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
        
        // Конвертуємо байти в строку
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Received message: {}", message);
        
        // Відправляємо відповідь
        let response = match message.as_ref() {
            "PING" => "PONG",
            "HELLO" => "WELCOME",
            "CREATE_LOBBY" => "LOBBY_CREATED",
            "JOIN_LOBBY" => "JOINED_LOBBY",
            "LIST_LOBBIES" => "LOBBY1:2/8,LOBBY2:1/8",
            _ => "UNKNOWN_COMMAND",
        };
        
        writer.write_all(response.as_bytes())?;
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
    
    // Відправляємо тестові повідомлення
    let test_messages = vec!["PING", "HELLO", "CREATE_LOBBY", "JOIN_LOBBY", "LIST_LOBBIES"];
    
    for message in test_messages {
        println!("Client {} sending: {}", client_name, message);
        
        // Відправляємо повідомлення
        writer.write_all(message.as_bytes())?;
        writer.flush()?;
        
        // Читаємо відповідь
        let mut buffer = [0u8; 1024];
        let bytes_read = reader.read(&mut buffer)?;
        let response = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Client {} received: {}", client_name, response);
        
        // Зачекаємо трохи між повідомленнями
        thread::sleep(Duration::from_millis(500));
    }
    
    println!("Client {} test completed", client_name);
    Ok(())
}

fn main() {
    println!("Simple Network Demo - Mineland Forsaken");
    println!("======================================");
    
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