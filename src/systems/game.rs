use std::ops::Deref;

use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output;
use amethyst::audio::Source;
use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::ui::UiText;
use log::info;

use crate::components::{BallComponent, BoundingBoxComponent};
use crate::{ScoreBoard, ScoreText, Sounds, ARENA_WIDTH};

fn play_score_sound(sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.score_sfx) {
            output.play_once(sound, 1.0);
        }
    }
}

#[derive(Default, SystemDesc)]
pub struct ScoreSystem;

impl<'s> System<'s> for ScoreSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, BallComponent>,
        WriteStorage<'s, BoundingBoxComponent>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, Sounds>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (
            mut balls,
            mut bounds,
            mut transforms,
            mut ui_text,
            mut scores,
            score_text,
            storage,
            sounds,
            audio_output,
        ): Self::SystemData,
    ) {
        for (ball, transform, ball_bounds) in (&mut balls, &mut transforms, &mut bounds).join() {
            let bounds_center = transform.translation() + ball_bounds.center();
            let half_width = ball_bounds.extents().x;

            // check for score, update score text if so
            let did_score = if bounds_center.x <= half_width {
                scores.score_right = (scores.score_right + 1).min(999);

                if let Some(text) = ui_text.get_mut(score_text.p2_score) {
                    text.text = scores.score_right.to_string();
                }
                true
            } else if bounds_center.x >= ARENA_WIDTH - half_width {
                scores.score_left = (scores.score_left + 1).min(999);

                if let Some(text) = ui_text.get_mut(score_text.p1_score) {
                    text.text = scores.score_left.to_string();
                }
                true
            } else {
                false
            };

            // if someone scored, move the ball back to the center
            // and reverse its direction
            if did_score {
                transform.set_translation_x(ARENA_WIDTH * 0.5);
                ball.velocity[0] = -ball.velocity[0];
                play_score_sound(&*sounds, &storage, audio_output.as_ref().map(|o| o.deref()));

                info!(
                    "Score: | {:^3} | {:^3} |",
                    scores.score_left, scores.score_right
                );
            }
        }
    }
}
