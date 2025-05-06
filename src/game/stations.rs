use bevy::color::palettes::basic as colors;
use bevy::prelude::*;

use crate::{
    AppState,
    game::{
        lines::{LineDragHandle, StationLineDragTarget},
        metro,
        utils::STATION_MESHES,
    },
};

use super::{GameComponent, metro::Metro};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), setup_scene);
}

fn setup_scene(
    mut commands: Commands,
    metro: Res<Metro>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (station_id, station) in metro.stations.iter().enumerate() {
        const BORDER_SCALE: f32 = 1.2;
        const INNER_COLOR: Srgba = colors::GRAY;
        const BORDER_COLOR: Srgba = colors::WHITE;

        commands.spawn((
            GameComponent,
            // StationEntityData { station_id: i },
            LineDragHandle::New { station_id },
            StationLineDragTarget { station_id },
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
}
