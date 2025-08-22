use bevy::{prelude::*, render::render_resource::encase::private::SizeValue};

use crate::components::{animation_cat::{AnimationIndices, AnimationTimer}, player::{Player, Velocity}};


// pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>,) {
//     commands.spawn(Camera2d);
//     commands.spawn((
//         Sprite::from_image(asset_server.load("cat/blink/Sprite-0003.png")),
//         Transform{
//             translation: Vec3::new(0.0, 0.0, 0.0),
//             scale: Vec3::new(3.,3.,1.),
//             ..Default::default()
//         },
//         Player,
//         Velocity{x: 0.0, y: 0.0},
//     ));
// }


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("cat/animation-0001.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(100), 5, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 4 };

    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
        ),  
        Transform::from_scale(Vec3::splat(5.0)),
        animation_indices,
        AnimationTimer{
            frame_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            pause_timer: Timer::from_seconds(1.6, TimerMode::Repeating),
            is_in_pause: true,
        },
        Player,
        Velocity { x: 0.0, y: 0.0 },
    ));
}