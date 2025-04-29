use std::{any::TypeId, fmt::Debug};

use bevy::{
    animation::{AnimationEvaluationError, AnimationTargetId, animated_field},
    ecs::component::ComponentInfo,
    math::VectorSpace,
    prelude::*,
    reflect::hash_error,
};

use crate::{AppState, style};

use super::{
    ActiveLinesChanged, GameComponent,
    metro::{LINE_COLORS, LineId, Metro, MetroResources},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), setup_ui);
    // app.add_systems(OnEnter(AppState::Game), (setup_ui, debug_shit).chain());
}

#[derive(Component)]
struct LineSlot {
    line_id: LineId,
}

fn setup_ui(
    mut commands: Commands,
    metro_resources: Res<MetroResources>,
    metro: Res<Metro>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            GameComponent,
            Node {
                justify_self: JustifySelf::End,
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                width: Val::Px(100.),
                height: Val::Auto,
                ..default()
            },
        ))
        .with_children(|parent| {
            let active_lines = metro.get_active_lines();

            for i in 0..metro_resources.max_lines {
                parent
                    .spawn((
                        LineSlot { line_id: i },
                        Node {
                            justify_self: JustifySelf::Center,
                            align_self: AlignSelf::Center,
                            align_content: AlignContent::Center,
                            justify_content: JustifyContent::Center,
                            width: Val::Px(60.),
                            height: Val::Px(60.),
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Node {
                                width: Val::Px(20.),
                                height: Val::Px(20.),
                                ..default()
                            },
                            Mesh2d(meshes.add(Circle::new(20.))),
                            MeshMaterial2d(
                                materials.add(ColorMaterial::from_color(LINE_COLORS[i])),
                            ),
                            Transform::from_translation(Vec3::ZERO),
                            // Transform::from_translation(station.position.extend(1.0)),
                        ));

                        // if i < metro_resources.lines {
                        //     if active_lines.contains(&i) {
                        //         parent.spawn((
                        //             LineIndicator,
                        //             Node {
                        //                 justify_self: JustifySelf::Center,
                        //                 align_self: AlignSelf::Center,
                        //                 width: Val::Px(60.),
                        //                 height: Val::Px(60.),
                        //                 border: UiRect::all(Val::Px(5.)),
                        //                 ..default()
                        //             },
                        //             BackgroundColor(LINE_COLORS[i].into()),
                        //             BorderColor(style::ON_BACKGROUND.into()),
                        //             BorderRadius::all(Val::Px(30.)),
                        //         ));
                        //     } else {
                        //         parent.spawn((
                        //             LineIndicator,
                        //             Node {
                        //                 justify_self: JustifySelf::Center,
                        //                 align_self: AlignSelf::Center,
                        //                 width: Val::Px(20.),
                        //                 height: Val::Px(20.),
                        //                 border: UiRect::all(Val::Px(3.)),
                        //                 ..default()
                        //             },
                        //             BackgroundColor(LINE_COLORS[i].into()),
                        //             // BorderColor(style::ON_BACKGROUND.into()),
                        //             BorderRadius::all(Val::Px(30.)),
                        //         ));
                        //     };
                        // } else {
                        //     parent.spawn((
                        //         Node {
                        //             justify_self: JustifySelf::Center,
                        //             align_self: AlignSelf::Center,
                        //             width: Val::Px(20.),
                        //             height: Val::Px(20.),
                        //             border: UiRect::all(Val::Px(3.)),
                        //             ..default()
                        //         },
                        //         BackgroundColor(style::ON_BACKGROUND.into()),
                        //         BorderRadius::all(Val::Px(10.)),
                        //     ));
                        // }
                    });
            }
        });
}

// fn debug_shit(world: &World, line_slot: Query<(Entity, &Children), With<LineSlot>>) {
//     if let Some((entity, children)) = line_slot.iter().next() {
//         let names = world.inspect_entity(entity).map(|iter| {
//             iter.for_each(|item| {
//                 println!("{:?}", item);
//             })
//         });
//         //
//         println!("{:#?}", names);
//
//         // println!("=====  children: ======");
//
//         // for child in children {
//         //     let names = world
//         //         .inspect_entity(entity)
//         //         .map(|iter| iter.map(|item| item.fmt()).collect::<Vec<_>>());
//         //     //
//         //     println!("{:#?}", names);
//         // }
//         // world.inspect_entity(entity).unwrap().find(|item| item.)
//     } else {
//         println!("didn't find the entity")
//     }
// }
