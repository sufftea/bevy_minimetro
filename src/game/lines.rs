use bevy::{
    color::palettes::css::PURPLE, ecs::relationship::RelationshipSourceCollection, prelude::*,
};

use crate::game::lines_visual::MetroLineVisualBundle;

use super::{
    events::LinePathChanged,
    lines_visual::MetroLineVisual,
    metro::{Connection, LINE_COLORS, LineId, Metro, MetroResources, Station, StationId},
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(on_line_handle_spawned)
        .insert_resource(LineDragState::None);
}

#[derive(Resource)]
enum LineDragState {
    None,
    New {
        path: Vec<PathNode>,
        line_id: LineId,
    },

    Extend {
        path: Vec<StationId>,
        line_id: LineId,
    },

    Edit {
        line_id: LineId,
        path: Vec<StationId>,
        stations: (StationId, StationId),
    },
    // new_connections: Vec<StationId>,
    // line_id: LineId,
}

struct PathNode {
    start_station_id: StationId,
    end_station_id: Option<StationId>,
    line_entity: Entity,
}

#[derive(Component)]
pub enum LineDragHandle {
    New {
        station_id: StationId,
    },
    Extend {
        station_id: StationId,
        line_id: LineId,
    },
    Edit {
        station_ids: (StationId, StationId),
        line_id: LineId,
    },
}

#[derive(Component)]
pub struct StationLineDragTarget {
    pub station_id: StationId,
}

#[derive(Component)]
struct MetroLine {
    start_station_id: StationId,
    /// `None`, if the user is currently dragging it to another station.
    end_station_id: Option<StationId>,
    line_id: LineId,
}

fn on_line_handle_spawned(trigger: Trigger<OnAdd, LineDragHandle>, mut commands: Commands) {
    commands.spawn(Observer::new(on_drag_start).with_entity(trigger.target()));
    commands.spawn(Observer::new(on_drag).with_entity(trigger.target()));
    commands.spawn(Observer::new(on_drag_end).with_entity(trigger.target()));
}

fn on_drag_start(
    trigger: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    handle_q: Query<&LineDragHandle>,
    lines_q: Query<(Entity, &MetroLine)>,
    camera_transform_q: Single<(&Camera, &GlobalTransform)>,

    mut line_drag_state: ResMut<LineDragState>,
    metro: Res<Metro>,
    metro_resources: Res<MetroResources>,
) {
    let (camera, camera_transform) = *camera_transform_q;

    let drag_position =
        camera.viewport_to_world_2d(camera_transform, trigger.pointer_location.position);

    let Ok(drag_position) = drag_position else {
        return;
    };
    let Ok(line_handle) = handle_q.get(trigger.target()) else {
        return;
    };

    match line_handle {
        LineDragHandle::New { station_id } => {
            let active_lines = metro.get_active_lines();
            let new_line_id = (0..metro_resources.available_lines)
                .find(|line_id| !active_lines.contains(line_id));

            let Some(new_line_id) = new_line_id else {
                println!("now lines available");
                *line_drag_state = LineDragState::None;
                return;
            };

            let station = &metro.stations[*station_id];
            println!(
                "new line connection. line_id: {} \t station_id: {}",
                new_line_id, station_id
            );

            let new_line_entity = commands
                .spawn((
                    MetroLineVisualBundle::new(
                        station.position,
                        drag_position,
                        LINE_COLORS[new_line_id].into(),
                    ),
                    MetroLine {
                        start_station_id: *station_id,
                        end_station_id: None,
                        line_id: new_line_id,
                    },
                ))
                .id();

            *line_drag_state = LineDragState::New {
                path: vec![PathNode {
                    start_station_id: *station_id,
                    end_station_id: None,
                    line_entity: new_line_entity,
                }],
                line_id: new_line_id,
            };
        }
        LineDragHandle::Extend {
            station_id,
            line_id,
        } => todo!(),
        LineDragHandle::Edit {
            station_ids,
            line_id,
        } => todo!(),
    };
}

