use bevy::{
    ecs::query::QueryFilter,
    prelude::*,
    render::{camera::ScalingMode, view::window},
    window::{PrimaryWindow, WindowResized, WindowResolution},
};
use metro::{MAP_SIZE, Metro};
use utils::STATION_MESHES;

use crate::AppState;
use bevy::color::palettes::basic as colors;

mod metro;
mod utils;

#[derive(Resource)]
pub struct BestScore(pub u32);

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(BestScore(0))
        .insert_resource(Metro::new())
        .add_systems(OnEnter(AppState::Game), setup_scene)
        .add_systems(Update, scale_view);
    // .add_systems(Update, on_update);
}

fn setup_scene(
    mut commands: Commands,
    metro_res: Res<Metro>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            ..OrthographicProjection::default_2d()
        },
    ));

    for station in &metro_res.stations {
        const BORDER_SCALE: f32 = 1.2;
        const INNER_COLOR: Srgba = colors::GRAY;
        const BORDER_COLOR: Srgba = colors::WHITE;
        println!("drawing a station");
        match station.kind {
            metro::StationKind::Square => {
                commands.spawn((
                    Mesh2d(meshes.add(STATION_MESHES.square())),
                    MeshMaterial2d(materials.add(Color::from(INNER_COLOR))),
                    Transform::from_translation(station.position.extend(1.0)),
                ));
                commands.spawn((
                    Mesh2d(meshes.add(STATION_MESHES.square())),
                    MeshMaterial2d(materials.add(Color::from(BORDER_COLOR))),
                    Transform::from_translation(station.position.extend(0.0))
                        .with_scale(Vec3::ONE * BORDER_SCALE),
                ));
            }
            metro::StationKind::Triangle => {
                commands.spawn((
                    Mesh2d(meshes.add(STATION_MESHES.triangle())),
                    MeshMaterial2d(materials.add(Color::from(INNER_COLOR))),
                    Transform::from_translation(station.position.extend(1.0)),
                ));

                commands.spawn((
                    Mesh2d(meshes.add(STATION_MESHES.triangle())),
                    MeshMaterial2d(materials.add(Color::from(BORDER_COLOR))),
                    Transform::from_translation(station.position.extend(0.0))
                        .with_scale(Vec3::ONE * BORDER_SCALE),
                ));
            }
            metro::StationKind::Circle => {
                commands.spawn((
                    Mesh2d(meshes.add(STATION_MESHES.circle())),
                    MeshMaterial2d(materials.add(Color::from(INNER_COLOR))),
                    Transform::from_translation(station.position.extend(1.0)),
                ));

                commands.spawn((
                    Mesh2d(meshes.add(STATION_MESHES.circle())),
                    MeshMaterial2d(materials.add(Color::from(BORDER_COLOR))),
                    Transform::from_translation(station.position.extend(0.0))
                        .with_scale(Vec3::ONE * BORDER_SCALE),
                ));
            }
        };
    }
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(200., 0.5))),
        MeshMaterial2d(materials.add(Color::from(colors::PURPLE))),
        Transform::from_xyz(0., 0., 0.),
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(0.5, 200.))),
        MeshMaterial2d(materials.add(Color::from(colors::PURPLE))),
        Transform::from_xyz(0., 0., 0.),
    ));
}

fn scale_view(
    mut projection: Single<&mut OrthographicProjection, With<Camera2d>>,
    // window: Single<&Window, With<PrimaryWindow>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.read() {
        println!("resizing window:  {} / {}", e.width, MAP_SIZE.x);

        let scale_factor = if MAP_SIZE.x / MAP_SIZE.y > e.width / e.height {
            MAP_SIZE.x / e.width
        } else {
            MAP_SIZE.y / e.height
        };

        // let scale_factor = e.width / MAP_SIZE.x;
        projection.scale = scale_factor;

        // projection.scaling_mode = ScalingMode::Fixed {
        //     width: MAP_SIZE.x * scale_factor,
        //     height: MAP_SIZE.y * scale_factor,
        // };
    }
}

fn on_update(commands: Commands) {}
