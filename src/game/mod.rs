// геймплей
// author: Андрій Будильников

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::{PrimaryWindow, CursorGrabMode};

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
            Player
        ));
    
    // створюємо ворога
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(1.0, 2.0, 1.0)),
                material: materials.add(Color::srgb(0.5, 0.2, 0.3)),
                transform: Transform::from_xyz(5.0, 1.0, 0.0),
                ..default()
            },
            Enemy
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
    
    // налаштовуємо курсор
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::Confined;
        window.cursor.visible = false;
    }
}

pub fn cleanup_game(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    // повертаємо курсор
    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
    
    // тут буде очистка гри
    println!("cleaning up game...");
}

pub fn game_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
    mut mouse_state: ResMut<MouseState>,
    mut mouse_events: EventReader<MouseMotion>,
) {
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
    for mut player_transform in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let speed = 5.0;
        
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