fn on_drag(
    trigger: Trigger<Pointer<Drag>>,

    mut commands: Commands,

    mut lines_q: Query<(&mut MetroLine, &mut MetroLineVisual)>,
    stations_q: Query<&StationLineDragTarget>,
    camera_transform_q: Single<(&Camera, &GlobalTransform)>,

    mut ray_cast: MeshRayCast,
    mut drag_state: ResMut<LineDragState>,
    metro: Res<Metro>,
    metro_resources: Res<MetroResources>,

    mut station_intersection_handled: Local<Option<bool>>,
) {
    let station_intersection_handled = station_intersection_handled.get_or_insert(true);

    let (camera, camera_transform) = *camera_transform_q;

    let drag_position =
        camera.viewport_to_world_2d(camera_transform, trigger.pointer_location.position);

    let Ok(drag_position) = drag_position else {
        return;
    };

    let intersectinos = ray_cast.cast_ray(
        Ray3d::new(drag_position.extend(0.), Dir3::Z),
        &MeshRayCastSettings::default().with_filter(&|entity| stations_q.contains(entity)),
    );

    let intersecting_station = if let Some((intersecting_station_entity, _)) = intersectinos.first()
    {
        stations_q.get(*intersecting_station_entity).ok()
    } else {
        None
    };

    match &mut *drag_state {
        LineDragState::None => {}
        LineDragState::New { path, line_id } => {
            let Some(last_line_node) = path.last() else {
                panic!(
                    "something weird. there should be at least one node in the path from the beginning"
                );
            };

            let Ok((mut last_line_dragging, mut lsat_line_2d_data)) =
                lines_q.get_mut(last_line_node.line_entity)
            else {
                panic!("something weird");
            };

            lsat_line_2d_data.end = drag_position;

            if let Some(intersecting_station) = intersecting_station {
                if !*station_intersection_handled {
                    if intersecting_station.station_id == last_line_node.start_station_id {
                        println!("i should detach the line from the station here");

                        if path.len() > 1 {
                            commands.entity(last_line_node.line_entity).despawn();
                            path.pop();
                            if let Some(last_node) = path.last_mut() {
                                last_node.end_station_id = None;
                            }
                        }
                    } else {
                        if path.len() > 2
                            && intersecting_station.station_id
                                == path.first().unwrap().start_station_id
                        {
                            // TODO: loop the path
                            println!("todo: loop the path");

                            *station_intersection_handled = true;
                            return;
                        }

                        // Check if we're trying to create an impossible loop.
                        if path
                            .iter()
                            .any(|node| node.start_station_id == intersecting_station.station_id)
                        {
                            return;
                        };

                        if let Some(last_node) = path.last_mut() {
                            last_node.end_station_id = Some(intersecting_station.station_id);
                        }

                        let station = &metro.stations[intersecting_station.station_id];

                        lsat_line_2d_data.end = station.position;
                        last_line_dragging.end_station_id = Some(intersecting_station.station_id);

                        let new_line_entity = commands
                            .spawn((
                                MetroLineVisualBundle::new(
                                    station.position,
                                    drag_position,
                                    LINE_COLORS[*line_id].into(),
                                ),
                                MetroLine {
                                    start_station_id: intersecting_station.station_id,
                                    end_station_id: None,
                                    line_id: *line_id,
                                },
                            ))
                            .id();

                        path.push(PathNode {
                            start_station_id: intersecting_station.station_id,
                            end_station_id: None,
                            line_entity: new_line_entity,
                        });
                    }

                    *station_intersection_handled = true;
                }
            } else {
                *station_intersection_handled = false;
            };
        }
        LineDragState::Extend {
            path: connections,
            line_id,
        } => todo!(),
        LineDragState::Edit {
            line_id,
            path: connections,
            stations,
        } => todo!(),
    };
}

