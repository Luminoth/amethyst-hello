use std::ops::Deref;

use amethyst::assets::AssetStorage;
use amethyst::audio::output::Output;
use amethyst::audio::Source;
use amethyst::core::{SystemDesc, Transform};
use amethyst::derive::SystemDesc;
use amethyst::ecs::prelude::*;
use amethyst::ui::UiText;
use log::info;

use crate::components::{BallComponent, BoundingBoxComponent, PhysicalComponent};
use crate::{ScoreBoard, ScoreText, SoundEffects, ARENA_WIDTH};

fn play_score_sound(
    sound_effects: &SoundEffects,
    storage: &AssetStorage<Source>,
    output: Option<&Output>,
) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sound_effects.score) {
            output.play_once(sound, 1.0);
        }
    }
}

#[derive(Default, SystemDesc)]
pub struct ScoreSystem;

impl<'s> System<'s> for ScoreSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, BallComponent>,
        WriteStorage<'s, PhysicalComponent>,
        WriteStorage<'s, BoundingBoxComponent>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, ScoreText>,
        Read<'s, AssetStorage<Source>>,
        ReadExpect<'s, SoundEffects>,
        Option<Read<'s, Output>>,
    );

    fn run(
        &mut self,
        (
            balls,
            mut physicals,
            mut bounds,
            mut transforms,
            mut ui_text,
            mut scores,
            score_text,
            storage,
            sound_effects,
            audio_output,
        ): Self::SystemData,
    ) {
        for (_ball, ball_physical, ball_transform, ball_bounds) in
            (&balls, &mut physicals, &mut transforms, &mut bounds).join()
        {
            let bounds_center = ball_transform.translation() + ball_bounds.center();
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
                ball_transform.set_translation_x(ARENA_WIDTH * 0.5);
                ball_physical.velocity.x = -ball_physical.velocity.x;
                play_score_sound(
                    &*sound_effects,
                    &storage,
                    audio_output.as_ref().map(|o| o.deref()),
                );

                info!(
                    "Score: | {:^3} | {:^3} |",
                    scores.score_left, scores.score_right
                );
            }
        }
    }
}
