// таймер гри та система раундів
// author: Андрій Будильников

use bevy::prelude::*;

// ресурс для таймера гри
#[derive(Resource)]
pub struct GameTimer {
    pub timer: Timer,
    pub round_timer: Timer, // 40 секунд на раунд
    pub game_duration: Timer, // 3 хвилини на гру
    pub last_man_timer: Option<Timer>, // 1 хвилина 15 секунд для Last Man Standing
    pub game_state: GameState,
    pub survivors_killed: u32, // кількість вбитих виживших
}

// стан гри
#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Lobby,      // лобі
    Playing,    // гра триває
    LastManStanding, // останній виживший
    GameOver,   // гра закінчена
}

impl Default for GameTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(40.0, TimerMode::Once),
            round_timer: Timer::from_seconds(40.0, TimerMode::Once),
            game_duration: Timer::from_seconds(180.0, TimerMode::Once), // 3 хвилини
            last_man_timer: None,
            game_state: GameState::Lobby,
            survivors_killed: 0,
        }
    }
}

// компонент для відображення таймера на екрані
#[derive(Component)]
pub struct TimerDisplay;

// компонент для HUD здоров'я та бігу
#[derive(Component)]
pub struct HealthStaminaHUD;

// компонент для списку гравців
#[derive(Component)]
pub struct PlayerList;

// компонент для кнопки магазину
#[derive(Component)]
pub struct ShopButton;

// компонент для кнопки інвентарю
#[derive(Component)]
pub struct InventoryButton;

// ресурс для магазину
#[derive(Resource, Default)]
pub struct Shop {
    pub items: Vec<ShopItem>,
}

// предмет у магазині
#[derive(Debug, Clone)]
pub struct ShopItem {
    pub name: String,
    pub price: u32,
    pub item_type: ItemType,
}

// тип предмета
#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Character(CharacterType),
    Weapon,
    Ability,
}

// тип персонажа
#[derive(Debug, Clone, PartialEq)]
pub enum CharacterType {
    Casual,     // безкоштовний персонаж
    Sparcy,     // 500$ - пневматична винтовка
    TimePlay,   // 500$ - Java-радар
}

impl Shop {
    pub fn new() -> Self {
        let mut shop = Self::default();
        
        // додаємо персонажів до магазину
        shop.items.push(ShopItem {
            name: "Спарки".to_string(),
            price: 500,
            item_type: ItemType::Character(CharacterType::Sparcy),
        });
        
        shop.items.push(ShopItem {
            name: "TimePlay".to_string(),
            price: 500,
            item_type: ItemType::Character(CharacterType::TimePlay),
        });
        
        shop
    }
}