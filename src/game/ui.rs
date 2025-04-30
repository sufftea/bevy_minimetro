use std::time::Duration;

use bevy::{
    ecs::spawn::SpawnWith, log::tracing_subscriber::fmt::format, math::VectorSpace, prelude::*,
};
use bevy_tweening::{AnimationSystem, Animator, Lens, Tween, component_animator_system};

use crate::{AppState, style};

use super::{
    ActiveLinesChanged, GameComponent,
    metro::{LINE_COLORS, Metro, MetroResources},
};

const LINE_INDICATOR_INACTIVE_SIZE: f32 = 20.;
const LINE_INDICATOR_ACTIVE_SIZE: f32 = 50.;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), setup_ui)
        .add_systems(OnEnter(AppState::Game), build_line_indicators)
        .add_systems(
            Update,
            build_line_indicators
                .run_if(in_state(AppState::Game))
                .run_if(on_event::<ActiveLinesChanged>),
        )
        .add_systems(
            Update,
            component_animator_system::<Node>
                .in_set(AnimationSystem::AnimationUpdate)
                .run_if(in_state(AppState::Game)),
        );
}

#[derive(Component, Clone)]
struct LineIndicatorsState {
    line_states: Vec<LineIndicatorState>,
}

#[derive(Clone, Copy)]
enum LineIndicatorState {
    Selected,
    Active,
    Inactive,
    Unavailable,
}

fn setup_ui(mut commands: Commands, ) {
    commands.spawn((
        Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            justify_self: JustifySelf::Center,
            align_self: AlignSelf::End,
            ..default()
        },
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(style::SECONDARY.into()),
                    BorderRadius::MAX,
                    BackgroundColor(style::CONTAINER_SECONDARY.into()),
                    children![(
                        Text::new("Test"),
                        TextColor(style::ON_SECONDARY.into()),
                        TextShadow::default(),
                    )],
                ))
                .observe(
                    |_: Trigger<Pointer<Click>>,
                     mut events: EventWriter<ActiveLinesChanged>,
                     mut metro_resources: ResMut<MetroResources>| {
                        println!("button clicked. metro lines: {}", metro_resources.lines);
                        metro_resources.lines += 1;
                        events.write(ActiveLinesChanged);
                    },
                );
        })),
    ));
}

