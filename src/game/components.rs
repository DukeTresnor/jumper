// game/components.rs
// add a gravity component

use bevy::prelude::*;

#[derive(Component)]
pub struct Gravity {
    pub gravity_constant: f32,
}

#[derive(Component)]
pub struct Floor {}


#[derive(Component)]
pub struct BoundingBox {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub z_min: f32,
    pub z_max: f32,
}