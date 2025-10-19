// лоббі гри
// author: Андрій Будильников

use bevy::prelude::*;

// компонент для елементів лоббі
#[derive(Component)]
pub struct LobbyElement;

// компонент для кнопки старту гри
#[derive(Component)]
pub struct StartGameButton;

// компонент для кнопки повернення в меню
#[derive(Component)]
pub struct BackToMenuButton;

// компонент для гравця в лоббі
#[derive(Component)]
pub struct LobbyPlayer {
    pub name: String,
    pub is_ready: bool,
}

pub fn setup_lobby(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // створюємо інтерфейс лоббі
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::FlexStart,
                    ..default()
                },
                ..default()
            },
            LobbyElement
        ))
        .with_children(|parent| {
            // заголовок лоббі
            parent.spawn((
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
                LobbyElement
            ));
            
            // контейнер для гравців
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        height: Val::Percent(60.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::FlexStart,
                        margin: UiRect::all(Val::Px(20.0)),
                        padding: UiRect::all(Val::Px(20.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    border_color: Color::WHITE.into(),
                    background_color: Color::srgb(0.1, 0.1, 0.1).into(),
                    ..default()
                },
                LobbyElement
            )).with_children(|parent| {
                // список гравців
                parent.spawn((
                    TextBundle::from_section(
                        "Гравці в лоббі:",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(10.0)),
                        ..default()
                    }),
                    LobbyElement
                ));
                
                // контейнер для списку гравців
                parent.spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            height: Val::Percent(70.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            justify_content: JustifyContent::FlexStart,
                            ..default()
                        },
                        ..default()
                    },
                    LobbyElement
                )).with_children(|parent| {
                    // приклад гравця
                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                height: Val::Px(50.0),
                                flex_direction: FlexDirection::Row,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::SpaceBetween,
                                padding: UiRect::all(Val::Px(10.0)),
                                margin: UiRect::bottom(Val::Px(10.0)),
                                ..default()
                            },
                            background_color: Color::srgb(0.2, 0.2, 0.2).into(),
                            ..default()
                        },
                        LobbyElement
                    )).with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Гравець1 (Ви)",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                                    font_size: 24.0,
                                    color: Color::WHITE,
                                },
                            ),
                            LobbyElement
                        ));
                        
                        parent.spawn((
                            TextBundle::from_section(
                                "Готовий",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                                    font_size: 24.0,
                                    color: Color::srgb(0.0, 1.0, 0.0), // зелений
                                },
                            ),
                            LobbyElement
                        ));
                    });
                });
            });
            
            // контейнер для кнопок
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceAround,
                        margin: UiRect::top(Val::Px(20.0)),
                        ..default()
                    },
                    ..default()
                },
                LobbyElement
            )).with_children(|parent| {
                // кнопка повернення в меню
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(200.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgb(0.3, 0.3, 0.3).into(),
                            ..default()
                        },
                        BackToMenuButton,
                        LobbyElement
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Назад",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ),
                            LobbyElement
                        ));
                    });
                
                // кнопка старту гри
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Px(200.0),
                                height: Val::Px(60.0),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },
                            background_color: Color::srgb(0.15, 0.5, 0.15).into(),
                            ..default()
                        },
                        StartGameButton,
                        LobbyElement
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                "Грати",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                                    font_size: 30.0,
                                    color: Color::WHITE,
                                },
                            ),
                            LobbyElement
                        ));
                    });
            });
        });
}

pub fn cleanup_lobby(
    mut commands: Commands,
    lobby_query: Query<Entity, With<LobbyElement>>,
) {
    // видаляємо всі елементи лоббі
    for entity in lobby_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn lobby_system(
    mut state: ResMut<NextState<crate::GameState>>,
    mut interaction_query: Query<
        (&Interaction, Option<&StartGameButton>, Option<&BackToMenuButton>),
        (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, start_button, back_button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if start_button.is_some() {
                // перехід до гри
                state.set(crate::GameState::Game);
            } else if back_button.is_some() {
                // повернення в меню
                state.set(crate::GameState::Menu);
            }
        }
    }
}