// game/player/systems.rs


use bevy::a11y::accesskit::Action;
use bevy::animation;
use bevy::input::keyboard;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::components::*;
use crate::game::player;
use crate::game::resources::*;
use crate::game::player::components::*;
use crate::game::player::PlayerState;
use crate::game::player::GroundedState;
use crate::game::player::AirState;
use crate::game::player::{PLAYER_SPEED_VERTICAL, PLAYER_SPEED_HORIZONTAL, PLAYER_SIZE};
use crate::game::GRAVITY;

use bevy::input::ButtonState;

use super::SPECIAL_MOVE_BUFFER_TIME;
use super::DIRECTION_JUMP_BUFFER_TIME;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Access the primary window
    let window = window_query.get_single().unwrap();

    //let sprite_sheet_handle = asset_server.load("sprites/lenneth/idle_anim/idle_spritesheet.png");    
    let texture_atlas = 
        TextureAtlas::from_grid(
            //asset_server.load("sprites/lenneth/idle_anim/idle_spritesheet.png"),
            //asset_server.load("sprites/lenneth/test_sprite_sheet/spritesheet.png"),
            asset_server.load("sprites/lenneth/test_sprite_sheet/test_lenneth_spritesheet_spread_mod.png"),
            // Inputs here are the size of each individual sprite inside the spritesheet
            //Vec2::new(64.0, 64.0), 12, 1, None, None
            Vec2::new(96.0, 64.0), 17, 4, None, None
        );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 11 };


    let texture_atlas_second = 
        TextureAtlas::from_grid(
            //asset_server.load("sprites/lenneth/idle_anim/idle_spritesheet.png"),
            //asset_server.load("sprites/lenneth/test_sprite_sheet/spritesheet.png"),
            asset_server.load("sprites/lenneth/test_sprite_sheet/test_lenneth_spritesheet_spread_mod.png"),
            // Inputs here are the size of each individual sprite inside the spritesheet
            //Vec2::new(64.0, 64.0), 12, 1, None, None
            Vec2::new(96.0, 64.0), 17, 4, None, None
        );

    let texture_atlas_handle_second = texture_atlases.add(texture_atlas_second);
    let animation_indices_second = AnimationIndices { first: 0, last: 11 };

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
                transform: Transform::from_xyz(window.width() / 4.0, window.height() / 2.0, 0.0),
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
            NegativeEdgeStateVector {
                negative_edge_vector: Vec::new()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        )

    );

    commands.spawn(
        (
            SpriteSheetBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                texture_atlas: texture_atlas_handle_second,
                sprite: TextureAtlasSprite::new(animation_indices_second.first),
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
            NegativeEdgeStateVector {
                negative_edge_vector: Vec::new()
            },
            animation_indices_second,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        )
    );

}

pub fn despawn_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    //player_query: Query<(Entity, &Player)>,
) {
    // If player_entity exists because the player_query contains some entity with the Player component,
    //   we want to despawn that player entity with commands
    //if let Ok(player_entity) = player_query.get_single() {
    for player_entity in player_query.iter() {
        commands.entity(player_entity).despawn()
    }
}

