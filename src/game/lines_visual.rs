use bevy::prelude::*;

const LINE_WIDTH: f32 = 2.;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, update_line_position);
}

#[derive(Component)]
pub struct MetroLineVisual {
    pub start: Vec2,
    pub end: Vec2,
    pub color: Color,
}

#[derive(Bundle)]
pub struct MetroLineVisualBundle {
    pub data: MetroLineVisual,
    pub transform: Transform,
    pub sprite: Sprite,
}

impl MetroLineVisualBundle {
    pub fn new(start: Vec2, end: Vec2, color: Color) -> Self {
        let direction = end - start;
        let length = direction.length();
        let angle = direction.y.atan2(direction.x);

        MetroLineVisualBundle {
            data: MetroLineVisual { color, end, start },
            transform: Transform {
                translation: ((start + end) / 2.0).extend(0.0),
                rotation: Quat::from_rotation_z(angle),
                scale: Vec3::new(length, LINE_WIDTH, 1.0),
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
    lines: Query<(&MetroLineVisual, &mut Transform, &mut Sprite), Changed<MetroLineVisual>>,
) {
    for (data, mut transform, mut sprite) in lines {
        let direction = data.end - data.start;
        let length = direction.length();
        let angle = direction.y.atan2(direction.x);

        transform.translation = ((data.start + data.end) / 2.0).extend(0.0);
        transform.rotation = Quat::from_rotation_z(angle);
        transform.scale = Vec3::new(length, LINE_WIDTH, 1.0);

        sprite.color = data.color;
    }
}
