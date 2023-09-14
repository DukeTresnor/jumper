// game/player/components.rs

// why is this here?
//use std::thread::local_impl::Key;

use bevy::prelude::*;

// Non component structs //
pub struct CollisionBox {
    pub position: Vec3,
    pub size: Vec2,
    pub lifespan: Timer,
}

// Non component structs //





// Define Player component
#[derive(Component)]
pub struct Player {}


#[derive(Component)]
pub struct JumpVelocity {
    pub horizontal_velocity: f32,
    pub vertical_velocity: f32,
}


// action vector has a keycode and a f32, the f32 is meant to hold time values
#[derive(Component)]
pub struct ActionStateVector {

    pub action_vector: Vec<(KeyCode, f32)>,
}

#[derive(Component)]
pub struct  NegativeEdgeStateVector {
    pub negative_edge_vector: Vec<(KeyCode, f32)>,
}


#[derive(Component)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub light: bool,
    pub medium: bool,
    pub heavy: bool,
    pub unique: bool,
    pub special: bool,
}

#[derive(Component)]
pub struct CollisionInfo {
    hurt: CollisionBox,
    hurt_crouching: CollisionBox,
    throw: CollisionBox,
    overhead_hurt: CollisionBox,
    overhead_hit: CollisionBox,
    light_hurt: CollisionBox,
    light_hit: CollisionBox,
    light_hurt_crouching: CollisionBox,
    light_hit_crouching: CollisionBox,
    light_hurt_jumping: CollisionBox,
    light_hit_jumping: CollisionBox,
    medium_hurt: CollisionBox,
    medium_hit: CollisionBox,
    medium_hurt_crouching: CollisionBox,
    medium_hit_crouching: CollisionBox,
    medium_hurt_jumping: CollisionBox,
    medium_hit_jumping: CollisionBox,
    heavy_hurt: CollisionBox,
    heavy_hit: CollisionBox,
    heavy_hurt_crouching: CollisionBox,
    heavy_hit_crouching: CollisionBox,
    heavy_hurt_jumping: CollisionBox,
    heavy_hit_jumping: CollisionBox,
    unique_hurt: CollisionBox,
    unique_hit: CollisionBox,
    unique_hurt_crouching: CollisionBox,
    unique_hit_crouching: CollisionBox,
    unique_hurt_jumping: CollisionBox,
    unique_hit_jumping: CollisionBox,
    special_hurt: CollisionBox,
    special_hit: CollisionBox,
    special_hurt_crouching: CollisionBox,
    special_hit_crouching: CollisionBox,
    special_hurt_jumping: CollisionBox,
    special_hit_jumping: CollisionBox,


}


#[derive(Component)]
pub struct InputBinding {
    pub up_bind: KeyCode,
    pub down_bind: KeyCode,
    pub left_bind: KeyCode,
    pub right_bind: KeyCode,
    pub light_bind: KeyCode,
    pub medium_bind: KeyCode,
    pub heavy_bind: KeyCode,
    pub unique_bind: KeyCode,
    pub special_bind: KeyCode,
}

#[derive(Component, Default)]
pub struct SpriteSheetIndeces {
    pub idle_first: usize,
    pub idle_last: usize,
    pub crouching_first: usize,
    pub crouching_last: usize,
    pub walk_forward_first: usize,
    pub walk_forward_last: usize,
    pub walk_back_first: usize,
    pub walk_back_last: usize,
    pub dash_forward_first: usize,
    pub dash_forward_last: usize,
    pub dash_back_first: usize,
    pub dash_back_last: usize,
    pub jump_first: usize,
    pub jump_last: usize,
    pub jump_forward_first: usize,
    pub jump_forward_last: usize,
    pub jump_back_first: usize,
    pub jump_back_last: usize,
    pub throw_first: usize,
    pub throw_last: usize,
    pub throw_jumping_first: usize,
    pub throw_jumping_last: usize,
    //pub throw_back_first: usize,
    //pub throw_back_last: usize,
    pub universal_overhead_first: usize,
    pub universal_overhead_last: usize,
    pub light_first: usize,
    pub light_last: usize,
    pub light_crouching_first: usize,
    pub light_crouching_last: usize,
    pub light_jumping_first: usize,
    pub light_jumping_last: usize,
    pub light_far_first: usize,
    pub light_far_last: usize,
    //pub light_back_first: usize,
    //pub light_back_last: usize,
    pub medium_first: usize,
    pub medium_last: usize,
    pub medium_crouching_first: usize,
    pub medium_crouching_last: usize,
    pub medium_jumping_first: usize,
    pub medium_jumping_last: usize,
    //pub medium_forward_first: usize,
    //pub medium_forward_last: usize,
    //pub medium_back_first: usize,
    //pub medium_back_last: usize,
    pub heavy_first: usize,
    pub heavy_last: usize,
    pub heavy_crouching_first: usize,
    pub heavy_crouching_last: usize,
    pub heavy_jumping_first: usize,
    pub heavy_jumping_last: usize,
    //pub heavy_forward_first: usize,
    //pub heavy_forward_last: usize,
    //pub heavy_back_first: usize,
    //pub heavy_back_last: usize,
    pub unique_first: usize,
    pub unique_last: usize,
    pub unique_crouching_first: usize,
    pub unique_crouching_last: usize,
    pub unique_jumping_first: usize,
    pub unique_jumping_last: usize,
    pub unique_forward_first: usize,
    pub unique_forward_last: usize,
    //pub unique_back_first: usize,
    //pub unique_back_last: usize,
    pub special_first: usize,
    pub special_last: usize,
    pub special_crouching_first: usize,
    pub special_crouching_last: usize,
    pub special_jumping_first: usize,
    pub special_jumping_last: usize,
    pub special_forward_first: usize,
    pub special_forward_last: usize,
    pub special_back_first: usize,
    pub special_back_last: usize,
    pub blocking_first: usize,
    pub blocking_last: usize,
    pub blocking_crouching_first: usize,
    pub blocking_crouching_last: usize,
    pub blocking_jumping_first: usize,
    pub blocking_jumping_last: usize,
    pub hit_standing_first: usize,
    pub hit_standing_last: usize,
    pub hit_crouching_first: usize,
    pub hit_crouching_last: usize,
    pub hit_air_first: usize,
    pub hit_air_last: usize,
    pub hit_throw_first: usize,
    pub hit_throw_last: usize,
    pub knockdown_light_first: usize,
    pub knockdown_light_last: usize,
    pub knockdown_hard_first: usize,
    pub knockdown_hard_last: usize,
    pub knockdown_getup_first: usize,
    pub knockdown_getup_last: usize,

}



#[derive(Component)]
pub struct MovementState {
    pub is_grounded: bool,
    pub is_walking: bool,
}

#[derive(Component)]
pub struct AttackState {
    pub is_attacking: bool,
}

#[derive(Component)]
pub struct HitState {
    pub is_in_hitstun: i32,
}

#[derive(Component)]
pub struct BlockState {
    pub is_in_blockstun: i32,
}

#[derive(Component)]
pub struct PlayerNumber {
    pub player_number: i32,
}

