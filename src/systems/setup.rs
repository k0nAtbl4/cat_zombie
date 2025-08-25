use bevy::{prelude::*, render::render_resource::encase::private::SizeValue};

use crate::components::player::{AnimationIndices, AnimationState, AnimationTimer, Direction, Player, StartEnd};


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
    let texture = asset_server.load("cat/animation_player.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(100), 13, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { idle: StartEnd { first: 0, last: 4 }, walking: StartEnd { first: 5, last: 12 }};

    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.idle.first,
            },
        ),  
        Transform::from_scale(Vec3::splat(2.1)),
        animation_indices,
        AnimationTimer{
            frame_timer: Timer::from_seconds(0.085, TimerMode::Repeating),
            pause_timer: Timer::from_seconds(1.6, TimerMode::Repeating),
            is_in_pause: true,
        },
        Player,
        Direction::Right,
        AnimationState::Idle,
    ));
}