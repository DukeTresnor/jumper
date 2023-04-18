// game/systems.rs
// need systems to toggle between paused and running simulation states
// also need pause_simulation and resume_simulation systems that should only run when
//   entering and exiting AppState::Game, respectively

use::bevy::prelude::*;

use crate::game::SimulationState;
use crate::game::components::*;

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
        println!("Applying gravity");
    }
}