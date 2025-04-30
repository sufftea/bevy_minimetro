use bevy::{
    math::VectorSpace,
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use metro::{LineId, MAP_SIZE, Metro, MetroResources};
use utils::STATION_MESHES;

use crate::AppState;
use bevy::color::palettes::basic as colors;

mod line_connector;
mod metro;
mod ui;
mod utils;

#[derive(Resource)]
pub struct BestScore(pub u32);

#[derive(Component)]
pub struct GameComponent;

#[derive(Event)]
pub struct ActiveLinesChanged;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ui::plugin)
        .add_plugins(line_connector::plugin)
        .insert_resource(BestScore(0))
        .insert_resource(Metro::new())
        .insert_resource(MetroResources::new())
        .add_event::<ActiveLinesChanged>()
        .add_systems(OnEnter(AppState::Game), (setup_scene, scale_view).chain())
        .add_systems(Update, scale_view.run_if(on_event::<WindowResized>));
}

fn setup_scene(
    mut commands: Commands,
    metro_res: Res<Metro>,
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

    for station in &metro_res.stations {
        const BORDER_SCALE: f32 = 1.2;
        const INNER_COLOR: Srgba = colors::GRAY;
        const BORDER_COLOR: Srgba = colors::WHITE;

        commands.spawn((
            GameComponent,
            Mesh2d(meshes.add(STATION_MESHES.square())),
            Transform::from_translation(station.position.extend(1.0)),
            children![
                (
                    match station.kind {
                        metro::StationKind::Square => Mesh2d(meshes.add(STATION_MESHES.square())),
                        metro::StationKind::Triangle =>
                            Mesh2d(meshes.add(STATION_MESHES.triangle())),
                        metro::StationKind::Circle => Mesh2d(meshes.add(STATION_MESHES.circle())),
                    },
                    MeshMaterial2d(materials.add(Color::from(INNER_COLOR))),
                    Transform::from_translation(Vec3::new(0., 0., 1.)),
                ),
                (
                    match station.kind {
                        metro::StationKind::Square => Mesh2d(meshes.add(STATION_MESHES.square())),
                        metro::StationKind::Triangle =>
                            Mesh2d(meshes.add(STATION_MESHES.triangle())),
                        metro::StationKind::Circle => Mesh2d(meshes.add(STATION_MESHES.circle())),
                    },
                    MeshMaterial2d(materials.add(Color::from(BORDER_COLOR))),
                    Transform::from_translation(Vec3::ZERO).with_scale(Vec3::ONE * BORDER_SCALE),
                )
            ],
        ));
    }
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