// player_query needs the transform along with player b/c we are trying to move the player
// we again also need the time resource
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut AnimationIndices, &mut TextureAtlasSprite), With<Player>>,
    time: Res<Time>,
    grounded_state: Res<State<GroundedState>>,
    mut next_player_state: ResMut<NextState<GroundedState>>,
) {
    // Get the single mutable thing that exists in player_query, and store it into the transform variable
    // If transform gets a Transform component, continue the if block
    //if let Ok((mut transform, mut animation_indeces, mut texture_atlas_sprite_sprite_sheet)) = player_query.get_single_mut() {
    for (mut transform, mut animation_indeces, mut texture_atlas_sprite_sprite_sheet) in player_query.iter_mut() {
        if grounded_state.0 == GroundedState::Neutral {

            let mut direction = Vec3::ZERO;
            // Handle the different keyboard inputs that dictate movement
            if keyboard_input.pressed(KeyCode::A) {
                direction += Vec3::new(-1.0, 0.0, 0.0);
                // set indeces to walking animation
            }
            if keyboard_input.pressed(KeyCode::D) {
                direction += Vec3::new(1.0, 0.0, 0.0);
                // set indeces to walking animation
            }  
            //
            if direction.length() > 0.0 {
                direction = direction.normalize();
            }
            transform.translation += direction * PLAYER_SPEED_HORIZONTAL * time.delta_seconds();


            
            let mut ending_index = if keyboard_input.pressed(KeyCode::S) {
                // crouch -- issue right now is that system runs whole animation, locking you out of other animations
                //  the goal here then is to switch in and out of Neutral and Crouching GroundedStates as you enter and exit those states
                //  use a new system? (Toggle crouch and release)
                next_player_state.set(GroundedState::Crouching);
                animation_indeces.first = 50;
                animation_indeces.last = 55;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                //next_player_state.set(PlayerState::Attack);
                animation_indeces.last
            }
            else {
                animation_indeces.last
            };

            /*
            let mut ending index = if keyboard event . released( KeyCode::S ) && player state == GroundedState::Crouching
                next player state .set( GroundedState::Neutral)
                animation_indeces first --> ...
                animation_indeces last --> ...
                texture_atlas_sprite_sprite_sheet index = animation_indeces first
                animation_indeces last <-- this is returned

            idea is if we detect a release S while the player is crouching, set the indeces to the neutral animation, and set state to neutral
            same process would work for walking
             */

        }
        
    }
}


/*
        if grounded_state.0 == GroundedState::Neutral {
            let mut ending_index = if keyboard_input.just_pressed(KeyCode::J) {
                
                next_grounded_state.set(GroundedState::Attack);
                //if player_state.0 == PlayerState::Grounded {
                //    animation_indeces.first = 11;
                //    animation_indeces.last = 16;
                //    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                //    next_player_state.set(PlayerState::Attack);
                //}
                //animation_indeces.first = 11;
                animation_indeces.first = 18;
                //animation_indeces.last = 16;
                animation_indeces.last = 21;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                //next_player_state.set(PlayerState::Attack);
                animation_indeces.last
            }
            else {
                animation_indeces.last
            };
            

 */





// temporary //

pub fn temp_player_up_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    //if let Ok(mut transform) = player_query.get_single_mut() {
    for mut transform in player_query.iter_mut() {
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
        transform.translation += direction * PLAYER_SPEED_VERTICAL * time.delta_seconds();
    }
}

// temporary //


pub fn player_jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut JumpVelocity, &ActionStateVector), With<Player>>,
    player_state: Res<State<PlayerState>>,
    grounded_state: Res<State<GroundedState>>,
) {
    //if let Ok((transform, mut jump_velocity, action_state_vector)) = player_query.get_single_mut() {
    for (transform, mut jump_velocity, action_state_vector) in player_query.iter_mut() {
        // Jumping left or right
        // create the time difference between last and second to last input, as well as the last two inputs, only if the number of inputs is greater than 2
        let (direction_jump_time_difference, (recent_key_first, recent_key_second)) = if action_state_vector.action_vector.len() >= 2 {
            // direction_jump_time_difference is the difference between the time values of the two most recent action time vector values
            // we need to create a recent action vector
            // let recent_action_vector = action_state_vector.action_vector.as_slice()[action_state_vector.action_vector.len()-3..].to_vec();
            let recent_action_vector = action_state_vector.action_vector.as_slice()[action_state_vector.action_vector.len()-2..].to_vec();
            let recent_time_difference = recent_action_vector[1].1 - recent_action_vector[0].1;
            let recent_key_first = recent_action_vector[0].0;
            let recent_key_second = recent_action_vector[1].0;
            (recent_time_difference, (recent_key_first, recent_key_second))
        } else {
            // give default info for time difference and inputs
            (500.0, (KeyCode::Key1, KeyCode::Key2))
        };

        if keyboard_input.just_pressed(KeyCode::Space) && player_state.0 == PlayerState::Grounded && grounded_state.0 == GroundedState::Neutral {
            println!("I just jumped");
            jump_velocity.vertical_velocity = PLAYER_SPEED_VERTICAL;
            

            if direction_jump_time_difference <= DIRECTION_JUMP_BUFFER_TIME {
                match recent_key_first {
                    KeyCode::A => jump_velocity.horizontal_velocity = -1.0 * PLAYER_SPEED_HORIZONTAL,
                    KeyCode::D => jump_velocity.horizontal_velocity = PLAYER_SPEED_HORIZONTAL,
                    _ => jump_velocity.horizontal_velocity = jump_velocity.horizontal_velocity,
                }
            } 
            
            
            
        }
        
    }
}





