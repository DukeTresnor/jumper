// game/player/systems.rs


use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::components::*;
use crate::game::player::components::*;
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