fn on_drag_end(
    trigger: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    drag_data_q: Query<(Entity, &mut MetroLine, &mut MetroLineVisual)>,

    camera_transform_q: Single<(&Camera, &GlobalTransform)>,
    mut drag_state: ResMut<LineDragState>,
    // mut metro: ResMut<Metro>,
    metro_resources: Res<MetroResources>,

    mut line_path_changed: EventWriter<LinePathChanged>,
) {
    match &mut *drag_state {
        LineDragState::None => {}
        LineDragState::New { path, line_id } => {
            let Some(last_line) = path.last() else {
                return;
            };

            if let Ok((entity, metro_line, _)) = drag_data_q.get(last_line.line_entity) {
                if metro_line.end_station_id.is_none() {
                    commands.entity(entity).despawn();
                }
                path.pop();
            }

            if path.is_empty() {
                return;
            }

            let mut new_path = Vec::with_capacity(path.len() + 1);
            new_path.push(path[0].start_station_id);

            for line in path {
                let Some(end_station_id) = line.end_station_id else {
                    panic!("one of the lines in the path doesn't contain `end_station_id`");
                };

                new_path.push(end_station_id);
            }

            println!("sending linepathchanged event");
            line_path_changed.write(LinePathChanged {
                line_id: *line_id,
                new_path,
            });
        }
        LineDragState::Extend { path, line_id } => todo!(),
        LineDragState::Edit {
            line_id,
            path,
            stations,
        } => todo!(),
    }
}

// impl LineHandle {
//     pub fn spawn_for_station(
//         commands: &mut Commands,
//         meshes: &mut ResMut<Assets<Mesh>>,
//         materials: &mut ResMut<Assets<ColorMaterial>>,
//         station: &Station,
//         station_id: StationId,
//     ) {
//         let station_position = station.position;
//
//         commands
//             .spawn((
//                 LineHandle {
//                     position: station.position,
//                     station_id,
//                     line_id: None,
//                 },
//                 Mesh2d(meshes.add(Circle::new(4.))),
//                 MeshMaterial2d(materials.add(ColorMaterial::from_color(PURPLE))),
//                 Transform::from_translation(station_position.extend(3.)),
//             ))
//             .observe(
//                 move |trigger: Trigger<Pointer<DragStart>>,
//                       mut commands: Commands,
//                       metro: Res<Metro>,
//                       metro_resources: Res<MetroResources>| {
//                     println!("drag_start triggered. station_id: {station_id}");
//                     let active_lines = metro.get_active_lines();
//                     let new_line_id = (0..metro_resources.available_lines)
//                         .find(|line_id| !active_lines.contains(line_id));
//
//                     if let Some(new_line_id) = new_line_id {
//
//
//                         println!("new line connection. line_id: {new_line_id} \t station_id: {station_id}");
//
//                         commands.spawn((
//                             Line2dBundle::new(
//                                 station_position,
//                                 trigger.pointer_location.position,
//                                 5.,
//                                 LINE_COLORS[new_line_id].into(),
//                             ),
//                             LineDragData {
//                                 start_position: station_position,
//                                 start_station_id: station_id,
//                                 line_id: new_line_id,
//                             },
//                         ));
//                     }
//                 },
//             )
//             .observe(
//                 move |trigger: Trigger<Pointer<Drag>>,
//                  mut drag_data_q: Query<(&LineDragData, &mut Line2dData)>,
//                  camera_transform_q: Single<(&Camera, &GlobalTransform)>| {
//                     println!("drag triggered for station {station_id}");
//                     if let Ok((line_drag_data, mut line_2d_data)) = drag_data_q.single_mut() {
//                         println!("single mut OK");
//                         let (camera, transform) = *camera_transform_q;
//
//                         let pointer_location = trigger.pointer_location.position;
//
//                         let world_location =
//                             camera.viewport_to_world_2d(transform, pointer_location);
//
//                         match world_location {
//                             Ok(world_location) => {
//                                 line_2d_data.end = world_location;
//                             }
//                             Err(err) => println!("something's wrong: {}", err),
//                         }
//                     }
//                 },
//             )
//             .observe(|trigger: Trigger<Pointer<DragEnd>>, mut drag_data_q: Query<(&LineDragData, &mut Line2dData)>| {
//
//
//                 });
//     }
// }
