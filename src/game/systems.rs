// game/systems.rs
// need systems to toggle between paused and running simulation states
// also need pause_simulation and resume_simulation systems that should only run when
//   entering and exiting AppState::Game, respectively

use::bevy::prelude::*;
use bevy::render::camera;
use bevy::window::PrimaryWindow;
use bevy::utils::Duration;
//use bevy::window::PrimaryWindow;


use nalgebra::geometry::Translation;

use crate::game::SimulationState;
use crate::game::components::*;
//use crate::game::resources::*;

use crate::game::TILE_SIZE;
use crate::game::OVERALL_FRAME_RATE;

use crate::components::*;

use super::player::components::PlayerNumber;
use super::player::components::JumpVelocity;
use super::player::components::Player;


pub const TEMP_VEL_MOD: f32 = 10.0;
pub const CAMERA_LERP_FACTOR: f32 = 1.0;
pub const CAMERA_ORTHOGRAPHIC_PROJECTION_FACTOR: f32 = 200.0;
pub const CENTERED_CAMERA_FACTOR: f32 = 0.5;

pub fn toggle_simulation_state(
    // needs access to keyboard input
    keyboard_input: Res<Input<KeyCode>>,
    // needs to have access to the current state, and needs to transition to another state
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        if simulation_state.get() == &SimulationState::Paused {
            // Enter Running with set()
            next_simulation_state.set(SimulationState::Running);
            println!("Simulation Running");
        }
        if simulation_state.get() == &SimulationState::Running {
            // Enter Paused with set()
            next_simulation_state.set(SimulationState::Paused);
            println!("Simulation Paused");
        }
    }
}

// Systems that deal with the debugger can go into the debugger_systems module
// Right now this transition system is in here because it's similar to other systems that
//   deal with state changing within the game
// not sure if this is correct
pub fn transition_to_debugger_state(
    //
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    //
    if keyboard_input.just_pressed(KeyCode::Q) {
        if simulation_state.get() != &SimulationState::Debugger {
            next_simulation_state.set(SimulationState::Debugger);
            println!("Entering Debugger");
        }
    }
    if keyboard_input.just_pressed(KeyCode::Q) {
        if simulation_state.get() == &SimulationState::Debugger {
            next_simulation_state.set(SimulationState::Running);
            println!("Exiting Debugger and entering Running");
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
        
        let gravity_direction = Vec3::new(0.0, -entity_gravity.gravity_constant, 0.0);
        let velocity_direction = Vec3::new(entity_velocity.horizontal_velocity, entity_velocity.vertical_velocity, 0.0);
        entity_transform.translation += TEMP_VEL_MOD * velocity_direction * time.delta_seconds() + gravity_direction * time.delta_seconds();
        


        //println!("Applying gravity");

        // reduce vertical velocity over time
        if entity_velocity.vertical_velocity > 0.0 {
            entity_velocity.vertical_velocity -= entity_gravity.gravity_constant;
        } else {
            entity_velocity.vertical_velocity = 0.0;
        }

        if entity_velocity.horizontal_velocity > 0.0 {
            entity_velocity.horizontal_velocity -= entity_velocity.horizontal_velocity * 0.10;
        }

        if entity_velocity.horizontal_velocity < 0.0 {
            entity_velocity.horizontal_velocity += entity_velocity.horizontal_velocity * -0.10;
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
        &PlayerNumber,
    )>
) {
    let mut dummy_delta: f64 = time.delta_seconds_f64();
    dummy_delta *= 1.0;
    let dummy_duration: Duration = Duration::from_secs_f64(dummy_delta);
    //println!("dummy_delta: {}", dummy_delta);
    for (animation_indices, mut timer, mut sprite_sheet, player_number) in animation_query.iter_mut() {
        timer.tick(time.delta());

        // Attempts to fix ticks issue
        //timer.tick(Duration::from_millis(90));
        //timer.tick(dummy_duration);
        //timer.tick(Duration::from_secs_f64(2.0 * 0.016789568));

        //println!("delta: {:?}, delta_seconds: {}", time.delta(), time.delta_seconds());


        // If the timer is finished, force the sprite sheet to be at
        //   the next sprite in the sheet. If you are at the last sprite in the
        //   sheet, go to the first sprite so it loops

        //println!("{:?}", time.delta());

        if timer.just_finished() {
            sprite_sheet.index = if sprite_sheet.index == animation_indices.last {
                animation_indices.first
            } else {

                //println!("Sprite Sheet index player {}: {}", player_number.player_number, sprite_sheet.index);
                sprite_sheet.index + 1
            }
        }
    }
}


// Animate sprite //


// Camera Zooming Systems //
// Need to fund adjustment that works for you
pub fn camera_zoom(
    //
    player_query: Query<&Transform, (With<Player>, Without<MyGameCamera>)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), (Without<Player>, With<MyGameCamera>)>,
) {
    let window = window_query.get_single().unwrap();
    let window_width = window.width();

    // Following strat for dealing with multiple players seems good??
    let mut player_positions = Vec::new();

    // bundle the two player positions into player_positions
    for player_transform in &mut player_query.iter() {
        player_positions.push(player_transform.translation);
    }


    // Calculate the midpoint between the two players
    let midpoint = player_positions.iter().fold(Vec3::ZERO, |acc, &pos| acc + pos) / player_positions.len() as f32;


    for (mut camera_transform, mut orthographic_projection) in camera_query.iter_mut() {
        
        //let midpoint = (player_transform.translation + camera_transform.translation) / 2.0;


        // Calculate the maximum distance from the midpoint to any player
        // this max distance should just be from the midpoint to the wall shouldn't it?
        let max_distance = player_positions.iter().map(|&pos| pos.distance(midpoint)).fold(0.0, f32::max);

        
        // Adjust the camera's orthographic projection component based on the distance between itself and the given player
        //orthographic_projection.scale = 1.0 / (max_distance / 100.0 + 1.0);
        orthographic_projection.scale = max_distance / CAMERA_ORTHOGRAPHIC_PROJECTION_FACTOR + CENTERED_CAMERA_FACTOR;



        
        // Update the camera position
        //camera_transform.translation.x = midpoint.x;
        //camera_transform.translation.y = midpoint.y;

        let lerp_factor = CAMERA_LERP_FACTOR;
        let interpolated_position = camera_transform.translation.lerp(*&midpoint, lerp_factor);
        camera_transform.translation = interpolated_position;

    }

}




// Camera Zooming Systems //




