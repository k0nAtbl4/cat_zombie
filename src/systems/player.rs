use bevy::prelude::*;
use crate::{components::player::Player, resources};


use resources::sound::MySound;


const PLAYER_SPEED: f32 = 500.0;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_secs();
    }
}


pub fn play_sound_on_space(input: Res<ButtonInput<KeyCode>>, audio: Res<MySound>, mut commands: Commands) {
    if input.just_pressed(KeyCode::Space) {
        commands.spawn(AudioPlayer::new(
            audio.sound.clone(),
        ));
    }
}