fn build_line_indicators(
    mut commands: Commands,
    metro_resources: Res<MetroResources>,
    metro: Res<Metro>,
    old_tree_q: Query<(Entity, &LineIndicatorsState)>,
) {
    let old_state = if let Some((old_entity, old_state)) = old_tree_q.iter().next() {
        commands.entity(old_entity).despawn();
        old_state.clone()
    } else {
        LineIndicatorsState {
            line_states: vec![LineIndicatorState::Unavailable; metro_resources.max_lines],
        }
    };

    let active_lines = metro.get_active_lines();
    let new_state = match metro_resources.lines {
        5 => LineIndicatorsState {
            line_states: vec![
                LineIndicatorState::Active,
                LineIndicatorState::Active,
                LineIndicatorState::Active,
                LineIndicatorState::Inactive,
                LineIndicatorState::Unavailable,
                LineIndicatorState::Unavailable,
            ],
        },
        4 => LineIndicatorsState {
            line_states: vec![
                LineIndicatorState::Active,
                LineIndicatorState::Active,
                LineIndicatorState::Inactive,
                LineIndicatorState::Inactive,
                LineIndicatorState::Unavailable,
                LineIndicatorState::Unavailable,
            ],
        },
        _ => LineIndicatorsState {
            line_states: vec![
                LineIndicatorState::Active,
                LineIndicatorState::Inactive,
                LineIndicatorState::Inactive,
                LineIndicatorState::Unavailable,
                LineIndicatorState::Unavailable,
                LineIndicatorState::Unavailable,
            ],
        },
    };
    // let new_state = LineIndicatorsState {
    //     line_states: (0..metro_resources.max_lines)
    //         .map(|i| {
    //             if i < metro_resources.lines {
    //                 if active_lines.contains(&i) {
    //                     LineIndicatorState::Active
    //                 } else {
    //                     LineIndicatorState::Inactive
    //                 }
    //             } else {
    //                 LineIndicatorState::Unavailable
    //             }
    //         })
    //         .collect(),
    // };

    commands
        .spawn((
            GameComponent,
            new_state.clone(),
            Node {
                justify_self: JustifySelf::End,
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                width: Val::Px(100.),
                // width: Val::Auto,
                height: Val::Auto,
                ..default()
            },
        ))
        .with_children(|parent| {
            for (i, (new_state, old_state)) in
                std::iter::zip(new_state.line_states, old_state.line_states).enumerate()
            {
                parent
                    .spawn((Node {
                        justify_self: JustifySelf::Center,
                        align_self: AlignSelf::Center,
                        align_content: AlignContent::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Auto,
                        height: Val::Px(LINE_INDICATOR_ACTIVE_SIZE + 10.),
                        ..default()
                    },))
                    .with_children(|parent| {
                        match new_state {
                            LineIndicatorState::Selected => todo!(),

                            LineIndicatorState::Active => {
                                let tween = Tween::new(
                                    EaseFunction::BounceOut,
                                    Duration::from_millis(400),
                                    NodeSizeLens {
                                        start: match old_state {
                                            LineIndicatorState::Active => {
                                                LINE_INDICATOR_ACTIVE_SIZE
                                            }
                                            _ => LINE_INDICATOR_INACTIVE_SIZE,
                                        },
                                        end: LINE_INDICATOR_ACTIVE_SIZE,
                                    },
                                );

                                parent.spawn((
                                    Node {
                                        justify_self: JustifySelf::Center,
                                        align_self: AlignSelf::Center,
                                        width: Val::Px(LINE_INDICATOR_ACTIVE_SIZE),
                                        height: Val::Px(LINE_INDICATOR_ACTIVE_SIZE),
                                        border: UiRect::all(Val::Px(5.)),
                                        ..default()
                                    },
                                    Animator::new(tween),
                                    BackgroundColor(LINE_COLORS[i].into()),
                                    BorderColor(style::ON_BACKGROUND.into()),
                                    BorderRadius::all(Val::Px(LINE_INDICATOR_ACTIVE_SIZE / 2.)),
                                ))
                            }
                            LineIndicatorState::Inactive => {
                                let tween = Tween::new(
                                    EaseFunction::BounceOut,
                                    Duration::from_millis(400),
                                    NodeSizeLens {
                                        start: match old_state {
                                            LineIndicatorState::Active => {
                                                LINE_INDICATOR_ACTIVE_SIZE
                                            }
                                            _ => LINE_INDICATOR_INACTIVE_SIZE,
                                        },
                                        end: LINE_INDICATOR_INACTIVE_SIZE,
                                    },
                                );
                                parent.spawn((
                                    Node {
                                        justify_self: JustifySelf::Center,
                                        align_self: AlignSelf::Center,
                                        width: Val::Px(LINE_INDICATOR_INACTIVE_SIZE),
                                        height: Val::Px(LINE_INDICATOR_INACTIVE_SIZE),
                                        // border: UiRect::all(Val::Px(3.)),
                                        ..default()
                                    },
                                    Animator::new(tween),
                                    BackgroundColor(LINE_COLORS[i].into()),
                                    // BorderColor(style::ON_BACKGROUND.into()),
                                    BorderRadius::all(Val::Px(LINE_INDICATOR_ACTIVE_SIZE) / 2.),
                                ))
                            }
                            LineIndicatorState::Unavailable => parent.spawn((
                                Node {
                                    justify_self: JustifySelf::Center,
                                    align_self: AlignSelf::Center,
                                    width: Val::Px(LINE_INDICATOR_INACTIVE_SIZE),
                                    height: Val::Px(LINE_INDICATOR_INACTIVE_SIZE),
                                    ..default()
                                },
                                BackgroundColor(style::ON_BACKGROUND.into()),
                                BorderRadius::all(Val::Px(LINE_INDICATOR_INACTIVE_SIZE) / 2.),
                            )),
                        };
                    });
            }
        });
}

struct NodeSizeLens {
    start: f32,
    end: f32,
}

impl Lens<Node> for NodeSizeLens {
    fn lerp(&mut self, target: &mut dyn bevy_tweening::Targetable<Node>, ratio: f32) {
        target.width = Val::Px(FloatExt::lerp(self.start, self.end, ratio));
        target.height = Val::Px(FloatExt::lerp(self.start, self.end, ratio));
    }
}

// fn on_line_state_change(
//     mut commands: Commands,
//     changes: Query<(&LineIndicatorSlot, &Node, Entity), Changed<LineIndicatorSlot>>,
// ) {
//     println!("on line state change");
//     for (slot_data, node, entity) in changes {
//         match slot_data.state {
//             LineIndicatorState::Selected => todo!(),
//             LineIndicatorState::Active => {
//                 println!("adding the animator");
//                 let curr_size = match node.width {
//                     Val::Px(width) => width,
//                     _ => LINE_INDICATOR_INACTIVE_SIZE,
//                 };
//
//                 let tween = Tween::new(
//                     EaseFunction::BounceOut,
//                     Duration::from_secs(1),
//                     NodeSizeLens {
//                         start: curr_size,
//                         end: LINE_INDICATOR_ACTIVE_SIZE,
//                     },
//                 );
//
//                 commands.entity(entity).insert(Animator::new(tween));
//             }
//             LineIndicatorState::Inactive => {}
//             LineIndicatorState::Unavailable => {}
//         };
//     }
// }

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
