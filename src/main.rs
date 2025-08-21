use bevy::prelude::*;


mod components;
mod systems;




use crate::systems::{player::player_movement, setup::setup};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}
    
