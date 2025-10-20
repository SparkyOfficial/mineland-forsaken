// ігрова логіка
// author: Андрій Будильников

use bevy::prelude::*;
use crate::game::timer::{GameTimer, GameState};

// функція для обробки вбивства вижившого
pub fn on_survivor_killed(
    game_timer: &mut ResMut<GameTimer>,
) {
    // додаємо 30 секунд до таймера гри
    let current_duration = game_timer.game_duration.duration();
    let new_duration = current_duration + std::time::Duration::from_secs(30);
    game_timer.game_duration.set_duration(new_duration);
    
    // збільшуємо лічильник вбитих виживших
    game_timer.survivors_killed += 1;
    
    println!("Survivor killed! +30 seconds added to game timer");
}

// функція для активації Last Man Standing
pub fn activate_last_man_standing(
    game_timer: &mut ResMut<GameTimer>,
) {
    // встановлюємо таймер на 1 хвилину 15 секунд
    game_timer.last_man_timer = Some(Timer::from_seconds(75.0, TimerMode::Once));
    game_timer.game_state = GameState::LastManStanding;
    
    println!("Last Man Standing activated! Timer set to 1:15");
}

// функція для перевірки умов гри
pub fn check_game_conditions(
    game_timer: &mut ResMut<GameTimer>,
    survivor_count: u32, // кількість виживших
    killer_alive: bool, // чи живий убийця
) {
    // якщо убийця вбив усіх виживших
    if survivor_count == 0 && killer_alive {
        game_timer.game_state = GameState::GameOver;
        println!("Game over - killer eliminated all survivors!");
        return;
    }
    
    // якщо залишився лише один виживший
    if survivor_count == 1 && killer_alive && game_timer.game_state != GameState::LastManStanding {
        activate_last_man_standing(game_timer);
    }
    
    // якщо убийця мертвий
    if !killer_alive {
        game_timer.game_state = GameState::GameOver;
        println!("Game over - killer defeated!");
    }
}