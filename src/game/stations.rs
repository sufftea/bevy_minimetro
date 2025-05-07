use bevy::ecs::spawn::SpawnRelatedBundle;
use bevy::prelude::*;
use bevy::{color::palettes::basic as colors, platform::collections::HashMap};
use std::f32::consts::SQRT_2;

use crate::{
    AppState,
    game::{
        lines::{LineDragHandle, StationLineDragTarget},
        metro,
        utils::STATION_MESHES,
    },
};

const STATION_MESH_RADIUS: f32 = 4.;

use super::{
    GameComponent,
    metro::{Metro, StationId, StationKind},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(AppState::Game),
        (create_station_meshes, setup_scene).chain(),
    )
    .add_observer(on_station_spawned);
}

#[derive(Component)]
struct StationComponent {
    station_id: StationId,
    position: Vec2,
}

fn on_station_spawned(
    trigger: Trigger<OnAdd, StationComponent>,
    station_component_q: Query<&StationComponent>,
    mut commands: Commands,
    // station: Station,
    station_meshes: Res<StationMeshes>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    const BORDER_SCALE: f32 = 1.2;
    const INNER_COLOR: Srgba = colors::GRAY;
    const BORDER_COLOR: Srgba = colors::WHITE;

    let Ok(station_component) = station_component_q.get(trigger.target()) else {
        return;
    };

    commands.spawn((
        GameComponent,
        LineDragHandle::New {
            station_id: station_component.station_id,
        },
        StationLineDragTarget {
            station_id: station_component.station_id,
        },
        Mesh2d(
            station_meshes
                .meshes
                .get(&station_component.station_id)
                .unwrap()
                .clone(),
        ),
        Transform::from_translation(station_component.position.extend(1.0)),
        children![
            (
                Mesh2d(
                    station_meshes
                        .meshes
                        .get(&station_component.station_id)
                        .unwrap()
                        .clone(),
                ),

                MeshMaterial2d(materials.add(Color::from(INNER_COLOR))),
                Transform::from_translation(Vec3::new(0., 0., 1.)),
            ),
            (
                Mesh2d(
                    station_meshes
                        .meshes
                        .get(&station_component.station_id)
                        .unwrap()
                        .clone(),
                ),
                MeshMaterial2d(materials.add(Color::from(BORDER_COLOR))),
                Transform::from_translation(Vec3::ZERO).with_scale(Vec3::ONE * BORDER_SCALE),
            )
        ],
    ));
}
fn create_station_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mut mesh_map = HashMap::new();
    mesh_map.insert(0, Mesh::from(square()));
    mesh_map.insert(1, Mesh::from(triangle()));
    mesh_map.insert(2, Mesh::from(circle()));

    commands.insert_resource(StationMeshes {
        meshes: mesh_map
            .iter()
            .map(|(station_kind, mesh)| (*station_kind, meshes.add(mesh.clone())))
            .collect(),
    });
}

fn circle() -> Circle {
    Circle::new(STATION_MESH_RADIUS)
}

fn triangle() -> Triangle2d {
    let angle1 = 90.0_f32.to_radians();
    let angle2 = 210.0_f32.to_radians();
    let angle3 = 330.0_f32.to_radians();

    Triangle2d::new(
        Vec2::new(
            STATION_MESH_RADIUS * angle1.cos(),
            STATION_MESH_RADIUS * angle1.sin(),
        ),
        Vec2::new(
            STATION_MESH_RADIUS * angle2.cos(),
            STATION_MESH_RADIUS * angle2.sin(),
        ),
        Vec2::new(
            STATION_MESH_RADIUS * angle3.cos(),
            STATION_MESH_RADIUS * angle3.sin(),
        ),
    )
}

fn square() -> Rectangle {
    Rectangle::from_length(STATION_MESH_RADIUS * SQRT_2)
}

#[derive(Resource)]
struct StationMeshes {
    meshes: HashMap<StationKind, Handle<Mesh>>,
}

fn setup_scene(mut commands: Commands, metro: Res<Metro>) {
    for (station_id, station) in metro.stations.iter().enumerate() {
        commands.spawn(StationComponent {
            station_id,
            position: station.position,
        });
    }
}
