use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_line_position);
}

#[derive(Component)]
pub struct Line2dData {
    pub start: Vec2,
    pub end: Vec2,
    pub color: Color,
    pub width: f32,
}

#[derive(Bundle)]
pub struct Line2dBundle {
    pub data: Line2dData,
    pub transform: Transform,
    pub sprite: Sprite,
}

impl Line2dBundle {
    pub fn new(start: Vec2, end: Vec2, width: f32, color: Color) -> Self {
        let direction = end - start;
        let length = direction.length();
        let angle = direction.y.atan2(direction.x);

        Line2dBundle {
            data: Line2dData {
                color,
                end,
                start,
                width,
            },
            transform: Transform {
                translation: ((start + end) / 2.0).extend(0.0),
                rotation: Quat::from_rotation_z(angle),
                scale: Vec3::new(length, width, 1.0),
                // ..Default::default()
            },
            sprite: Sprite {
                color,
                ..Default::default()
            },
        }
    }
}

fn update_line_position(
    lines: Query<(&Line2dData, &mut Transform, &mut Sprite), Changed<Line2dData>>,
) {
    for (data, mut transform, mut sprite) in lines {
        let direction = data.end - data.start;
        let length = direction.length();
        let angle = direction.y.atan2(direction.x);

        transform.translation = ((data.start + data.end) / 2.0).extend(0.0);
        transform.rotation = Quat::from_rotation_z(angle);
        transform.scale = Vec3::new(length, data.width, 1.0);

        sprite.color = data.color;
    }
}
