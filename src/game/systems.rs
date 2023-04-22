// game/systems.rs
// need systems to toggle between paused and running simulation states
// also need pause_simulation and resume_simulation systems that should only run when
//   entering and exiting AppState::Game, respectively

use::bevy::prelude::*;
use bevy::render::view::window;
use bevy::transform::commands;
use bevy::window::PrimaryWindow;
//use bevy::window::PrimaryWindow;

use crate::game::SimulationState;
use crate::game::components::*;
use crate::game::resources::*;

use crate::game::TILE_SIZE;

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


// Applies "gravity" to any entity that has the gravity component
//   gravity is a component that exists in the game folder
// We also need to use a time resource
pub fn apply_gravity(
    //mut entity_query: Query<Entity, With<Gravity>>,
    mut transform_gravity_query: Query<(&mut Transform, &Gravity)>,
    time: Res<Time>,
) {
    for (mut entity_transform, entity_gravity) in transform_gravity_query.iter_mut() {
        // set movement to go down according to the entity's gravity constant component value
        let gravity_direction = Vec3::new(0.0, -entity_gravity.gravity_constant, 0.0);
        entity_transform.translation += gravity_direction * time.delta_seconds();
        //println!("Applying gravity");
    }
}







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