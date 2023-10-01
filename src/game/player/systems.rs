// game/player/systems.rs


use bevy::a11y::accesskit::Action;
use bevy::animation;
use bevy::ecs::storage;
use bevy::input::keyboard;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::components::*;
use crate::game::player;
use crate::game::resources::*;
use crate::game::player::components::*;
//use crate::game::player::PlayerState;
//use crate::game::player::AttackState;
use crate::game::player::{PLAYER_SPEED_VERTICAL, PLAYER_SPEED_HORIZONTAL, PLAYER_SIZE, MARISA_PLAYER_SIZE};
use crate::game::GRAVITY;
use crate::game::SimulationState;

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
            //asset_server.load("sprites/lenneth/test_sprite_sheet/test_lenneth_spritesheet_spread_mod.png"),
            asset_server.load("sprites/touhou/marisa_kirisame/marisa_spritesheet.png"),
            // Inputs here are the size of each individual sprite inside the spritesheet
            //Vec2::new(64.0, 64.0), 12, 1, None, None
            //Vec2::new(96.0, 64.0), 17, 4, None, None,
            // Marisa
            Vec2::new(288.0, 144.0), 22, 41, None, None,
        );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 9 };


    let texture_atlas_second = 
        TextureAtlas::from_grid(
            //asset_server.load("sprites/lenneth/idle_anim/idle_spritesheet.png"),
            //asset_server.load("sprites/lenneth/test_sprite_sheet/spritesheet.png"),
            asset_server.load("sprites/touhou/marisa_kirisame/marisa_spritesheet.png"),
            // Inputs here are the size of each individual sprite inside the spritesheet
            //Vec2::new(64.0, 64.0), 12, 1, None, None
            Vec2::new(288.0, 144.0), 22, 41, None, None
        );

    let texture_atlas_handle_second = texture_atlases.add(texture_atlas_second);
    let animation_indices_second = AnimationIndices { first: 0, last: 9 };

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
            SpriteSheetIndeces {
                idle_first: 0,
                idle_last: 9,
                crouching_first: 22,
                crouching_last: 32,
                walk_forward_first: 44,
                walk_forward_last: 51,
                walk_back_first: 66,
                walk_back_last: 71,
                dash_forward_first: 88,
                dash_forward_last: 96,
                dash_back_first: 110,
                dash_back_last: 116,
                jump_first: 132,
                jump_last: 145,
                jump_forward_first: 154,
                jump_forward_last: 165,
                jump_back_first: 176,
                jump_back_last: 183,
                throw_first: 198,
                throw_last: 208,
                throw_jumping_first: 220,
                throw_jumping_last: 230,
                universal_overhead_first: 242,
                universal_overhead_last: 251,
                light_first: 264,
                light_last: 270,
                light_crouching_first: 286,
                light_crouching_last: 293,
                light_jumping_first: 308,
                light_jumping_last: 317,
                light_far_first: 330,
                light_far_last: 338,
                medium_first: 352,
                medium_last: 365,
                medium_crouching_first: 374,
                medium_crouching_last: 382,
                medium_jumping_first: 396,
                medium_jumping_last: 402,
                heavy_first: 418,
                heavy_last: 427,
                heavy_crouching_first: 440,
                heavy_crouching_last: 450,
                heavy_jumping_first: 462,
                heavy_jumping_last: 470,
                unique_first: 484,
                unique_last: 493,
                unique_crouching_first: 506,
                unique_crouching_last: 525,
                unique_jumping_first: 528,
                unique_jumping_last: 537,
                unique_forward_first: 550,
                unique_forward_last: 559,
                special_first: 572,
                special_last: 588,
                special_crouching_first: 594,
                special_crouching_last: 602,
                special_jumping_first: 616,
                special_jumping_last: 625,
                special_forward_first: 638,
                special_forward_last: 648,
                special_back_first: 660,
                special_back_last: 681,
                blocking_first: 682,
                blocking_last: 683,
                blocking_crouching_first: 704,
                blocking_crouching_last: 705,
                blocking_jumping_first: 726,
                blocking_jumping_last: 727,
                hit_standing_first: 748,
                hit_standing_last: 750,
                hit_crouching_first: 770,
                hit_crouching_last: 772,
                hit_air_first: 792,
                hit_air_last: 799,
                hit_throw_first: 814,
                hit_throw_last: 820,
                knockdown_light_first: 836,
                knockdown_light_last: 842,
                knockdown_hard_first: 858,
                knockdown_hard_last: 864,
                knockdown_getup_first: 880,
                knockdown_getup_last: 892,
            },
            Player {},
            Gravity {
                gravity_constant: GRAVITY,
            },
            EntitySizeCollision {
                horizontal_entity_size: PLAYER_SIZE,
                vertical_entity_size: MARISA_PLAYER_SIZE,
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
            PlayerInput {
                up: false,
                down: false,
                left: false,
                right: false,
                light: false,
                medium: false,
                heavy: false,
                unique: false,
                special: false,
            },
            InputBinding {
                up_bind: KeyCode::W,
                down_bind: KeyCode::S,
                left_bind: KeyCode::A,
                right_bind: KeyCode::D,
                light_bind: KeyCode::J,
                medium_bind: KeyCode::K,
                heavy_bind: KeyCode::L,
                unique_bind: KeyCode::I,
                special_bind: KeyCode::O,
            },
            MovementState {
                is_grounded: false,
                is_walking: false,
            },
            AttackState {
                is_attacking: false,
            },
            PlayerNumber {
                player_number: 1,
            },

            
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
            SpriteSheetIndeces {
                idle_first: 0,
                idle_last: 9,
                crouching_first: 22,
                crouching_last: 32,
                walk_forward_first: 44,
                walk_forward_last: 51,
                walk_back_first: 66,
                walk_back_last: 71,
                dash_forward_first: 88,
                dash_forward_last: 96,
                dash_back_first: 110,
                dash_back_last: 116,
                jump_first: 132,
                jump_last: 145,
                jump_forward_first: 154,
                jump_forward_last: 165,
                jump_back_first: 176,
                jump_back_last: 183,
                throw_first: 198,
                throw_last: 208,
                throw_jumping_first: 220,
                throw_jumping_last: 230,
                universal_overhead_first: 242,
                universal_overhead_last: 251,
                light_first: 264,
                light_last: 270,
                light_crouching_first: 286,
                light_crouching_last: 293,
                light_jumping_first: 308,
                light_jumping_last: 317,
                light_far_first: 330,
                light_far_last: 338,
                medium_first: 352,
                medium_last: 365,
                medium_crouching_first: 374,
                medium_crouching_last: 382,
                medium_jumping_first: 396,
                medium_jumping_last: 402,
                heavy_first: 418,
                heavy_last: 427,
                heavy_crouching_first: 440,
                heavy_crouching_last: 450,
                heavy_jumping_first: 462,
                heavy_jumping_last: 470,
                unique_first: 484,
                unique_last: 493,
                unique_crouching_first: 506,
                unique_crouching_last: 525,
                unique_jumping_first: 528,
                unique_jumping_last: 537,
                unique_forward_first: 550,
                unique_forward_last: 559,
                special_first: 572,
                special_last: 588,
                special_crouching_first: 594,
                special_crouching_last: 602,
                special_jumping_first: 616,
                special_jumping_last: 625,
                special_forward_first: 638,
                special_forward_last: 648,
                special_back_first: 660,
                special_back_last: 681,
                blocking_first: 682,
                blocking_last: 683,
                blocking_crouching_first: 704,
                blocking_crouching_last: 705,
                blocking_jumping_first: 726,
                blocking_jumping_last: 727,
                hit_standing_first: 748,
                hit_standing_last: 750,
                hit_crouching_first: 770,
                hit_crouching_last: 772,
                hit_air_first: 792,
                hit_air_last: 799,
                hit_throw_first: 814,
                hit_throw_last: 820,
                knockdown_light_first: 836,
                knockdown_light_last: 842,
                knockdown_hard_first: 858,
                knockdown_hard_last: 864,
                knockdown_getup_first: 880,
                knockdown_getup_last: 892,
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
            PlayerInput {
                up: false,
                down: false,
                left: false,
                right: false,
                light: false,
                medium: false,
                heavy: false,
                unique: false,
                special: false,
            },
            InputBinding {
                up_bind: KeyCode::Key5,
                down_bind: KeyCode::T,
                left_bind: KeyCode::R,
                right_bind: KeyCode::Y,
                light_bind: KeyCode::Key7,
                medium_bind: KeyCode::Key8,
                heavy_bind: KeyCode::Key9,
                unique_bind: KeyCode::U,
                special_bind: KeyCode::Key0,
            },
            MovementState {
                is_grounded: false,
                is_walking: false,
            },
            AttackState {
                is_attacking: false,
            },
            PlayerNumber {
                player_number: 2,
            }
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



// mut player_query: Query<(&ActionStateVector, &mut AnimationIndices,
// &mut TextureAtlasSprite, &PlayerInput, &MovementState, &mut AttackState, &SpriteSheetIndeces), With<Player>>,


// player_query needs the transform along with player b/c we are trying to move the player
// we again also need the time resource
pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut AnimationIndices, &mut TextureAtlasSprite, &mut PlayerInput, &mut JumpVelocity, &mut MovementState, &AttackState, &SpriteSheetIndeces), With<Player>>,
    time: Res<Time>,
    //player_state: Res<State<PlayerState>>,
    //mut next_player_state: ResMut<NextState<PlayerState>>,
    //attack_state: Res<State<AttackState>>,
    //mut next_attack_state: ResMut<NextState<AttackState>>,
) {
    // Get the single mutable thing that exists in player_query, and store it into the transform variable
    // If transform gets a Transform component, continue the if block
    //if let Ok((mut transform, mut animation_indeces, mut texture_atlas_sprite_sprite_sheet)) = player_query.get_single_mut() {
    for (mut transform, mut animation_indeces, mut texture_atlas_sprite_sprite_sheet, mut player_input, mut player_velocity, mut movement_state, attack_state, sprite_sheet_indeces) in player_query.iter_mut() {
        if !attack_state.is_attacking && movement_state.is_grounded {

            // holding: player_state.0 == PlayerState::Grounded

            let mut direction = Vec3::ZERO;


            /* 
            if player_input.up && player_input.left {
                // Jump left -- set vertical and horizontal velocity
            
            }

            if player_input.up && player_input.right {
                // Jump right
            }
            */



/*

            // Back Throw Logic //
            ending_index = if player_input.light && player_input.unique &&
                ( (player_input.left && !texture_atlas_sprite_sprite_sheet.flip_x) || (player_input.right && texture_atlas_sprite_sprite_sheet.flip_x) ) {
            //
                println!("Doing back throw");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.throw_first;
                animation_indeces.last = sprite_sheet_indeces.throw_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            } else {
                animation_indeces.last
            };
*/
            if player_input.left || player_input.right {
                println!("ksjnvkfjnvkdnvkndkjnkndkkdjn");
                movement_state.is_walking = true;
            } else {
                movement_state.is_walking = false;
            }
            // Handle the different keyboard inputs that dictate movement
            if player_input.left {
                //movement_state.is_walking = true;
                direction += Vec3::new(-1.0, 0.0, 0.0);
                //println!("left movement state: {}", movement_state.is_walking);
                //if !texture_atlas_sprite_sprite_sheet.flip_x {
                //    // set indeces to walking animation
                //    animation_indeces.first = sprite_sheet_indeces.walk_back_first;
                //    animation_indeces.last = sprite_sheet_indeces.walk_back_last;
                //    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                //} else {
                //    // when texture_atlas_sprite_sprite_sheet.flip_x is true
                //    animation_indeces.first = sprite_sheet_indeces.walk_forward_first;
                //    animation_indeces.last = sprite_sheet_indeces.walk_forward_last;
                //    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                //}

            } else {
                //movement_state.is_walking = false;
                //println!("left movement state: {}", movement_state.is_walking);
            }

            if player_input.right {
                //movement_state.is_walking = true;
                direction += Vec3::new(1.0, 0.0, 0.0);

                //println!("right movement state: {}", movement_state.is_walking);

                //if !texture_atlas_sprite_sprite_sheet.flip_x {
                //    // set indeces to walking animation
                //    animation_indeces.first = sprite_sheet_indeces.walk_forward_first;
                //    animation_indeces.last = sprite_sheet_indeces.walk_forward_last;
                //    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                //} else {
                //    //  when texture_atlas_sprite_sprite_sheet.flip_x is true
                //    animation_indeces.first = sprite_sheet_indeces.walk_back_first;
                //    animation_indeces.last = sprite_sheet_indeces.walk_back_last;
                //    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                //}

            } else {
                //movement_state.is_walking = false;
                //println!("right movement state: {}", movement_state.is_walking);
            }

            //
            if direction.length() > 0.0 {
                direction = direction.normalize();
            }

            if player_input.up {
                // Jumping
                player_velocity.vertical_velocity = PLAYER_SPEED_VERTICAL;
                player_velocity.horizontal_velocity = direction.x * PLAYER_SPEED_HORIZONTAL;
            }

            transform.translation += direction * PLAYER_SPEED_HORIZONTAL * time.delta_seconds();



            let mut ending_index = if player_input.down {
                animation_indeces.first = sprite_sheet_indeces.crouching_first;
                animation_indeces.last = sprite_sheet_indeces.crouching_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
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


// This should be renamed into player_movement_animation
pub fn loop_walking_animation(
    //
    mut player_query: Query<(&mut AnimationIndices, &mut TextureAtlasSprite, &mut MovementState, &SpriteSheetIndeces, &PlayerInput), With<Player>>,
    
) {
    //
    for (mut animation_indeces, mut texture_atlas_sprite_sprite_sheet, mut movement_state, sprite_sheet_indeces, player_input) in player_query.iter_mut() {
        //

        
        println!("movement state external: {}", movement_state.is_walking);


        if texture_atlas_sprite_sprite_sheet.index == animation_indeces.last { //|| !movement_state.is_walking
            // When you try to implement looping for jump and crouch animations, I think they sprite logic should go in this block

            if player_input.left {
                if !texture_atlas_sprite_sprite_sheet.flip_x {
                    // set indeces to walking animation
                    animation_indeces.first = sprite_sheet_indeces.walk_back_first;
                    animation_indeces.last = sprite_sheet_indeces.walk_back_last;
                    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                } else {
                    // when texture_atlas_sprite_sprite_sheet.flip_x is true
                    animation_indeces.first = sprite_sheet_indeces.walk_forward_first;
                    animation_indeces.last = sprite_sheet_indeces.walk_forward_last;
                    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                }
            }

            if player_input.right {
                if !texture_atlas_sprite_sprite_sheet.flip_x {
                    // set indeces to walking animation
                    animation_indeces.first = sprite_sheet_indeces.walk_forward_first;
                    animation_indeces.last = sprite_sheet_indeces.walk_forward_last;
                    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                } else {
                    //  when texture_atlas_sprite_sprite_sheet.flip_x is true
                    animation_indeces.first = sprite_sheet_indeces.walk_back_first;
                    animation_indeces.last = sprite_sheet_indeces.walk_back_last;
                    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                }

            }
        }

        // this block overrides every other animation i think
        //if !movement_state.is_walking {
        //    animation_indeces.first = sprite_sheet_indeces.idle_first;
        //    animation_indeces.last = sprite_sheet_indeces.idle_last;
        //    texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
        //}

        
        // I want some logic here that resets me to idle animations if im not walking, jumping or crouching




/*



*/



    }
}




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
            //direction += Vec3::new(0.0, 1.0, 0.0);
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


pub fn _player_jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut JumpVelocity, &ActionStateVector), With<Player>>,
    //player_state: Res<State<PlayerState>>,
    //attack_state: Res<State<AttackState>>,
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

        /*
        if keyboard_input.just_pressed(KeyCode::Space) && player_state.0 == PlayerState::Grounded && attack_state.0 == AttackState::Neutral {
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
        */
        
    }
}




// This should be renamed to player_attack
//  add a new system called player_attack_animation
//  this system should take the code blocks currently in player_attack that deal with setting the animation indeces
//  player_attack should then be for dealing with input...
// No i dont need to do this -- both systems would have to process inputs
//  need to think about this more...
//
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

    mut commands: Commands, // <-- used for spawning projectiles I think? Also used for spawning hitboxes
    mut player_query: Query<(&ActionStateVector, &mut AnimationIndices, &mut TextureAtlasSprite, &PlayerInput, &MovementState, &mut AttackState, &SpriteSheetIndeces), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut keyboard_event_reader: EventReader<KeyboardInput>,
    //attack_state: Res<State<AttackState>>,
    //mut next_attack_state: ResMut<NextState<AttackState>>,
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
    for (action_state_vector, mut animation_indeces, mut texture_atlas_sprite_sprite_sheet, player_input, movement_state, mut attack_state, sprite_sheet_indeces) in player_query.iter_mut() {    
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

            //println!("first key: {:?}, second key: {:?}, third key: {:?}", recent_action_vector[0].0, recent_action_vector[1].0, recent_action_vector[2].0);

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

        
        if !attack_state.is_attacking && movement_state.is_grounded {
            
            // Throw Logic //
            let mut ending_index = if player_input.light && player_input.unique {
                //
                println!("Doing throw");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.throw_first;
                animation_indeces.last = sprite_sheet_indeces.throw_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            } else {
                animation_indeces.last
            };
            
            // Back Throw Logic //
            ending_index = if player_input.light && player_input.unique &&
                ( (player_input.left && !texture_atlas_sprite_sprite_sheet.flip_x) || (player_input.right && texture_atlas_sprite_sprite_sheet.flip_x) ) {
            //
                println!("Doing back throw");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.throw_first;
                animation_indeces.last = sprite_sheet_indeces.throw_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            } else {
                animation_indeces.last
            };

            // Universal Overhead Logic //
            ending_index = if player_input.medium && player_input.heavy {
                //
                println!("Doing universal overhead");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.universal_overhead_first;
                animation_indeces.last = sprite_sheet_indeces.universal_overhead_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            } else {
                animation_indeces.last
            };      
            
            // Light Attack Logic //
            ending_index = if player_input.light {
                
                println!("Doing Light Attack");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.light_first;
                animation_indeces.last = sprite_sheet_indeces.light_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            }
            else {
                animation_indeces.last
            };
            

            // Crouching Light Attack Logic // 
            ending_index = if player_input.light && player_input.down {
                println!("Doing Crouching Light Attack");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.light_crouching_first;
                animation_indeces.last = sprite_sheet_indeces.light_crouching_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last

            } else {
                animation_indeces.last
            };


            // Forward Light Attack Logic // 
            ending_index = if player_input.light &&
                ( (player_input.right && !texture_atlas_sprite_sprite_sheet.flip_x) || (player_input.left && texture_atlas_sprite_sprite_sheet.flip_x) ){
                println!("Doing Forward Light Attack");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.light_far_first;
                animation_indeces.last = sprite_sheet_indeces.light_far_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last

            } else {
                animation_indeces.last
            };

            /*
            // Back Light Attack Logic // Not using back light attack for now
            ending_index = if player_input.light &&
                ( (player_input.left && !texture_atlas_sprite_sprite_sheet.flip_x) || (player_input.right && texture_atlas_sprite_sprite_sheet.flip_x) ){
                println!("Doing Back Light Attack");
                attack_state.is_attacking = true;
                //animation_indeces.first = Sprite Sheet Indeces var first
                //animation_indeces.last = Sprite Sheet Indeces var last
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last

            } else {
                animation_indeces.last
            };
             */

            // Medium Attack Logic //
            // can repeat same structure, just with different first and last indeces and different keycodes
            ending_index = if player_input.medium {
                //next_attack_state.set(AttackState::Attack);
                attack_state.is_attacking = true;
                println!("Doing Medium Attack");
                animation_indeces.first = sprite_sheet_indeces.medium_first;
                animation_indeces.last = sprite_sheet_indeces.medium_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last

            }
            else {
                animation_indeces.last
            };

            // Crouching Medium Attack Logic // 
            ending_index = if player_input.medium && player_input.down {
                println!("Doing Crouching Medium Attack");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.medium_crouching_first;
                animation_indeces.last = sprite_sheet_indeces.medium_crouching_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last

            } else {
                animation_indeces.last
            };


            /* 
            // Forward Medium Attack Logic // 
            ending_index = if player_input.medium &&
                ( (player_input.right && !texture_atlas_sprite_sprite_sheet.flip_x) || (player_input.left && texture_atlas_sprite_sprite_sheet.flip_x) ) {
                println!("Doing Forward Medium Attack");
                attack_state.is_attacking = true;
                //animation_indeces.first = Sprite Sheet Indeces var first
                //animation_indeces.last = Sprite Sheet Indeces var last
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last

            } else {
                animation_indeces.last
            };    
    
            // Back Medium Attack Logic
            ending_index = if player_input.medium &&
                ( (player_input.left && !texture_atlas_sprite_sprite_sheet.flip_x) || (player_input.right && texture_atlas_sprite_sprite_sheet.flip_x) ) {
                //
                println!("Doing Back Medium Attack");
                attack_state.is_attacking = true;
                //animation_indeces.first = Sprite Sheet Indeces var first
                //animation_indeces.last = Sprite Sheet Indeces var last
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            } else {
                animation_indeces.last
            };
            */


            // Heavy Attack Logic //
            ending_index = if player_input.heavy {
                //next_attack_state.set(AttackState::Attack);
                attack_state.is_attacking = true;
                println!("Doing Heavy Attack");
                animation_indeces.first = sprite_sheet_indeces.heavy_first;
                animation_indeces.last = sprite_sheet_indeces.heavy_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            }
            else {
                animation_indeces.last
            };


            // Crouching Heavy Attack Logic // 
            ending_index = if player_input.heavy && player_input.down {
                println!("Doing Crouching Heavy Attack");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.heavy_crouching_first;
                animation_indeces.last = sprite_sheet_indeces.heavy_crouching_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last

            } else {
                animation_indeces.last
            };


            /* 
            // Forward Heavy Attack Logic // 
            ending_index = if player_input.heavy &&
                ( (player_input.right && !texture_atlas_sprite_sprite_sheet.flip_x) || (player_input.left && texture_atlas_sprite_sprite_sheet.flip_x) ) {
                println!("Doing Forward Heavy Attack");
                attack_state.is_attacking = true;
                //animation_indeces.first = Sprite Sheet Indeces var first
                //animation_indeces.last = Sprite Sheet Indeces var last
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last

            } else {
                animation_indeces.last
            };    
    
            // Back Heavy Attack Logic
            ending_index = if player_input.heavy &&
                ( (player_input.left && !texture_atlas_sprite_sprite_sheet.flip_x) || (player_input.right && texture_atlas_sprite_sprite_sheet.flip_x) ) {
                //
                println!("Doing Back Heavy Attack");
                attack_state.is_attacking = true;
                //animation_indeces.first = Sprite Sheet Indeces var first
                //animation_indeces.last = Sprite Sheet Indeces var last
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            } else {
                animation_indeces.last
            };
            */


            // Unique Attack Logic //
            ending_index = if player_input.unique {
                //next_attack_state.set(AttackState::Attack);
                attack_state.is_attacking = true;
                println!("Doing Unique Attack");
                animation_indeces.first = sprite_sheet_indeces.unique_first;
                animation_indeces.last = sprite_sheet_indeces.unique_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            }
            else {
                animation_indeces.last
            };


            // Crouching Unique Attack Logic // 
            ending_index = if player_input.unique && player_input.down {
                println!("Doing Crouching Unique Attack");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.unique_crouching_first;
                animation_indeces.last = sprite_sheet_indeces.unique_crouching_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last

            } else {
                animation_indeces.last
            };

            // Forward Unique Attack Logic // 
            ending_index = if player_input.unique &&
                ( (player_input.right && !texture_atlas_sprite_sprite_sheet.flip_x) || (player_input.left && texture_atlas_sprite_sprite_sheet.flip_x) ) {
                println!("Doing Forward Unique Attack");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.unique_forward_first;
                animation_indeces.last = sprite_sheet_indeces.unique_forward_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last

            } else {
                animation_indeces.last
            };    
    
            /* 
            // Back Unique Attack Logic
            ending_index = if player_input.unique &&
                ( (player_input.left && !texture_atlas_sprite_sprite_sheet.flip_x) || (player_input.right && texture_atlas_sprite_sprite_sheet.flip_x) ) {
                //
                println!("Doing Back Unique Attack");
                attack_state.is_attacking = true;
                //animation_indeces.first = Sprite Sheet Indeces var first
                //animation_indeces.last = Sprite Sheet Indeces var last
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            } else {
                animation_indeces.last
            };
            */

            // Special Attack Logic //
            ending_index = if player_input.special {
                //next_attack_state.set(AttackState::Attack);
                attack_state.is_attacking = true;
                println!("Doing Special Attack");
                animation_indeces.first = sprite_sheet_indeces.special_first;
                animation_indeces.last = sprite_sheet_indeces.special_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            }
            else {
                animation_indeces.last
            };


            // Crouching Special Attack Logic // 
            ending_index = if player_input.special && player_input.down {
                println!("Doing Crouching Special Attack");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.special_crouching_first;
                animation_indeces.last = sprite_sheet_indeces.special_crouching_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last

            } else {
                animation_indeces.last
            };

            // Forward Special Attack Logic // 
            ending_index = if player_input.special &&
                ( (player_input.right && !texture_atlas_sprite_sprite_sheet.flip_x) || (player_input.left && texture_atlas_sprite_sprite_sheet.flip_x) ) {
                println!("Doing Forward Special Attack");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.special_forward_first;
                animation_indeces.last = sprite_sheet_indeces.special_forward_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last

            } else {
                animation_indeces.last
            };    
    
            // Back Special Attack Logic
            ending_index = if player_input.special &&
                ( (player_input.left && !texture_atlas_sprite_sprite_sheet.flip_x) || (player_input.right && texture_atlas_sprite_sprite_sheet.flip_x) ) {
                //
                println!("Doing Back Special Attack");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.special_back_first;
                animation_indeces.last = sprite_sheet_indeces.special_back_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            } else {
                animation_indeces.last
            };


            // -- Special Moves -- //


            // try to make an if block for quarter circle forward here -- down, down-right, right + button
            //   this would actually just be in the J, K, L, or ; if blocks...
            //   maybe they could be in there own system???

            // fireball
            //   if you pressed j and your recent action vector contains S, D, j, do fireball
            ending_index = if player_input.light 
                && second_first_difference <= SPECIAL_MOVE_BUFFER_TIME 
                && third_second_difference <= SPECIAL_MOVE_BUFFER_TIME
                && recent_key_first == KeyCode::S                                               // Fix this part later -- I'm not sure if I'm using a special button or movement inputs...
                && recent_key_second == KeyCode::D
                && recent_key_third == KeyCode::J
                {
                    //next_attack_state.set(AttackState::Attack);
                    attack_state.is_attacking = true;
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

            ending_index = if player_input.medium
                && second_first_difference <= SPECIAL_MOVE_BUFFER_TIME
                && third_second_difference <= SPECIAL_MOVE_BUFFER_TIME 
                && recent_key_first == KeyCode::S
                && recent_key_second == KeyCode::D
                && recent_key_third == KeyCode::K
                {
                    //next_attack_state.set(AttackState::Attack);
                    attack_state.is_attacking = false;
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


            ending_index = if player_input.heavy
                && second_first_difference <= SPECIAL_MOVE_BUFFER_TIME 
                && third_second_difference <= SPECIAL_MOVE_BUFFER_TIME 
                && recent_key_first == KeyCode::S
                && recent_key_second == KeyCode::D
                && recent_key_third == KeyCode::L
                {
                    //next_attack_state.set(AttackState::Attack);
                    attack_state.is_attacking = true;
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

        // Jumping Attacks
        if !attack_state.is_attacking && !movement_state.is_grounded {
            
            // Light Attack Logic //
            let mut ending_index = if player_input.light {
                
                println!("Doing Jumping Light Attack");
                attack_state.is_attacking = true;
                animation_indeces.first = sprite_sheet_indeces.light_jumping_first;
                animation_indeces.last = sprite_sheet_indeces.light_jumping_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                //next_player_state.set(PlayerState::Attack);
                animation_indeces.last
            } else {
                animation_indeces.last
            };

            // Medium Attack Logic //
            // can repeat same structure, jsut with different first and last indeces and different keycodes
            ending_index = if player_input.medium {
                //next_attack_state.set(AttackState::Attack);
                attack_state.is_attacking = true;
                println!("Doing Jumping Medium Attack");
                animation_indeces.first = sprite_sheet_indeces.medium_jumping_first;
                animation_indeces.last = sprite_sheet_indeces.medium_jumping_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last

            }
            else {
                animation_indeces.last
            };

            // Heavy Attack Logic //
            ending_index = if player_input.heavy {
                //next_attack_state.set(AttackState::Attack);
                attack_state.is_attacking = true;
                println!("Doing Jumping Heavy Attack");
                animation_indeces.first = sprite_sheet_indeces.heavy_jumping_first;
                animation_indeces.last = sprite_sheet_indeces.heavy_jumping_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            }
            else {
                animation_indeces.last
            };

            // Unique Attack Logic //
            ending_index = if player_input.unique {
                //next_attack_state.set(AttackState::Attack);
                attack_state.is_attacking = true;
                println!("Doing Jumping Unique Attack");
                animation_indeces.first = sprite_sheet_indeces.unique_jumping_first;
                animation_indeces.last = sprite_sheet_indeces.unique_jumping_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            }
            else {
                animation_indeces.last
            };

            // Special Attack Logic //
            ending_index = if player_input.special {
                //next_attack_state.set(AttackState::Attack);
                attack_state.is_attacking = true;
                println!("Doing Jumping Special Attack");
                animation_indeces.first = sprite_sheet_indeces.special_jumping_first;
                animation_indeces.last = sprite_sheet_indeces.special_jumping_last;
                texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
                animation_indeces.last
            }
            else {
                animation_indeces.last
            };


        }




    }
}




// Add commands here, in order to despawn hitboxes from the current action
pub fn player_reset_to_neutral(
    mut player_query: Query<(&mut AnimationIndices, &mut TextureAtlasSprite, &mut AttackState, &mut MovementState, &SpriteSheetIndeces), With<Player>>,
    //attack_state: Res<State<AttackState>>,
    //mut next_attack_state: ResMut<NextState<AttackState>>,
) {
    //if let Ok((mut animation_indeces, mut texture_atlas_sprite_sprite_sheet)) = player_query.get_single_mut() {
    for (mut animation_indeces, mut texture_atlas_sprite_sprite_sheet, mut attack_state, mut movement_state, sprite_sheet_indeces) in player_query.iter_mut() {
        if texture_atlas_sprite_sprite_sheet.index == animation_indeces.last && (attack_state.is_attacking) {
            // reset animation indeces to the default for the particular state
            animation_indeces.first = sprite_sheet_indeces.idle_first;
            animation_indeces.last = sprite_sheet_indeces.idle_last;
            texture_atlas_sprite_sprite_sheet.index = animation_indeces.first;
            //next_player_state.set(PlayerState::Grounded);

            println!("resetting");

            // reset to neutral state
            //next_attack_state.set(AttackState::Neutral);
            attack_state.is_attacking = false;
        }

        //if texture_atlas_sprite_sprite_sheet.index == animation_indeces.last && movement_state.is_walking {
        //    // i want to keep looping if you are holding walk
        //    // basically if you are holding left or right i want to not reset to neutral
        //    println!("We're Walking");
        //    movement_state.is_walking = false;
        //}


    }
}


pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    //if let Ok(mut player_transform) = player_query.get_single_mut() {
    for mut player_transform in player_query.iter_mut() {    
        let window = window_query.get_single().unwrap();
        let half_player_size_horizontal = PLAYER_SIZE / 2.0;
        let half_player_size_vertical   = MARISA_PLAYER_SIZE / 2.0;
        let x_pos_min = 0.0 + half_player_size_horizontal;
        let x_pos_max = window.width() - half_player_size_horizontal;
        let y_pos_min = 0.0 + half_player_size_vertical;
        let y_pos_max = window.height() - half_player_size_vertical;
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

pub fn _debug_player_state(
    player_query: Query<(&MovementState, &AttackState, &PlayerNumber), With<Player>>,
) {
    for (movement_state, attack_state, player_number) in player_query.iter() {
        println!("printing state for player {} -- is_grounded: {}, is_walking: {}, is_attacking: {}", player_number.player_number, movement_state.is_grounded, movement_state.is_walking, attack_state.is_attacking);

    }
}

pub fn _debug_player_velocity(
    player_query: Query<(&JumpVelocity, &PlayerNumber), With<Player>>,
) {
    for (jump_velocity, player_number) in player_query.iter() {
        println!("printing jump velocity for player {}, horizontal_velocity: {}, vertical_velocity: {}", player_number.player_number, jump_velocity.horizontal_velocity, jump_velocity.vertical_velocity);
    }
}

pub fn player_flip(
    //
    mut player_query: Query<(&Transform, &PlayerNumber, &mut TextureAtlasSprite), With<Player>>,
    //keyboard_input: Res<Input<KeyCode>>,
    //window_query: Query<&Window, With<PrimaryWindow>>,
    //simulation_state: Res<State<SimulationState>>,
) {
    //
    // this assignment to storage transform should only happen once -- but each time the system is called it resets to the window value
    let mut storage_transform_player_one: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    let mut storage_transform_player_two: Vec3 = Vec3::new(0.0, 0.0, 0.0);

    for (player_transform, player_number, mut player_atlas_sprite) in player_query.iter_mut() {
        if player_number.player_number == 1 {
            storage_transform_player_one = player_transform.translation;
        }
        if player_number.player_number == 2 {
            storage_transform_player_two = player_transform.translation;
        }


        /*
        
        
        println!("test difference: {}", storage_transform_player_two.x - storage_transform_player_one.x);

        let mut distance_from_player_two = 0.0;
        let mut distance_from_player_one = 0.0;

        if player_number.player_number == 1 {
            distance_from_player_two = storage_transform_player_two.x - player_transform.translation.x;
            println!("distance_from_player_two for player {}: {}", player_number.player_number, distance_from_player_two);
        }
        if player_number.player_number == 2 {
            distance_from_player_one = storage_transform_player_one.x - player_transform.translation.x;
            println!("distance_from_player_one for player {}: {}", player_number.player_number, distance_from_player_one);
        }

        // something is off here -- distance from player one seems to work fine,
        //   but distance from player two doesn't

        */

        /*
        
        if distance_from_other_player > 0.0 {
            // TextureAtlasSprite (player_atlas_sprite) --> flip_x to false
            player_atlas_sprite.flip_x = false;
        } else {
            // TextureAtlasSprite (player_atlas_sprite) --> flip_x to true
            player_atlas_sprite.flip_x = true;
        }
        */

        
        //println!("distance_from_other_player for two: {}", storage_transform_player_two.x);



    }

    for (player_transform, player_number, mut player_atlas_sprite) in player_query.iter_mut() {
        //
        if player_number.player_number == 1 {
            if player_transform.translation.x < storage_transform_player_two.x {
                // TextureAtlasSprite --> flip_x to false
                //println!("player 1 to the left of player 2");
                player_atlas_sprite.flip_x = false;
            } else {
                // TextureAtlasSprite --> flip_x to true
                //println!("player 1 to the right of player 2");
                player_atlas_sprite.flip_x = true;
            }
        }
        if player_number.player_number == 2 {
            if player_transform.translation.x < storage_transform_player_one.x {
                // TextureAtlasSprite --> flip_x to false
                //println!("player 2 to the left of player 1");
                player_atlas_sprite.flip_x = false;
            } else {
                // TextureAtlasSprite --> flip_x to true
                //println!("player 2 to the right of player 1");
                player_atlas_sprite.flip_x = true;
            }
        }
    }

    /*
    
    let window = window_query.get_single().unwrap();
    let mut storage_transform: Transform = Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0);
    for (player_transform, player_number, mut player_atlas_sprite) in player_query.iter_mut() {
        println!("storage_transform x pos: {:?}", storage_transform.translation.x);
        if player_transform.translation.x < storage_transform.translation.x {
            // TextureAtlasSprite --> flip_x to true
            player_atlas_sprite.flip_x = true;
            println!("player {} flipped: {}", player_number.player_number, player_atlas_sprite.flip_x);
        } else {
            // TextureAtlasSprite --> flip_x to false
            player_atlas_sprite.flip_x = false;
            println!("player {} didn't flip: {}", player_number.player_number, player_atlas_sprite.flip_x);
        }

        storage_transform.translation.x = player_transform.translation.x;
    }
    */
}




// Check if the player is grounded
pub fn ground_check(
    mut player_query: Query<(&Transform, &EntitySizeCollision, &mut JumpVelocity, &mut MovementState, &PlayerNumber), With<Player>>,
    floor_query: Query<(&Transform, &EntitySizeCollision), With<Floor>>,
    //player_state: Res<State<PlayerState>>,
    //mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    //if let Ok((player_transform, player_collision, mut jump_velocity)) = player_query.get_single_mut() {
    for (player_transform, player_collision, mut jump_velocity, mut movement_state, player_number) in player_query.iter_mut() {
        for (floor_transform, floor_collision) in floor_query.iter() {
            // check collision get boolean
            //let distance = player_transform.translation.distance(floor_transform.translation);
            //let _horizontal_distance = player_transform.translation.x - floor_transform.translation.x;
            let vertical_distance = player_transform.translation.y - floor_transform.translation.y;
            //let _horizontal_player_length = player_collision.horizontal_entity_size / 2.0;
            let vertical_player_length = player_collision.vertical_entity_size / 2.0;
            //let _horizontal_floor_length = floor_collision.horizontal_entity_size / 2.0;
            let vertical_floor_length = floor_collision.vertical_entity_size / 2.0;
            
        

            // if (horizontal_distance < horizontal_player_length + horizontal_floor_length)
            if vertical_distance < vertical_player_length + vertical_floor_length {

                if !movement_state.is_grounded {
                    movement_state.is_grounded = true;
                    // we also want to force horizontal velocity to be 0 here
                    jump_velocity.horizontal_velocity = 0.0;
                }

                /* 
                // if boolean is true {}            
                if player_state.0 == PlayerState::Air {
                    // switch to ground state
                    next_player_state.set(PlayerState::Grounded);
                    //println!("I'm grounded");


                    // we also want to force horizontal velocity to be 0 here
                    jump_velocity.horizontal_velocity = 0.0;
                }
                */
            } else {
                // if boolean is false ()

                if movement_state.is_grounded {
                    movement_state.is_grounded = false;
                }

                /*
                if player_state.0 == PlayerState::Grounded {
                    // switch to air state
                    next_player_state.set(PlayerState::Air);
                    //println!("I'm in the air");
                }
                 */

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
        //println!("{:?}", player_action_state_vector.action_vector);
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
                    //println!("Key release: {:?} ({})", keyboard_event.key_code, keyboard_event.scan_code);
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



// -- Section for re doing input system -- //

// mut player_query: Query<(&mut ActionStateVector, &mut NegativeEdgeStateVector), With<Player>>,


pub fn testing_new_input_system(
    //
    mut player_query: Query<(&mut PlayerInput, &InputBinding), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    //
    for (mut player_inputs, player_keybinds) in player_query.iter_mut() {
        player_inputs.up = keyboard_input.pressed(player_keybinds.up_bind);
        player_inputs.down = keyboard_input.pressed(player_keybinds.down_bind);
        player_inputs.left = keyboard_input.pressed(player_keybinds.left_bind);
        player_inputs.right = keyboard_input.pressed(player_keybinds.right_bind);
        player_inputs.light = keyboard_input.pressed(player_keybinds.light_bind);
        player_inputs.medium = keyboard_input.pressed(player_keybinds.medium_bind);
        player_inputs.heavy = keyboard_input.pressed(player_keybinds.heavy_bind);
        player_inputs.unique = keyboard_input.pressed(player_keybinds.unique_bind);
        player_inputs.special = keyboard_input.pressed(player_keybinds.special_bind);
        /* 
        println!("player_inputs: {}", player_inputs.up);
        println!("player_inputs: {}", player_inputs.down);
        println!("player_inputs: {}", player_inputs.left);
        println!("player_inputs: {}", player_inputs.right);
        println!("player_inputs: {}", player_inputs.light);
        println!("player_inputs: {}", player_inputs.medium);
        println!("player_inputs: {}", player_inputs.heavy);
        println!("player_inputs: {}", player_inputs.unique);
        println!("player_inputs: {}", player_inputs.special);
        */
    }
}



// -- Section for re doing input system -- //