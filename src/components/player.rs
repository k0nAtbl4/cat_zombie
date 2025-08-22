use std::default;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub struct StartEnd {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub idle: StartEnd,    // first, last для idle
    pub walking: StartEnd, // first, last для walking
}



#[derive(Component, Debug)]
pub struct AnimationTimer{
    pub frame_timer: Timer,
    pub pause_timer: Timer,
    pub is_in_pause: bool,
}

#[derive(Component, Default, PartialEq)]
pub enum AnimationState {
    #[default]
    Idle,
    Walking,
}



#[derive(Component, PartialEq)]
pub enum Direction {
    Right,
    Left,
}