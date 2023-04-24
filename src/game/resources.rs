// game / resources.rs
use bevy::prelude::*;
use std::collections::HashMap;


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
