// game/components.rs
// add a gravity component

use bevy::prelude::*;


// -- Animation components -- //

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

// Weird formatting, must be b/c of Deref and DerefMut
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

// -- Animation components -- //



#[derive(Component)]
pub struct Gravity {
    pub gravity_constant: f32,
}

#[derive(Component)]
pub struct Floor {}



// -- collision components -- //
#[derive(Component)]
pub struct BoundingBox {
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub z_min: f32,
    pub z_max: f32,
}


#[derive(Component)]
pub struct EntitySizeCollision {
    pub horizontal_entity_size: f32,
    pub vertical_entity_size: f32,
}
// -- collision components -- //