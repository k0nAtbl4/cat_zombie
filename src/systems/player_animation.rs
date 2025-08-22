use bevy::prelude::*;

use crate::components::animation_cat::{AnimationIndices, AnimationTimer};


pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
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
                    atlas.index = if atlas.index == indices.last {
                        indices.first
                    } else {
                        atlas.index + 1

                    };
                        if atlas.index == indices.last {
                            timer.is_in_pause = true;
                            timer.pause_timer.reset();
                        }
                }
            }
        }
    }
}