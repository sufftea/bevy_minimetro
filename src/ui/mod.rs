use bevy::prelude::*;

use crate::{AppState, style};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(style::BACKGROUND.into()))
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(OnExit(AppState::MainMenu), clean_up_main_menu);
    // app.add_systems(schedule, systems);
}

#[derive(Component)]
struct MainMenuComponent;

#[derive(Component)]
enum ButtonName {
    Start,
}

fn setup_main_menu(mut commands: Commands, best_score: Res<crate::game::BestScore>) {
    commands.spawn((Camera2d, MainMenuComponent));
    commands
        .spawn((
            MainMenuComponent,
            Node {
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                width: Val::Px(400.),
                height: Val::Auto,
                ..default()
            },
        ))
        .with_children(|parent| {
            let button = parent
                .spawn((
                    Button,
                    Node {
                        height: Val::Px(64.),
                        width: Val::Percent(100.),
                        ..default()
                    },
                    BorderColor(style::PRIMARY.into()),
                    BackgroundColor(style::CONTAINER_PRIMARY.into()),
                ))
                .with_child((
                    Text::new("Start"),
                    TextFont {
                        font_size: 33.0,
                        ..default()
                    },
                    TextColor(style::ON_PRIMARY.into()),
                ))
                .observe(|_: Trigger<Pointer<Click>>, mut commands: Commands| {
                    commands.set_state(AppState::Game);
                });

            parent.spawn((
                Text::new(format!("Best score: {}", best_score.0)),
                TextFont {
                    font_size: 33.0,
                    ..default()
                },
                TextColor(style::SECONDARY_VARIANT.into()),
            ));
        });
}

fn clean_up_main_menu(
    mut commands: Commands,
    main_menu_components_query: Query<Entity, With<MainMenuComponent>>,
) {
    for entity in main_menu_components_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
