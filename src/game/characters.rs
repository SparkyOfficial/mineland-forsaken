// персонажі та убийці гри
// author: Андрій Будильников

use bevy::prelude::*;

// типи персонажів
#[derive(Debug, Clone, PartialEq)]
pub enum CharacterType {
    Casual,     // безкоштовний персонаж
    Sparcy,     // 500$ - пневматична винтовка
    TimePlay,   // 500$ - Java-радар
}

// типи убийців
#[derive(Debug, Clone, PartialEq)]
pub enum KillerType {
    AlexRadievskiy, // мутант з Чорнобиля
}

// компонент для персонажа гравця
#[derive(Component)]
pub struct GameCharacter {
    pub character_type: CharacterType,
    pub health: f32,
    pub max_health: f32,
    pub stamina: f32,
    pub max_stamina: f32,
    pub money: u32,
}

// компонент для убийці
#[derive(Component)]
pub struct Killer {
    pub killer_type: KillerType,
    pub health: f32,
    pub max_health: f32,
    pub stamina: f32,
    pub max_stamina: f32,
}

// компонент для зброї
#[derive(Component)]
pub struct Weapon {
    pub name: String,
    pub damage: f32,
    pub cooldown: f32, // в секундах
    pub last_used: f32, // час останнього використання
}

// компонент для здібностей убийці
#[derive(Component)]
pub struct KillerAbility {
    pub name: String,
    pub ability_type: AbilityType,
    pub cooldown: f32, // в секундах
    pub last_used: f32, // час останнього використання
}

// типи здібностей
#[derive(Debug, Clone, PartialEq)]
pub enum AbilityType {
    Claws,      // кігті
    PoisonSpit, // плювок ядом
    AllSeeingEye, // всевидюче око
}

// компонент для радару
#[derive(Component)]
pub struct Radar {
    pub cooldown: f32,
    pub last_used: f32,
}

impl GameCharacter {
    pub fn new(character_type: CharacterType) -> Self {
        match character_type {
            CharacterType::Casual => Self {
                character_type,
                health: 100.0,
                max_health: 100.0,
                stamina: 100.0,
                max_stamina: 100.0,
                money: 0,
            },
            CharacterType::Sparcy => Self {
                character_type,
                health: 100.0,
                max_health: 100.0,
                stamina: 100.0,
                max_stamina: 100.0,
                money: 500,
            },
            CharacterType::TimePlay => Self {
                character_type,
                health: 100.0,
                max_health: 100.0,
                stamina: 100.0,
                max_stamina: 100.0,
                money: 500,
            },
        }
    }
}

impl Killer {
    pub fn new(killer_type: KillerType) -> Self {
        match killer_type {
            KillerType::AlexRadievskiy => Self {
                killer_type,
                health: 1000.0,
                max_health: 1000.0,
                stamina: 120.0,
                max_stamina: 120.0,
            },
        }
    }
}