// changing the indeces works!
// now just need a way to make the coding process more efficient
//   and I need a better sprite sheet
// code also doesn't return to original index set
pub fn player_ground_attack(
    // needs mutable commands
    // needs player query
    // needs Action state vector
    // needs keyboard input
    // needs player state
    // needs the mutable next player state to transition to another player state
    //    player_state: Res<State<PlayerState>>,
    // mut next_player_state: ResMut<NextState<PlayerState>>,
    // needs the asset server
    // needs a texture atlas
    //   asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    // needs time resource
    // time: Res<Time>,

    // we don't actually need the texture atlas itself I think, because all we should need to do is change the AnimationIndeces
    // we do need the texture atlas, because we need to access the current index of the texture atlas

    mut commands: Commands,
    mut player_query: Query<(&ActionStateVector, &mut AnimationIndices, &mut TextureAtlasSprite), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut keyboard_event_reader: EventReader<KeyboardInput>,
    grounded_state: Res<State<GroundedState>>,
    mut next_grounded_state: ResMut<NextState<GroundedState>>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,


) {
    // if there's a player entity, get its texture atlas 
    // 
    // if a is pressed
    //   change the indeces of the texture atlas so that animate_sprite() in game/systems.rs loops through the correct portion of the spritesheet
    //
    // (create system that is meant to 'reset' the the player's state and the texture atlas' indeces to the
    //   new state's default index range once 1 loop of the non-default animation is done)
    //
    // inside a button if block, check if last several elements of input buffer match the input for a special move, and check if the time values of those elements
    //   are close enough to time.elapsed_seconds()

    // this probably goes with the external ending system that resets states...
    //let mut ending_index = 0;

    //if let Ok((action_state_vector, mut animation_indeces, mut texture_atlas_sprite_sprite_sheet)) = player_query.get_single_mut() {
    for (action_state_vector, mut animation_indeces, mut texture_atlas_sprite_sprite_sheet) in player_query.iter_mut() {    
        let ((second_first_difference, third_second_difference), (recent_key_first, recent_key_second, recent_key_third)) = if action_state_vector.action_vector.len() >= 3 {
        //if !action_state_vector.action_vector.is_empty() {
            // assign last several inputs into a variable to check
            //let mut recent_action_vector = action_state_vector.action_vector.last().unwrap();
            let recent_action_vector = action_state_vector.action_vector.as_slice()[action_state_vector.action_vector.len()-3..].to_vec();
            let recent_time_first: f32 = recent_action_vector[0].1;
            let recent_time_second: f32 = recent_action_vector[1].1;
            let recent_time_third: f32 = recent_action_vector[2].1;

            let second_first_difference: f32 = recent_time_second - recent_time_first;
            let third_second_difference: f32 = recent_time_third - recent_time_second;

            //println!("_______________________________Recent actions: {:?}", recent_action_vector);
            //println!("first: {}, second: {}, third: {}", recent_element_first, recent_element_second, recent_element_third);
            //println!("2 - 1: {}, 3 - 2: {}", second_first_difference, third_second_difference);

            println!("first key: {:?}, second key: {:?}, third key: {:?}", recent_action_vector[0].0, recent_action_vector[1].0, recent_action_vector[2].0);

            ((second_first_difference, third_second_difference), (recent_action_vector[0].0, recent_action_vector[1].0, recent_action_vector[2].0))

        }
        else {
            ((500.0, 500.0), (KeyCode::Key1, KeyCode::Key2, KeyCode::Key3))
        };


        /*
        
        let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let last3 = v.as_slice()[v.len()-3..].to_vec();
         */



        // In the future replace all of this let if statements into a single match block

        if grounded_state.0 == GroundedState::Neutral {
            let mut ending_index = if keyboard_input.just_pressed(KeyCode::J) {
                
                next_grounded_state.set(GroundedState::Attack);
                //if player_state.0 == PlayerState::Grounded {
                //    animation_indeces.first = 11;
                //    animation_indeces.last = 16;
                //    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                //    next_player_state.set(PlayerState::Attack);
                //}
                //animation_indeces.first = 11;
                animation_indeces.first = 18;
                //animation_indeces.last = 16;
                animation_indeces.last = 21;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                //next_player_state.set(PlayerState::Attack);
                animation_indeces.last
            }
            else {
                animation_indeces.last
            };
            

            /* 
            // building block for overall input handler... fill in later
            // keyboard_event.key_code.unwrap()
            for keyboard_event in keyboard_event_reader.iter() {
                let mut test_index = match keyboard_event.key_code.unwrap() {
                    KeyCode::J =>
                        {
                            println!("Light Attack");
                            animation_indeces.last
                        }
                    KeyCode::K => 
                        {
                            println!("Medium Attack");
                            animation_indeces.last
                        }
                    KeyCode::L => 
                        {
                            println!("Heavy Attack");
                            animation_indeces.last
                        }
                    _ => animation_indeces.last,
                };
            };
            
            */




            // can repeat same structure, jsut with different first and last indeces and different keycodes
            ending_index = if keyboard_input.just_pressed(KeyCode::K) {
                next_grounded_state.set(GroundedState::Attack);
                println!("Doing K attack -- Medium");
                //animation_indeces.first = 11;
                animation_indeces.first = 18;
                //animation_indeces.last = 16;
                animation_indeces.last = 21;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last

            }
            else {
                animation_indeces.last
            };


            ending_index = if keyboard_input.just_pressed(KeyCode::L) {
                next_grounded_state.set(GroundedState::Attack);
                println!("Doing L attack -- Heavy");
                animation_indeces.last
            }
            else {
                animation_indeces.last
            };


            // -- Special Moves -- //


            // try to make an if block for quarter circle forward here -- down, down-right, right + button
            //   this would actually just be in the J, K, L, or ; if blocks...
            //   maybe they could be in there own system???

            // fireball
            //   if you pressed j and your recent action vector contains S, D, j, do fireball
            ending_index = if keyboard_input.just_pressed(KeyCode::J) 
                && second_first_difference <= SPECIAL_MOVE_BUFFER_TIME 
                && third_second_difference <= SPECIAL_MOVE_BUFFER_TIME
                && recent_key_first == KeyCode::S
                && recent_key_second == KeyCode::D
                && recent_key_third == KeyCode::J
                {
                    next_grounded_state.set(GroundedState::Attack);
                    println!("Doing Light Forward Fireball");
                    animation_indeces.first = 33;
                    //animation_indeces.last = 16;
                    animation_indeces.last = 48;
                    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                    animation_indeces.last
                }
                else {
                    animation_indeces.last
                };

            ending_index = if keyboard_input.just_pressed(KeyCode::K) 
                && second_first_difference <= SPECIAL_MOVE_BUFFER_TIME
                && third_second_difference <= SPECIAL_MOVE_BUFFER_TIME 
                && recent_key_first == KeyCode::S
                && recent_key_second == KeyCode::D
                && recent_key_third == KeyCode::K
                {
                    next_grounded_state.set(GroundedState::Attack);
                    println!("Doing Medium Forward Fireball");
                    animation_indeces.first = 0;
                    //animation_indeces.last = 16;
                    animation_indeces.last = 21;
                    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                    animation_indeces.last
                }
                else {
                    animation_indeces.last
                };


            ending_index = if keyboard_input.just_pressed(KeyCode::L) 
                && second_first_difference <= SPECIAL_MOVE_BUFFER_TIME 
                && third_second_difference <= SPECIAL_MOVE_BUFFER_TIME 
                && recent_key_first == KeyCode::S
                && recent_key_second == KeyCode::D
                && recent_key_third == KeyCode::L
                {
                    next_grounded_state.set(GroundedState::Attack);
                    println!("Doing Heavy Forward Fireball");
                    animation_indeces.first = 0;
                    //animation_indeces.last = 16;
                    animation_indeces.last = 21;
                    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                    animation_indeces.last
                }
                else {
                    animation_indeces.last
                };


            // -- Special Moves -- //

            // --- Ending Bit !! --- //
            // this needs to go in its own system at some point
            //if texture_atlas_sprite_sprite_sheet.index == ending_index {
            //    // reset animation indeces to the default for the particular state
            //    animation_indeces.first = 0;
            //    animation_indeces.last = 11;
            //    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
            //    ending_index = animation_indeces.last;
            //    next_grounded_state.set(GroundedState::Neutral);
            //    //next_player_state.set(PlayerState::Grounded);
            //}

            //println!("(first index, last index): [{}, {}], current index: {}, ending index: {}", animation_indeces.first, animation_indeces.last, texture_atlas_sprite_sprite_sheet.index, ending_index);

            // --- Ending Bit !! --- //
        }



    }
}

