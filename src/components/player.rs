use std::default;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player;


#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Debug)]
pub struct AnimationTimer{
    pub frame_timer: Timer,
    pub pause_timer: Timer,
    pub is_in_pause: bool,
}


#[derive(Component, PartialEq)]
pub enum Direction {
    Right,
    Left,
}