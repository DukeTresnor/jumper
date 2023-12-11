use bevy::prelude::*;


pub const MARISA_PLAYER_SIZE: f32 = 144.0;

pub const MARISA_HEALTH: f32 = 100.0;

pub const MARISA_HURT_OFFSET: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const MARISA_HURT_SIZE: Vec2 = Vec2::new(32.0, 96.0);                               // 2 * 16 , 6 * 16
pub const MARISA_HURT_DURATION: f32 = 0.15;
pub const MARISA_HURT_STARTUP: f32 = 0.5;
pub const MARISA_HURT_ACTIVE: f32 = 0.5;
pub const MARISA_HURT_COOLDOWN: f32 = 0.5;

pub const MARISA_HURT_CROUCHING_OFFSET: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const MARISA_HURT_CROUCHING_SIZE: Vec2 = Vec2::new(48.0, 80.0);                     // 3 * 16 , 5 * 16
pub const MARISA_HURT_CROUCHING_DURATION: f32 = 0.167;
pub const MARISA_HURT_CROUCHING_STARTUP: f32 = 0.5;
pub const MARISA_HURT_CROUCHING_ACTIVE: f32 = 0.5;
pub const MARISA_HURT_CROUCHING_COOLDOWN: f32 = 0.67;

// Light
pub const MARISA_LIGHT_HURT_OFFSET: Vec3 = Vec3::new(24.0, 5.0, 0.0);                 // 1.5 * 16 , 5 
pub const MARISA_LIGHT_HURT_SIZE: Vec2 = Vec2::new(48.0, 10.0);                         // 3 * 16, 10
pub const MARISA_LIGHT_HURT_DURATION: f32 = 0.1;                                             // 6 frames
pub const MARISA_LIGHT_HURT_STARTUP: f32 = 0.03;
pub const MARISA_LIGHT_HURT_ACTIVE: f32 = 0.03;
pub const MARISA_LIGHT_HURT_COOLDOWN: f32 = 0.04;

pub const MARISA_LIGHT_HIT_OFFSET: Vec3 = Vec3::new(20.0, 5.0, 0.0);                 // 1.5 * 16 , 5
pub const MARISA_LIGHT_HIT_SIZE: Vec2 = Vec2::new(40.0, 10.0);                          // 2.5 * 16, 10
pub const MARISA_LIGHT_HIT_STARTUP: f32 = 0.03;                                              // 2 frames
pub const MARISA_LIGHT_HIT_ACTIVE: f32 = 0.05;                                             // 3 frames
pub const MARISA_LIGHT_HIT_COOLDOWN: f32 = 0.02;                                                 // 1 frame

pub const MARISA_LIGHT_HIT_DAMAGE: f32 = 5.0;
// Light


// Light Crouching
pub const MARISA_LIGHT_CROUCHING_HURT_OFFSET: Vec3 = Vec3::new(32.0, 8.0, 0.0);                 // 1.5 * 16 , 5 
pub const MARISA_LIGHT_CROUCHING_HURT_SIZE: Vec2 = Vec2::new(32.0, 32.0);                         // 3 * 16, 10
pub const MARISA_LIGHT_CROUCHING_HURT_DURATION: f32 = 0.13;                                             // 6 frames
pub const MARISA_LIGHT_CROUCHING_HURT_STARTUP: f32 = 0.03;                                              // 2 frames
pub const MARISA_LIGHT_CROUCHING_HURT_ACTIVE: f32 = 0.03;                                             // 3 frames
pub const MARISA_LIGHT_CROUCHING_HURT_COOLDOWN: f32 = 0.07;         


pub const MARISA_LIGHT_CROUCHING_HIT_OFFSET: Vec3 = Vec3::new(32.0, 8.0, 0.0);                 // 1.5 * 16 , 5
pub const MARISA_LIGHT_CROUCHING_HIT_SIZE: Vec2 = Vec2::new(32.0, 32.0);                          // 2.5 * 16, 10
pub const MARISA_LIGHT_CROUCHING_HIT_STARTUP: f32 = 0.03;                                              // 2 frames
pub const MARISA_LIGHT_CROUCHING_HIT_ACTIVE: f32 = 0.03;                                             // 3 frames
pub const MARISA_LIGHT_CROUCHING_HIT_COOLDOWN: f32 = 0.07;                                                 // 1 frame

pub const MARISA_LIGHT_CROUCHING_HIT_DAMAGE: f32 = 5.0;
// Light Crouching




// Medium
pub const MARISA_MEDIUM_HURT_OFFSET: Vec3 = Vec3::new(32.0, 0.0, 0.0);                 // 1.5 * 16 , 5 
pub const MARISA_MEDIUM_HURT_SIZE: Vec2 = Vec2::new(64.0, 96.0);                         // 3 * 16, 10
pub const MARISA_MEDIUM_HURT_DURATION: f32 = 0.23;                                             // 14 frames

pub const MARISA_MEDIUM_HIT_OFFSET: Vec3 = Vec3::new(32.0, 0.0, 0.0);                 // 1.5 * 16 , 5
pub const MARISA_MEDIUM_HIT_SIZE: Vec2 = Vec2::new(64.0, 96.0);                          // 2.5 * 16, 10
pub const MARISA_MEDIUM_HIT_STARTUP: f32 = 0.07;                                              // 4 frames
pub const MARISA_MEDIUM_HIT_ACTIVE: f32 = 0.05;                                             // 3 frames
pub const MARISA_MEDIUM_HIT_COOLDOWN: f32 = 0.11;                                                 // 7 frames


pub const MARISA_MEDIUM_HIT_DAMAGE: f32 = 5.0;
// Medium


/* 
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

*/