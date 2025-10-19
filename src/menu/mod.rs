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

// компонент для позначення елементів меню
#[derive(Component)]
pub struct MenuElement;

pub fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // створюємо інтерфейс меню
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            MenuElement
        ))
        .with_children(|parent| {
            // заголовок
            parent.spawn((
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
                MenuElement
            ));
            
            // кнопка грати
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    MenuButton,
                    MenuElement
                ))
                .insert(ButtonType::Play)
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Грати",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        ),
                        MenuElement
                    ));
                });
            
            // кнопка налаштувань
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    MenuButton,
                    MenuElement
                ))
                .insert(ButtonType::Settings)
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Налаштування",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        ),
                        MenuElement
                    ));
                });
            
            // кнопка виходу
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(250.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                        ..default()
                    },
                    MenuButton,
                    MenuElement
                ))
                .insert(ButtonType::Quit)
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "Вийти",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Medium.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        ),
                        MenuElement
                    ));
                });
        });
}

pub fn cleanup_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<MenuElement>>,
) {
    // видаляємо всі елементи меню
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