pub fn player_reset_to_neutral(
    mut player_query: Query<(&mut AnimationIndices, &mut TextureAtlasSprite), With<Player>>,
    grounded_state: Res<State<GroundedState>>,
    mut next_grounded_state: ResMut<NextState<GroundedState>>,
    air_state: Res<State<AirState>>,
    mut next_air_state: ResMut<NextState<AirState>>,
) {
    //if let Ok((mut animation_indeces, mut texture_atlas_sprite_sprite_sheet)) = player_query.get_single_mut() {
    for (mut animation_indeces, mut texture_atlas_sprite_sprite_sheet) in player_query.iter_mut() {
        if texture_atlas_sprite_sprite_sheet.index == animation_indeces.last && (grounded_state.0 == GroundedState::Attack || air_state.0 == AirState::Attack || grounded_state.0 == GroundedState::Crouching) {
            // reset animation indeces to the default for the particular state
            animation_indeces.first = 0;
            animation_indeces.last = 11;
            texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
            //next_player_state.set(PlayerState::Grounded);

            // reset to neutral state
            next_grounded_state.set(GroundedState::Neutral);
            next_air_state.set(AirState::Neutral);
        }

    }
}


pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    //if let Ok(mut player_transform) = player_query.get_single_mut() {
    for mut player_transform in player_query.iter_mut() {    
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
    mut player_query: Query<(&Transform, &EntitySizeCollision, &mut JumpVelocity), With<Player>>,
    floor_query: Query<(&Transform, &EntitySizeCollision), With<Floor>>,
    player_state: Res<State<PlayerState>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    //if let Ok((player_transform, player_collision, mut jump_velocity)) = player_query.get_single_mut() {
    for (player_transform, player_collision, mut jump_velocity) in player_query.iter_mut() {
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

                    // we also want to force horizontal velocity to be 0 here
                    jump_velocity.horizontal_velocity = 0.0;
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



// Debug system -- might not need to be a debug...
pub fn populate_player_action_vector(
    mut player_query: Query<(&mut ActionStateVector, &mut NegativeEdgeStateVector), With<Player>>,
    //keyboard_input: Res<Input<KeyCode>>,
    mut keyboard_event_reader: EventReader<KeyboardInput>,
    time: Res<Time>,
    mut input_buffer_timer: ResMut<InputBufferTimer>
) {
    // If we get a valid player entity, have player_action_state_vector get that entity's ActioniStateVector component
    //if let Ok((mut player_action_state_vector, mut player_negative_edge_vector)) = player_query.get_single_mut() {
    for (mut player_action_state_vector, mut player_negative_edge_vector) in player_query.iter_mut() {
        
        // printing
        println!("{:?}", player_action_state_vector.action_vector);
        //for (key_code, action_timer_value) in &player_action_state_vector.action_vector {
        //    println!("{:?}", key_code);
        //    println!("{:?}", action_timer_value);
        //}

        

        // Tick the timer
        input_buffer_timer.timer.tick(time.delta());

        if input_buffer_timer.timer.finished() {
            // clear the player's action state vector
            //player_action_state_vector.action_vector.clear();

            // If the input buffer has elements, remove the 0th element (the one added last)
            if !player_action_state_vector.action_vector.is_empty() {
                player_action_state_vector.action_vector.remove(0);
            }
            if !player_negative_edge_vector.negative_edge_vector.is_empty() {
                player_negative_edge_vector.negative_edge_vector.remove(0);
            }
        }

        for keyboard_event in keyboard_event_reader.iter() {
            match keyboard_event.state {
                ButtonState::Pressed => {
                    //println!("Key press: {:?} ({})", keyboard_event.key_code, keyboard_event.scan_code);
                    player_action_state_vector.action_vector.push((keyboard_event.key_code.unwrap(), time.elapsed_seconds()) );
                }
                ButtonState::Released => {
                    println!("Key release: {:?} ({})", keyboard_event.key_code, keyboard_event.scan_code);
                    player_negative_edge_vector.negative_edge_vector.push((keyboard_event.key_code.unwrap(), time.elapsed_seconds()) );

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