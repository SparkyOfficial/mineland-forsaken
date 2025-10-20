// геймплей
// author: Андрій Будильников

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::{PrimaryWindow, CursorGrabMode};

pub mod characters;
pub mod timer;
pub mod game_logic;

use characters::{GameCharacter, CharacterType};
use timer::{GameTimer, TimerDisplay, HealthStaminaHUD, PlayerList, ShopButton, InventoryButton, Shop, GameState};

// компонент для гравця
#[derive(Component)]
pub struct Player;

// компонент для ворога (хоррор елемент)
#[derive(Component)]
pub struct Enemy;

// компонент для камери гравця
#[derive(Component)]
pub struct PlayerCamera;

// компонент для миші
#[derive(Resource, Default)]
pub struct MouseState {
    pub pitch: f32,
    pub yaw: f32,
}

pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // створюємо гравця (невидимий в first-person)
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
                transform: Transform::from_xyz(0.0, 1.0, 0.0),
                visibility: Visibility::Hidden, // ховаємо гравця в first-person
                ..default()
            },
            Player,
            GameCharacter::new(CharacterType::Casual) // стандартний персонаж
        ));
    
    // створюємо убийцю
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.5, 2.5, 1.5)),
                material: materials.add(Color::srgb(0.2, 0.5, 0.2)),
                transform: Transform::from_xyz(8.0, 1.0, 0.0),
                ..default()
            },
            Enemy,
            characters::Killer::new(characters::KillerType::AlexRadievskiy)
        ));
    
    // створюємо підлогу
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20.0, 20.0)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        });
    
    // створюємо стіни
    // задня стіна
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(20.0, 5.0, 1.0)),
        material: materials.add(Color::srgb(0.4, 0.4, 0.4)),
        transform: Transform::from_xyz(0.0, 2.5, -10.0),
        ..default()
    });
    
    // передня стіна
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(20.0, 5.0, 1.0)),
        material: materials.add(Color::srgb(0.4, 0.4, 0.4)),
        transform: Transform::from_xyz(0.0, 2.5, 10.0),
        ..default()
    });
    
    // ліва стіна
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 5.0, 20.0)),
        material: materials.add(Color::srgb(0.4, 0.4, 0.4)),
        transform: Transform::from_xyz(-10.0, 2.5, 0.0),
        ..default()
    });
    
    // права стіна
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 5.0, 20.0)),
        material: materials.add(Color::srgb(0.4, 0.4, 0.4)),
        transform: Transform::from_xyz(10.0, 2.5, 0.0),
        ..default()
    });
    
    // переміщуємо камеру для гри (first-person view)
    for mut camera_transform in camera_query.iter_mut() {
        camera_transform.translation = Vec3::new(0.0, 1.5, 0.0); // очі гравця
        camera_transform.look_at(Vec3::new(0.0, 1.5, 1.0), Vec3::Y);
    }
    
    // додаємо стан миші
    commands.insert_resource(MouseState::default());
    
    // додаємо таймер гри
    commands.insert_resource(GameTimer::default());
    
    // додаємо магазин
    commands.insert_resource(Shop::new());
    
    // створюємо інтерфейс таймера
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            TimerDisplay
        ))
        .with_children(|parent| {
            // таймер зверху
            parent.spawn((
                TextBundle::from_section(
                    "3:00",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    ..default()
                }),
                TimerDisplay
            ));
            
            // кнопки магазину та інвентарю зліва
            parent.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        left: Val::Px(20.0),
                        top: Val::Px(100.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
                TimerDisplay
            ))
            .with_children(|parent| {
                // кнопка магазину
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(120.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            background_color: Color::srgb(0.2, 0.2, 0.6).into(),
                            ..default()
                        },
                        ShopButton,
                        TimerDisplay
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Магазин",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ),
                            TimerDisplay
                        ));
                    });
                
                // кнопка інвентарю
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(120.0),
                                height: Val::Px(50.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgb(0.2, 0.6, 0.2).into(),
                            ..default()
                        },
                        InventoryButton,
                        TimerDisplay
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Інвентар",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                                    font_size: 20.0,
                                    color: Color::WHITE,
                                },
                            ),
                            TimerDisplay
                        ));
                    });
            });
            
            // HUD здоров'я та бігу
            parent.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(20.0),
                        left: Val::Px(20.0),
                        width: Val::Px(200.0),
                        height: Val::Px(80.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::srgb(0.1, 0.1, 0.1).into(),
                    ..default()
                },
                HealthStaminaHUD
            ))
            .with_children(|parent| {
                // здоров'я
                parent.spawn((
                    TextBundle::from_section(
                        "Здоров'я: 100",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                            font_size: 16.0,
                            color: Color::srgb(1.0, 0.2, 0.2),
                        },
                    ),
                    HealthStaminaHUD
                ));
                
                // біг
                parent.spawn((
                    TextBundle::from_section(
                        "Біг: 100",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                            font_size: 16.0,
                            color: Color::srgb(0.2, 0.6, 1.0),
                        },
                    ),
                    HealthStaminaHUD
                ));
            });
            
            // список гравців над HUD
            parent.spawn((
                NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(120.0),
                        left: Val::Px(20.0),
                        width: Val::Px(300.0),
                        height: Val::Px(150.0),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: Color::srgb(0.1, 0.1, 0.1).into(),
                    ..default()
                },
                PlayerList
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_section(
                        "Гравці:",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 18.0,
                            color: Color::WHITE,
                        },
                    ),
                    PlayerList
                ));
                
                // приклад гравців
                parent.spawn((
                    TextBundle::from_section(
                        "Виживший: Casual",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                            font_size: 14.0,
                            color: Color::srgb(0.2, 1.0, 0.2),
                        },
                    ),
                    PlayerList
                ));
                
                parent.spawn((
                    TextBundle::from_section(
                        "Убийця: AlexRadievskiy",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                            font_size: 14.0,
                            color: Color::srgb(1.0, 0.2, 0.2),
                        },
                    ),
                    PlayerList
                ));
            });
        });
    
    // налаштовуємо курсор
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = false;
    }
}

