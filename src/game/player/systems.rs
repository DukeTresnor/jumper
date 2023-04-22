// game/player/systems.rs


use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::components::*;
use crate::game::player::components::*;
use crate::game::player::PlayerState;
use crate::game::player::{PLAYER_SPEED, PLAYER_SIZE};
use crate::game::GRAVITY;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    // Access the primary window
    let window = window_query.get_single().unwrap();
    // Create a player component
    // spawn a Player with the Player and Gravity components
    commands.spawn(
        (
            SpriteBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                // the sprite bundle has a texture using the load() method
                texture: asset_server.load("sprites/ball_blue_large.png"),
                ..default()
            },
            Player {},
            //Gravity {
            //    gravity_constant: GRAVITY,
            //},
            EntitySizeCollision {
                horizontal_entity_size: PLAYER_SIZE,
                vertical_entity_size: PLAYER_SIZE,
            },
            JumpVelocity {
                horizontal_velocity: 0.0,
                vertical_velocity: 0.0,
            }
        )
    );
}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    // If player_entity exists because the player_query contains some entity with the Player component,
    //   we want to despawn that player entity with commands
    if let Ok(player_entity) = player_query.get_single() {
        commands.entity(player_entity).despawn()
    }
}

// player_query needs the transform along with player b/c we are trying to move the player
// we again also need the time resource
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    // Get the single mutable thing that exists in player_query, and store it into the transform variable
    // If transform gets a Transform component, continue the if block
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        // Handle the different keyboard inputs that dictate movement
        if keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        //
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

// temporary //

pub fn temp_player_up_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        // Handle the different keyboard inputs that dictate movement
        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }
        //
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

// temporary //


// this doens't work how I want either, but it's a start
pub fn player_jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut JumpVelocity), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut jump_velocity)) = player_query.get_single_mut() {
        let up_direction = Vec3::new(0.0, 1.0, 0.0);

        if keyboard_input.just_pressed(KeyCode::Space) {
            jump_velocity.vertical_velocity += PLAYER_SPEED;
            println!("{}", jump_velocity.vertical_velocity);
        }
        transform.translation += up_direction * jump_velocity.vertical_velocity * time.delta_seconds()
            - GRAVITY * time.delta_seconds() * time.delta_seconds();
        
        
        if jump_velocity.vertical_velocity > 0.0 {
            jump_velocity.vertical_velocity -= GRAVITY;
        }


    }
}



pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let half_player_size = PLAYER_SIZE / 2.0;
        let x_pos_min = 0.0 + half_player_size;
        let x_pos_max = window.width() - half_player_size;
        let y_pos_min = 0.0 + half_player_size;
        let y_pos_max = window.height() - half_player_size;
        //
        let mut translation = player_transform.translation;
        
        if translation.x < x_pos_min {
            translation.x = x_pos_min;
        } else if translation.x > x_pos_max {
            translation.x = x_pos_max;
        }

        if translation.y < y_pos_min {
            translation.y = y_pos_min;
        } else if translation.y > y_pos_max {
            translation.y = y_pos_max;
        }

        player_transform.translation = translation;
    }


}

// Check if the player is grounded
pub fn ground_check(
    player_query: Query<(&Transform, &EntitySizeCollision), With<Player>>,
    floor_query: Query<(&Transform, &EntitySizeCollision), With<Floor>>,
    player_state: Res<State<PlayerState>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    if let Ok((player_transform, player_collision)) = player_query.get_single() {
        for (floor_transform, floor_collision) in floor_query.iter() {
            // check collision get boolean
            //let distance = player_transform.translation.distance(floor_transform.translation);
            let horizontal_distance = player_transform.translation.x - floor_transform.translation.x;
            let vertical_distance = player_transform.translation.y - floor_transform.translation.y;
            let horizontal_player_length = player_collision.horizontal_entity_size / 2.0;
            let vertical_player_length = player_collision.vertical_entity_size / 2.0;
            let horizontal_floor_length = floor_collision.horizontal_entity_size / 2.0;
            let vertical_floor_length = floor_collision.vertical_entity_size / 2.0;
            if 
            horizontal_distance < horizontal_player_length + horizontal_floor_length &&
            vertical_distance < vertical_player_length + vertical_floor_length {
            // if boolean is true {}            
                if player_state.0 == PlayerState::Air {
                    // switch to ground state
                    next_player_state.set(PlayerState::Grounded);
                    //println!("I'm grounded");
                }
            } else {
            // if boolean is false ()
                if player_state.0 == PlayerState::Grounded {
                    // switch to air state
                    next_player_state.set(PlayerState::Air);
                }
            }
        }
    }
}


// make a new system that, when the player is grounded, sets their y position to the floor's y position
pub fn force_player_to_ground(
    mut player_query: Query<(&mut Transform, &EntitySizeCollision), With<Player>>,
    floor_query: Query<&Transform, With<Floor>>,
    player_state: Res<State<PlayerState>>,
) {
    if let Ok((mut player_transform, player_collision)) = player_query.get_single_mut() {
        for floor_transform in floor_query.iter() {
            if player_state.0 == PlayerState::Grounded {
                player_transform.translation.y = floor_transform.translation.y;
            }
        }
    }
}