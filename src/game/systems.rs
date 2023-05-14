// game/systems.rs
// need systems to toggle between paused and running simulation states
// also need pause_simulation and resume_simulation systems that should only run when
//   entering and exiting AppState::Game, respectively

use::bevy::prelude::*;
use bevy::window::PrimaryWindow;
//use bevy::window::PrimaryWindow;

use crate::game::SimulationState;
use crate::game::components::*;
//use crate::game::resources::*;

use crate::game::TILE_SIZE;

use super::player::components::JumpVelocity;

pub fn toggle_simulation_state(
    // needs access to keyboard input
    keyboard_input: Res<Input<KeyCode>>,
    // needs to have access to the current state, and needs to transition to another state
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        if simulation_state.0 == SimulationState::Paused {
            // Enter Running with set()
            next_simulation_state.set(SimulationState::Running);
            println!("Simulation Running");
        }
        if simulation_state.0 == SimulationState::Running {
            // Enter Paused with set()
            next_simulation_state.set(SimulationState::Paused);
            println!("Simulation Paused");
        }
    }
}

// If pause_simulation is added to the run schedule, we need to shift our Simulation state to
//   paused. Therefore, this system needs access to the NextState resource
// the next state needs to be set with set()
pub fn pause_simulation(
    mut next_simulation_state: ResMut<NextState<SimulationState>>
) {
    next_simulation_state.set(SimulationState::Paused);
    println!("I paused the simulation")
}

pub fn resume_simulation(
    mut next_simulation_state: ResMut<NextState<SimulationState>>
) {
    next_simulation_state.set(SimulationState::Running);
    println!("I resumed the simulation")
}


// ------- //

pub fn apply_gravity_and_velocity(
    mut transform_gravity_velocity_query: Query<(&mut Transform, &Gravity, &mut JumpVelocity)>,
    time: Res<Time>,
) {
    for (mut entity_transform, entity_gravity, mut entity_velocity) in transform_gravity_velocity_query.iter_mut() {
        // refine code, edit out later once you solidify on the constants you want
        let temp_vel_mod = 10.0;
        
        let gravity_direction = Vec3::new(0.0, -entity_gravity.gravity_constant, 0.0);
        let velocity_direction = Vec3::new(entity_velocity.horizontal_velocity, entity_velocity.vertical_velocity, 0.0);
        entity_transform.translation += temp_vel_mod * velocity_direction * time.delta_seconds() + gravity_direction * time.delta_seconds();
        //println!("Applying gravity");

        // reduce velocity over time
        if entity_velocity.vertical_velocity > 0.0 {
            entity_velocity.vertical_velocity -= entity_gravity.gravity_constant;
        } else {
            entity_velocity.vertical_velocity = 0.0;
        }

        //println!("Printing vertical velocity: {}", entity_velocity.vertical_velocity);
    }
}

// ------- //




// Floor Systems //

pub fn spawn_floor(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let number_of_tiles_in_floor = (window.width() / TILE_SIZE) as i32;

    for (i, _) in (0..number_of_tiles_in_floor+1).enumerate() {
        let i_f32 = i as f32;
        // spawn a floor tile
        commands.spawn(
    (
                SpriteBundle {
                    transform: Transform::from_xyz(i_f32 * TILE_SIZE, 0.0, 0.0),
                    texture: asset_server.load("sprites/tile_0000.png"),
                    ..default()
                },
                Floor {},
                EntitySizeCollision {
                    horizontal_entity_size: TILE_SIZE,
                    vertical_entity_size: TILE_SIZE,
                }
            )
        );
        println!("Spawning floor");
    }

}

pub fn despawn_floor(
    mut commands: Commands,
    floor_query: Query<Entity, With<Floor>>,
) {
    for floor_entity in floor_query.iter() {
        commands.entity(floor_entity).despawn();
    }
}

// Floor Systems //


// Animate sprite //

// Needs a time resource
// Needs a mutable query to AnimationIndices to get the entity's spritesheet's first and last desired sprite,
//   to a mutable AnimationTimer 
// this might need to also return the entity to a specific state?
//   or just handle it inside a player system
pub fn animate_sprite(
    time: Res<Time>,
    mut animation_query: Query<(
        &AnimationIndices, 
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>
) {
    for (animation_indices, mut timer, mut sprite_sheet) in animation_query.iter_mut() {
        timer.tick(time.delta());
        // If the timer is finished, force the sprite sheet to be at
        //   the next sprite in the sheet. If you are at the last sprite in the
        //   sheet, go to the first sprite so it loops
        if timer.just_finished() {
            sprite_sheet.index = if sprite_sheet.index == animation_indices.last {
                animation_indices.first
            } else {
                sprite_sheet.index + 1
            }
        }
    }
}


// Animate sprite //