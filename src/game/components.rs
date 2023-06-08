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
// I don't think I'm using this?
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

#[derive(Component)]
pub struct  EntityInfo {
    // idle
    // walking forwaerd
    // walking backward
    // crouching
    // dashing
    // jumping
    // getting hit
    // knocked down
    // roll
    
    // health value
    // gravity constant
    // size
    // jump force -- vertical and horizontal
    // walk speed
    // dash speed / length

    // light attack -- neutral, crouching, jumping
    // medium attack -- neutral, crouching, jumping
    // heavy attack -- neutral, crouching, jumping

    // special moves
    //  quarter forward
    //  quarter backward
    //  dragon punch forward

    // each of the above needs
    //  spritesheet animation indeces
    //  collision box size
    //  startup, active, recovery, 
    //  advantage if attack
    //  damage if attack
    //  attack level if attack
    //  guard type if attack
    //  proration info

}