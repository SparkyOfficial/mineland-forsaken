// геймплей
// author: Андрій Будильников

use bevy::prelude::*;

// компонент для гравця
#[derive(Component)]
pub struct Player;

// компонент для ворога (хоррор елемент)
#[derive(Component)]
pub struct Enemy;

// компонент для камери гравця
#[derive(Component)]
pub struct PlayerCamera;

pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
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
}

pub fn cleanup_game(_commands: Commands) {
    // тут буде очистка гри
    println!("cleaning up game...");
}

pub fn game_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<Player>)>,
) {
    // простий рух гравця
    for mut player_transform in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let speed = 5.0;
        
        // отримуємо напрямки з матриці трансформації
        let forward = player_transform.forward();
        let right = player_transform.right();
        
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(forward.x, 0.0, forward.z);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(-forward.x, 0.0, -forward.z);
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-right.x, 0.0, -right.z);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(right.x, 0.0, right.z);
        }
        
        // рухаємо і гравця, і камеру разом
        if direction.length() > 0.0 {
            direction = direction.normalize();
            player_transform.translation += direction * speed * time.delta_seconds();
            
            // оновлюємо камеру
            if let Ok(mut camera_transform) = camera_query.get_single_mut() {
                camera_transform.translation = player_transform.translation + Vec3::new(0.0, 0.5, 0.0);
            }
        }
    }
}