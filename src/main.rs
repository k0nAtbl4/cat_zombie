use bevy::prelude::*;


mod components;
mod systems;
mod resources;



use crate::{resources::sound::setup_sound_resources, systems::{player::{play_sound_on_space, player_movement}, player_animation::animate_sprite, setup::setup}};



fn main() {
    App::new()
        
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
//        .insert_resource(Msaa::Off)

        .add_systems(Startup,setup_sound_resources)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement,play_sound_on_space, animate_sprite))
        .run();
}
    
