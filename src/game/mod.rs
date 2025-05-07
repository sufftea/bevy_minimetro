use bevy::{
    math::VectorSpace,
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use lines::{LineDragHandle, StationLineDragTarget};
use metro::{LineId, MAP_SIZE, Metro, MetroResources, StationId};
use utils::STATION_MESHES;

use crate::AppState;
use bevy::color::palettes::basic as colors;

pub mod events;
pub mod lines;
pub mod lines_visual;
pub mod metro;
pub mod stations;
pub mod ui;
pub mod utils;

#[derive(Resource)]
pub struct BestScore(pub u32);

#[derive(Component)]
pub struct GameComponent;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(lines::plugin)
        .add_plugins(lines_visual::plugin)
        .add_plugins(ui::plugin)
        .add_plugins(stations::plugin)
        .add_plugins(metro::plugin)
        .add_plugins(events::plugin)
        .insert_resource(BestScore(0))
        .add_systems(OnEnter(AppState::Game), (setup_scene, scale_view).chain())
        .add_systems(
            Update,
            scale_view
                .run_if(in_state(AppState::Game))
                .run_if(on_event::<WindowResized>),
        );
}

fn setup_scene(
    mut commands: Commands,
    metro: Res<Metro>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        GameComponent,
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        GameComponent,
        Mesh2d(meshes.add(Rectangle::new(200., 0.5))),
        MeshMaterial2d(materials.add(Color::from(colors::PURPLE))),
        Transform::from_xyz(0., 0., 0.),
    ));
    commands.spawn((
        GameComponent,
        Mesh2d(meshes.add(Rectangle::new(0.5, 200.))),
        MeshMaterial2d(materials.add(Color::from(colors::PURPLE))),
        Transform::from_xyz(0., 0., 0.),
    ));
}

fn scale_view(
    mut projection: Single<&mut Projection, With<Camera2d>>,
    window: Single<&Window, With<PrimaryWindow>>,
) {
    let window_size = window.size();

    let scale_factor = if MAP_SIZE.x / MAP_SIZE.y > window_size.x / window_size.y {
        MAP_SIZE.x / window_size.x
    } else {
        MAP_SIZE.y / window_size.y
    };

    **projection = Projection::Orthographic(OrthographicProjection {
        scale: scale_factor,
        ..OrthographicProjection::default_2d()
    });
}
