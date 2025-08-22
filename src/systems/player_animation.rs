use std::{process::id, time::Duration};

use bevy::prelude::*;

use crate::components::player::{AnimationIndices, AnimationState, AnimationTimer};
use rand::Rng;




pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &AnimationState, &mut Sprite)>,
) {
        for (indices, mut timer, state, mut sprite) in &mut query {
        if state == &AnimationState::Idle {
        if timer.is_in_pause {
            timer.pause_timer.tick(time.delta());
            if timer.pause_timer.just_finished() {
                timer.is_in_pause = false;
                timer.frame_timer.reset();
            }
        } else {     
            timer.frame_timer.tick(time.delta());
            if timer.frame_timer.just_finished() {
                if let Some(atlas) = &mut sprite.texture_atlas {
                    atlas.index = if atlas.index == indices.idle.first {
                        indices.idle.last
                    } else {
                        atlas.index + 1

                    };
                        if atlas.index == indices.idle.last {
                            timer.is_in_pause = true;
                            // let z =rand::thread_rng().gen_range(0.4..6.0);
                            timer.pause_timer.set_duration(Duration::from_secs_f64( rand::thread_rng().gen_range(0.4..6.0) ));
                        }
                }
             }
        }
        }
        if state == &AnimationState::Walking {
        timer.frame_timer.tick(time.delta());
        if timer.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                let (first, last) = match state {
                    AnimationState::Idle => (indices.idle.first,indices.idle.last),
                    AnimationState::Walking => (indices.walking.first,indices.walking.last),
                };
                
                // Обновляем кадр анимации
                atlas.index = if atlas.index >= last {
                    first
                } else {
                    atlas.index + 1
                };
            }
        }
        }
    }










}