pub fn cleanup_game(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    timer_display_query: Query<Entity, With<TimerDisplay>>,
    mut commands: Commands,
) {
    // повертаємо курсор
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
    
    // видаляємо інтерфейс таймера
    for entity in timer_display_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    // тут буде очистка гри
    println!("cleaning up game...");
}

pub fn game_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut GameCharacter), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mut mouse_state: ResMut<MouseState>,
    mut mouse_events: EventReader<MouseMotion>,
    mut game_timer: ResMut<GameTimer>,
    mut timer_text_query: Query<&mut Text, With<TimerDisplay>>,
    mut health_text_query: Query<(Entity, &mut Text), With<HealthStaminaHUD>>,
) {
    // оновлюємо таймери
    game_timer.timer.tick(time.delta());
    game_timer.round_timer.tick(time.delta());
    game_timer.game_duration.tick(time.delta());
    
    // оновлюємо таймер Last Man Standing якщо активний
    if let Some(ref mut last_man_timer) = game_timer.last_man_timer {
        last_man_timer.tick(time.delta());
    }
    
    // оновлюємо відображення таймера
    if let Ok(mut timer_text) = timer_text_query.get_single_mut() {
        // відображаємо відповідний таймер залежно від стану гри
        let time_display = match game_timer.game_state {
            GameState::LastManStanding => {
                if let Some(ref last_man_timer) = game_timer.last_man_timer {
                    let remaining = last_man_timer.remaining_secs();
                    let minutes = (remaining / 60.0) as u32;
                    let seconds = (remaining % 60.0) as u32;
                    format!("{}:{:02}", minutes, seconds)
                } else {
                    "1:15".to_string()
                }
            },
            _ => {
                let remaining = game_timer.game_duration.remaining_secs();
                let minutes = (remaining / 60.0) as u32;
                let seconds = (remaining % 60.0) as u32;
                format!("{}:{:02}", minutes, seconds)
            }
        };
        timer_text.sections[0].value = time_display;
    }
    
    // оновлюємо HUD здоров'я та бігу
    let mut text_entities: Vec<Entity> = Vec::new();
    let mut text_components: Vec<Mut<Text>> = Vec::new();
    
    for (entity, text) in health_text_query.iter_mut() {
        text_entities.push(entity);
        text_components.push(text);
    }
    
    if !text_components.is_empty() {
        if let Ok((_, character)) = player_query.get_single() {
            // оновлюємо здоров'я (перший текст)
            text_components[0].sections[0].value = format!("Здоров'я: {:.0}", character.health);
            
            // оновлюємо біг (другий текст, якщо існує)
            if text_components.len() > 1 {
                text_components[1].sections[0].value = format!("Біг: {:.0}", character.stamina);
            }
        }
    }
    
    // перевіряємо чи закінчився раунд
    if game_timer.round_timer.finished() {
        // телепортуємо всіх на іншу карту (в реалізації просто переміщуємо)
        for (mut player_transform, _) in player_query.iter_mut() {
            player_transform.translation = Vec3::new(
                fastrand::f32() * 20.0 - 10.0, // випадкова позиція по X
                1.0,
                fastrand::f32() * 20.0 - 10.0, // випадкова позиція по Z
            );
        }
        
        // перезапускаємо таймер
        game_timer.round_timer.reset();
    }
    
    // перевіряємо чи закінчилась гра (3 хвилини)
    if game_timer.game_duration.finished() && game_timer.game_state != GameState::LastManStanding {
        game_timer.game_state = GameState::GameOver;
        println!("Game over - time's up!");
    }
    
    // перевіряємо чи активний таймер Last Man Standing
    if let Some(ref last_man_timer) = game_timer.last_man_timer {
        if last_man_timer.finished() {
            game_timer.game_state = GameState::GameOver;
            println!("Game over - Last Man Standing timer expired!");
        }
    }
    
    // обробка руху миші (працює завжди, не тільки під час руху)
    let mut mouse_delta = Vec2::ZERO;
    for event in mouse_events.read() {
        mouse_delta += event.delta;
    }
    
    // оновлюємо стан миші (виправлена інверсія)
    if mouse_delta.length_squared() > 0.0 {
        mouse_state.yaw -= mouse_delta.x * 0.002;
        mouse_state.pitch += mouse_delta.y * 0.002; // виправлено інверсію
        
        // обмежуємо кут нахилу
        mouse_state.pitch = mouse_state.pitch.clamp(-1.5, 1.5);
    }
    
    // оновлюємо обертання камери (працює завжди)
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        camera_transform.rotation = Quat::from_euler(EulerRot::ZYX, mouse_state.pitch, mouse_state.yaw, 0.0);
    }
    
    // простий рух гравця
    for (mut player_transform, mut character) in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let mut speed = 5.0;
        
        // біг на Shift
        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            if character.stamina > 0.0 {
                speed *= 1.5; // біг швидший
                character.stamina -= 10.0 * time.delta_seconds(); // витрата стаміни
            }
        } else {
            // відновлення стаміни
            character.stamina = (character.stamina + 5.0 * time.delta_seconds()).min(character.max_stamina);
        }
        
        // отримуємо напрямки з миші
        let yaw = mouse_state.yaw;
        
        // обчислюємо напрямки на основі миші
        let forward = Vec3::new(-yaw.sin(), 0.0, -yaw.cos()).normalize();
        let right = Vec3::new(yaw.cos(), 0.0, -yaw.sin()).normalize();
        
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += forward;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= forward;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= right;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += right;
        }
        
        // рухаємо гравця
        if direction.length() > 0.0 {
            direction = direction.normalize();
            player_transform.translation += direction * speed * time.delta_seconds();
            
            // оновлюємо позицію камери
            if let Ok(mut camera_transform) = camera_query.get_single_mut() {
                camera_transform.translation = player_transform.translation + Vec3::new(0.0, 0.5, 0.0);
            }
        }
    }
}