// game / debugger_stles.rs

use bevy::prelude::*;

pub const DEBUG_NORMAL_BUTTON_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

pub const DEBUG_SUB_BUTTON_COLOR: Color = Color::rgb(0.15, 0.30, 0.15);

pub const DEBUG_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.40, 0.25);

pub const DEBUG_HOVERED_BUTTON: Color = Color::rgb(0.35, 0.50, 0.35);

/*  Eventually figure out how to do default constants or fill in default values, so that you can use this constant within your code
pub const BUTTON_STYLE: Style = Style {
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    width: Val::Px(200.0),
    height: Val::Px(80.0),
    ..default()
    
};
*/