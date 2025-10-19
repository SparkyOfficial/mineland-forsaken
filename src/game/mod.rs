// геймплей
// author: Андрій Будильников

use bevy::prelude::*;

// компонент для гравця
#[derive(Component)]
pub struct Player;

// компонент для ворога (хоррор елемент)
#[derive(Component)]
pub struct Enemy;

pub fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // створюємо гравця
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        })
        .insert(Player);
    
    // створюємо ворога
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 2.0, 1.0)),
            material: materials.add(Color::rgb(0.5, 0.2, 0.3)),
            transform: Transform::from_xyz(5.0, 1.0, 0.0),
            ..default()
        })
        .insert(Enemy);
    
    // створюємо підлогу
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(10.0, 10.0)),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        });
    
    // камера від першої особи
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
    
    // світло
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

pub fn cleanup_game(mut commands: Commands) {
    // тут буде очистка гри
    println!("cleaning up game...");
}

pub fn game_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    // простий рух гравця
    for mut transform in player_query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let speed = 5.0;
        
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 0.0, -1.0);
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, 0.0, 1.0);
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        
        transform.translation += direction * speed * time.delta_seconds();
    }
}