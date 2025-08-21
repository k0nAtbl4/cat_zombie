use bevy::{prelude::*, render::render_resource::encase::private::SizeValue};

use crate::components::player::{Player, Velocity};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>,) {
    commands.spawn(Camera2d);
    commands.spawn((
        Sprite::from_image(asset_server.load("Sprite-0003.png")),
        Transform{
            translation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(5.,5.,1.),
            ..Default::default()
        },
        Player,
        Velocity{x: 0.0, y: 0.0},
    ));
}
