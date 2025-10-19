// мережа
// author: Андрій Будильников

use bevy::prelude::*;

// компонент для мережевого гравця
#[derive(Component)]
pub struct NetworkPlayer {
    pub player_id: u32,
}

// структура для мережевого повідомлення
#[derive(Event)]
pub struct NetworkMessage {
    pub sender_id: u32,
    pub content: String,
}

pub fn setup_network(_commands: Commands) {
    // тут буде ініціалізація мережі
    println!("setting up network...");
}

pub fn network_system() {
    // тут буде логіка мережі
    println!("network system running...");
}