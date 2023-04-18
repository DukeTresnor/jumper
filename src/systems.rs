// main systems
// spawn a camera
//     needs a window and commands
// handle main transitions:
//     transition to game state
//     transition to main menu state
// exit game
// handle game over event



// use statements

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::app::AppExit;

use crate::AppState;
use crate::game::SimulationState;
use crate::events::*;



// Systems
pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn(
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
    );
    //println!("I spawned a camera");
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    // resource to access current state
    app_state: Res<State<AppState>>,
    // mutable resource to access the next app state
    mut next_app_state: ResMut<NextState<AppState>>,

) {
    if keyboard_input.just_pressed(KeyCode::G) {
        if app_state.0 != AppState::Game {
            next_app_state.set(AppState::Game);
            println!("I transitioned to the game state");
        }

    }
    
}

pub fn transition_to_main_menu_state(
    keyboard_input: Res<Input<KeyCode>>,
    // resource to access current state
    app_state: Res<State<AppState>>,
    // mutable resource to access the next app state
    mut next_app_state: ResMut<NextState<AppState>>,
    // mutable resource to access the next simulation state
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if app_state.0 != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            next_simulation_state.set(SimulationState::Paused);
            println!("I transitioned to the main menu state");
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    // mutable resource to access the AppExit event writer
    //    this sends events of type AppExit
    //    goal is once Escape is pressed, send an event of type AppExit
    //    this event should close the window, as it exits the "App" which is the
    //    process that is keeping the window open in the first place
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
        println!("I exited the game and closed the window");
    }
}

pub fn handle_game_over_event(
    // mutable resource to access the next app state
    mut next_app_state: ResMut<NextState<AppState>>,
    // mutable EventReader that looks at GameOver events
    mut game_over_event_reader: EventReader<GameOver>,
) {
    // constantly loop over every event contained in the game over event reader
    for event in game_over_event_reader.iter() {
        println!("Your final score is: {}", event.score.to_string());
        // set the next state to be the GameOver state
        //    idea is once the game over event is found, exit the current AppState and
        //      go to the GameOver state
        next_app_state.set(AppState::GameOver);
    }
}