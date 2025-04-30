use std::f32::consts::SQRT_2;

use bevy::{
    math::Vec2,
    prelude::{Circle, Rectangle, Triangle2d},
};

use super::metro::StationKind;

pub struct StationMeshBuilder {
    radius: f32,
}

pub const STATION_MESHES: StationMeshBuilder = StationMeshBuilder { radius: 4. };



impl StationMeshBuilder {
    pub fn circle(&self) -> Circle {
        Circle::new(self.radius)
    }

    pub fn triangle(&self) -> Triangle2d {
        let angle1 = 90.0_f32.to_radians();
        let angle2 = 210.0_f32.to_radians();
        let angle3 = 330.0_f32.to_radians();

        Triangle2d::new(
            Vec2::new(self.radius * angle1.cos(), self.radius * angle1.sin()),
            Vec2::new(self.radius * angle2.cos(), self.radius * angle2.sin()),
            Vec2::new(self.radius * angle3.cos(), self.radius * angle3.sin()),
        )
    }

    pub fn square(&self) -> Rectangle {
        Rectangle::from_length(self.radius * SQRT_2)
    }
}
