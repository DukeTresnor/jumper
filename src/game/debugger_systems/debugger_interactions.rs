// game / debugger_systems / debugger_interactions.rs

use bevy::prelude::*;

use crate::game::SimulationState;
use crate::game::resources::*;



// Debugger -- enter and exit with Q, only when game is running
// Systems to make
//   spawn debugger menu  <-- OnEnter SimulationState::Debugger
//   despawn debugger menu <-- OnExit  SimulationState::Debugger
//   show active hitboxes <-- Update run_if(in_state(SimulationState::Debugger))
//   draw box button      <-- Update run_if(in_state(SimulationState::Debugger))
//     once button is pressed, enable click and drag on trackpad
//   save box button      <-- Update run_if(in_state(SimulationState::Debugger))
//     once button is pressed, disable click and drag on trackpad
//     write transform of hitbox and its size into the character's json file
//     saves drawn boxes into json file -- this file is what the spawn player system looks at
//   advance by 1 frame                                                                         --> Done

// Json File
//   create a json file                                                                         --> First Pass
//   make a test system that reads from that json file
//   make a test system that writes to that file


pub fn advance_one_frame(
    time: Res<Time>,
    mut advance_one_frame_mode: ResMut<AdvanceOneFrameMode>,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) { 
    if simulation_state.get() == &SimulationState::Debugger {

        /*
        Unneccessary timer logic, edit out later most likely but useful for reference for the idea?
        // tick the timer
        advance_one_frame_mode.frame_timer.tick(time.delta());


        println!("time elapsed for timer: {:?}", advance_one_frame_mode.frame_timer.elapsed());

        if advance_one_frame_mode.frame_timer.finished() {
            println!("Done with timer");
            advance_one_frame_mode.frame_timer.reset();
        }
         */

        if keyboard_input.just_pressed(KeyCode::Z) {
            next_simulation_state.set(SimulationState::Running);

            advance_one_frame_mode.should_advance_one_frame = true; 


        }
    } else if simulation_state.get() == &SimulationState::Running && advance_one_frame_mode.should_advance_one_frame {
        next_simulation_state.set(SimulationState::Debugger);
        advance_one_frame_mode.should_advance_one_frame = false;

    }


}