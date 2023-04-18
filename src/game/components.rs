// game/components.rs
// add a gravity component

use bevy::prelude::*;

#[derive(Component)]
pub struct Gravity {
    pub gravity_constant: f32,
}