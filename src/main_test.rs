// тестова програма для перевірки мережевої функціональності
// author: Андрій Будильников

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

// Простий тест без залежностей від основного проекту
fn main() {
    println!("Network test - simple demo");
    
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
    let _ = server_handle.join();
}

fn run_server() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Server running on 127.0.0.1:8080");
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("New client connected");
                thread::spawn(move || {
                    handle_client(&mut stream);
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
    
    Ok(())
}

fn handle_client(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("Client disconnected");
                break;
            }
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("Received: {}", message);
                
                let response = match message.as_ref() {
                    "PING" => "PONG",
                    "HELLO" => "WELCOME",
                    _ => "UNKNOWN",
                };
                
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    println!("Error sending response: {}", e);
                    break;
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
    
    // Відправляємо тестове повідомлення
    stream.write_all(b"PING")?;
    
    // Читаємо відповідь
    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..n]);
    
    println!("Received: {}", response);
    
    Ok(())
}