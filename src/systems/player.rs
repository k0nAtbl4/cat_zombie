use bevy::prelude::*;
use crate::{components::player::{Direction, Player}, resources};


use resources::sound::MySound;





const PLAYER_SPEED: f32 = 500.0;

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Direction, &mut Sprite), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut transform, mut looking_direction, mut sprite)) = query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
            *looking_direction = Direction::Left;
        }   
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
            *looking_direction = Direction::Right;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }


        if *looking_direction == Direction::Left {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }
        // if *looking_direction == Direction::Left {
        //     transform.scale.x = if transform.scale.x > 0.0 { transform.scale.x*-1.0 } else { transform.scale.x }; // Отражаем по горизонтали
        // } else {
        //     transform.scale.x = if transform.scale.x > 0.0 { transform.scale.x } else { transform.scale.x *-1.0};  // Нормальное отображение
        // }

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
