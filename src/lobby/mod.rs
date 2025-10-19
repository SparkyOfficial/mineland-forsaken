// лоббі гри
// author: Андрій Будильников

use bevy::prelude::*;

// компонент для елементів лоббі
#[derive(Component)]
pub struct LobbyElement;

// компонент для кнопки старту гри
#[derive(Component)]
pub struct StartGameButton;

pub fn setup_lobby(mut commands: Commands, asset_server: Res<AssetServer>) {
    // створюємо ui камеру для лоббі
    commands.spawn(Camera2dBundle::default());
    
    // створюємо інтерфейс лоббі
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // заголовок лоббі
            parent.spawn(
                TextBundle::from_section(
                    "Лоббі",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 60.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(20.0)),
                    ..default()
                }),
            );
            
            // список гравців
            parent.spawn(
                TextBundle::from_section(
                    "Гравці: 1/4",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                }),
            );
            
            // кнопка старту гри
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(20.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .insert(LobbyElement)
                .insert(StartGameButton)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Почати гру",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

pub fn cleanup_lobby(mut commands: Commands, lobby_query: Query<Entity, With<LobbyElement>>) {
    // видаляємо всі елементи лоббі
    for entity in lobby_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn lobby_system(
    mut state: ResMut<NextState<crate::GameState>>,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<StartGameButton>)>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            // перехід до гри
            state.set(crate::GameState::Game);
        }
    }
}