// game / resources.rs
use bevy::prelude::*;
use std::{collections::HashMap, default};

pub const INPUT_BUFFER_CLEAR_TIME: f32 = 1.0;


// Not in use atm
#[derive(Resource)]
pub struct CollisionHashMap {
    pub hash_map: HashMap<Entity, Entity>,
}

impl Default for CollisionHashMap {
    fn default() -> CollisionHashMap {
        CollisionHashMap {
            hash_map: HashMap::new()
        }        
    }
}



#[derive(Resource)]
pub struct InputBufferTimer {
    pub timer: Timer,
}

impl Default for InputBufferTimer {
    fn default() -> InputBufferTimer {
        InputBufferTimer {
            timer: Timer::from_seconds(INPUT_BUFFER_CLEAR_TIME, TimerMode::Repeating)
        }
    }
}