// game/player/systems.rs


use bevy::a11y::accesskit::Action;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::components::*;
use crate::game::player;
use crate::game::resources::*;
use crate::game::player::components::*;
use crate::game::player::PlayerState;
use crate::game::player::{PLAYER_SPEED, PLAYER_SIZE};
use crate::game::GRAVITY;

use bevy::input::ButtonState;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Access the primary window
    let window = window_query.get_single().unwrap();

    //let sprite_sheet_handle = asset_server.load("sprites/lenneth/idle_anim/idle_spritesheet.png");    
    // Something is off, not working...
    let texture_atlas = 
        TextureAtlas::from_grid(
            asset_server.load("sprites/lenneth/idle_anim/idle_spritesheet.png"),
            // Inputs here are the size of each individual sprite inside the spritesheet
            Vec2::new(64.0, 64.0), 12, 1, None, None
        );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 11 };
    
    // spawn a Player with the Player and Gravity components
    commands.spawn(
        (
            //SpriteBundle {
            //    transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            //    // the sprite bundle has a texture using the load() method
            //    //texture: asset_server.load("sprites/ball_blue_large.png"),
            //    texture: asset_server.load("sprites/lenneth_test.png"),
            //    ..default()
            //},
            SpriteSheetBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                ..default()
            },
            Player {},
            Gravity {
                gravity_constant: GRAVITY,
            },
            EntitySizeCollision {
                horizontal_entity_size: PLAYER_SIZE,
                vertical_entity_size: PLAYER_SIZE,
            },
            JumpVelocity {
                horizontal_velocity: 0.0,
                vertical_velocity: 0.0,
            },
            ActionStateVector {
                action_vector: Vec::new(),
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
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


pub fn player_jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut JumpVelocity), With<Player>>,
    //time: Res<Time>,
) {
    if let Ok((mut transform, mut jump_velocity)) = player_query.get_single_mut() {
        //let up_direction = Vec3::new(0.0, 1.0, 0.0);

        if keyboard_input.just_pressed(KeyCode::Space) {
            println!("I just jumped");
            jump_velocity.vertical_velocity = PLAYER_SPEED;
            //println!("{}", jump_velocity.vertical_velocity);
        }
        //transform.translation += up_direction * jump_velocity.vertical_velocity * time.delta_seconds()
        //    - GRAVITY * time.delta_seconds() * time.delta_seconds();
        
        
    }
}


pub fn player_attack(
    // needs mutable commands
    // needs player query
    // needs keyboard input
    // needs player state
    // needs the mutable next player state to transition to another player state
    //    player_state: Res<State<PlayerState>>,
    // mut next_player_state: ResMut<NextState<PlayerState>>,
    // needs the asset server
    // needs a texture atlas
    //   asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    //
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
            let _horizontal_distance = player_transform.translation.x - floor_transform.translation.x;
            let vertical_distance = player_transform.translation.y - floor_transform.translation.y;
            let _horizontal_player_length = player_collision.horizontal_entity_size / 2.0;
            let vertical_player_length = player_collision.vertical_entity_size / 2.0;
            let _horizontal_floor_length = floor_collision.horizontal_entity_size / 2.0;
            let vertical_floor_length = floor_collision.vertical_entity_size / 2.0;
            
            // if (horizontal_distance < horizontal_player_length + horizontal_floor_length)
            if vertical_distance < vertical_player_length + vertical_floor_length {

            // if boolean is true {}            
                if player_state.0 == PlayerState::Air {
                    // switch to ground state
                    next_player_state.set(PlayerState::Grounded);
                    println!("I'm grounded");
                }
            } else {
            // if boolean is false ()
                if player_state.0 == PlayerState::Grounded {
                    // switch to air state
                    next_player_state.set(PlayerState::Air);
                    //println!("I'm in the air");
                }
            }
        }
    }
}


// this system doesn't work right now
// make a new system that, when the player is grounded, sets their y position to the floor's y position
pub fn _force_player_to_ground(
    mut player_query: Query<(&mut Transform, &EntitySizeCollision), With<Player>>,
    floor_query: Query<(&Transform, &EntitySizeCollision), With<Floor>>,
) {
    if let Ok((mut player_transform, player_collision)) = player_query.get_single_mut() {
        for (floor_transform, floor_collision) in floor_query.iter() {
            let vertical_distance = player_transform.translation.y - floor_transform.translation.y;
            let vertical_player_length = player_collision.vertical_entity_size / 2.0;
            let vertical_floor_length = floor_collision.vertical_entity_size / 2.0;
            if vertical_distance < vertical_player_length + vertical_floor_length {
                player_transform.translation.y = floor_transform.translation.y - player_collision.vertical_entity_size;
            }
        }
    }
}



// Debug system -- might not need to be a debug...
pub fn debug_get_player_action_vector(
    mut player_query: Query<&mut ActionStateVector, With<Player>>,
    //keyboard_input: Res<Input<KeyCode>>,
    mut keyboard_event_reader: EventReader<KeyboardInput>,
    time: Res<Time>,
    mut input_buffer_timer: ResMut<InputBufferTimer>
) {
    // If we get a valid player entity, have player_action_state_vector get that entity's ActioniStateVector component
    if let Ok(mut player_action_state_vector) = player_query.get_single_mut() {
        
        // printing
        //println!("{}", player_action_state_vector.action_vector);
        for (key_code, action_timer_value) in &player_action_state_vector.action_vector {
            println!("{:?}", key_code);
            println!("{:?}", action_timer_value.action_timer);
        }

        

        // Tick the timer
        input_buffer_timer.timer.tick(time.delta());

        if input_buffer_timer.timer.finished() {
            // clear the player's action state vector
            player_action_state_vector.action_vector.clear();
        }

        for keyboard_event in keyboard_event_reader.iter() {
            match keyboard_event.state {
                ButtonState::Pressed => {
                    println!("Key press: {:?} ({})", keyboard_event.key_code, keyboard_event.scan_code);
                    player_action_state_vector.action_vector.push((keyboard_event.key_code.unwrap(), ActionTimerValue {action_timer: time.elapsed_seconds()})) ;
                }
                ButtonState::Released => {
                    println!("Key release: {:?} ({})", keyboard_event.key_code, keyboard_event.scan_code);
                }
            }
            // add the keyboard_event to the action state vector reference.
            //   might need to change type of the Action state vector to contain a vector of key events instead of a key codes
        }


        //if keyboard_input.just_pressed(KeyCode::L) {
        //    println!("Player action state vector: {:?}", player_action_state_vector.action_vector);
        //}
        
    }
}