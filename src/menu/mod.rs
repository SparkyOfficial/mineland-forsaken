// меню гри
// author: Андрій Будильников

use bevy::prelude::*;

// компонент для кнопок меню
#[derive(Component)]
pub struct MenuButton;

// компонент для визначення типу кнопки
#[derive(Component)]
pub enum ButtonType {
    Play,
    Settings,
    Quit,
}

pub fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // створюємо ui камеру
    commands.spawn(Camera2dBundle::default());
    
    // створюємо інтерфейс меню
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
            // заголовок
            parent.spawn(
                TextBundle::from_section(
                    "Mineland Forsaken",
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
            
            // кнопка грати
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .insert(MenuButton)
                .insert(ButtonType::Play)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Грати",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ));
                });
            
            // кнопка налаштувань
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .insert(MenuButton)
                .insert(ButtonType::Settings)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Налаштування",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ));
                });
            
            // кнопка виходу
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(250.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Px(10.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                    ..default()
                })
                .insert(MenuButton)
                .insert(ButtonType::Quit)
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Вийти",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ));
                });
        });
}

pub fn cleanup_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuButton>>) {
    // видаляємо всі кнопки меню
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn menu_system(
    mut state: ResMut<NextState<crate::GameState>>,
    mut interaction_query: Query<
        (&Interaction, &ButtonType),
        (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, button_type) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button_type {
                ButtonType::Play => {
                    // перехід до лоббі
                    state.set(crate::GameState::Lobby);
                }
                ButtonType::Settings => {
                    // тут будуть налаштування
                    println!("settings button pressed");
                }
                ButtonType::Quit => {
                    // вихід з гри
                    std::process::exit(0);
                }
            }
        }
    }
}