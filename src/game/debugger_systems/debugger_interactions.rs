// game / debugger_systems / debugger_interactions.rs

use bevy::prelude::*;

use crate::game::SimulationState;
use crate::game::resources::*;



// Debugger -- enter and exit with Q, only when game is running
// Systems to make
//   spawn debugger menu  <-- OnEnter SimulationState::Debugger
//   show active hitboxes <-- Update run_if(in_state(SimulationState::Debugger))
//   draw box button      <-- Update run_if(in_state(SimulationState::Debugger))
//     once button is pressed, enable click and drag on trackpad
//   save box button      <-- Update run_if(in_state(SimulationState::Debugger))
//     once button is pressed, disable click and drag on trackpad
//     write transform of hitbox and its size into the character's json file
//     saves drawn boxes into json file -- this file is what the spawn player system looks at
//   advance by 1 frame

// Json File
//   create a json file
//   make a test system that reads from that json file
//   make a test system that writes to that file


/*
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

    // Temporary
    if keyboard_input.just_pressed(KeyCode::Q) {
        if simulation_state.get() == &SimulationState::Debugger {
            next_simulation_state.set(SimulationState::Running);
            println!("Exiting Debugger and entering Running");
        }
    }
}
*/
pub fn temp_advance_one_frame(
    //
    time: Res<Time>,
    //timer: ResMut<Timer>,
    mut advance_one_frame_mode: ResMut<AdvanceOneFrameMode>,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    //
    let mut current_time_elapsed: f32 = time.elapsed_seconds();
    //let mut not_advance_one_frame_mode: bool = true;


    
    if simulation_state.get() == &SimulationState::Debugger {
        if keyboard_input.just_pressed(KeyCode::A) {
            next_simulation_state.set(SimulationState::Running);
            
            println!("current_time_elapsed: {}, then delta tiem: {}", current_time_elapsed, time.delta().as_secs_f32());

            advance_one_frame_mode.should_advance_one_frame = true;

            // wait 1 frame?
            //println!("{:?}", time.elapsed().as_secs_f32() * (1.0 / 60.0));
            //if (time.elapsed().as_secs_f32() * (60.0 / 60.0)) as u64 % 2 == 0 {
            //    println!("Switching to Running for one frame, mod result: {}", (time.elapsed().as_secs_f32() as u64) % 2);
            //    next_simulation_state.set(SimulationState::Debugger);
            //}

            //next_simulation_state.set(SimulationState::Debugger);
        } //else if (time.elapsed().as_secs_f32() * (60.0 / 60.0)) as u64 % 2 == 1 {
        //    println!("dkfjhbvhdfbvjdfvdhj");
        //    next_simulation_state.set(SimulationState::Debugger);
        //}
    } else if simulation_state.get() == &SimulationState::Running && advance_one_frame_mode.should_advance_one_frame{
        next_simulation_state.set(SimulationState::Debugger);
        advance_one_frame_mode.should_advance_one_frame = false;
        
    }
    //println!("advance_one_frame_mode: {}", advance_one_frame_mode);

    //if simulation_state.get() == &SimulationState::Running && !not_advance_one_frame_mode {
    //    //
    //    next_simulation_state.set(SimulationState::Debugger);
    //    println!("Re-entering Debugger");
    //    not_advance_one_frame_mode = true;
    //}

    println!("Current State is? : {:?}", simulation_state.get())
;

}