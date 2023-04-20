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
            Gravity {
                gravity_constant: GRAVITY,
            },
            //BoundingBox {
            //}
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
    player_query: Query<Entity, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_state: Res<State<PlayerState>>,
    mut next_player_state: ResMut<State<PlayerState>>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        //
    }
}