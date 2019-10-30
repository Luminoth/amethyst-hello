mod components;
mod states;
mod systems;

use std::iter::Cycle;
use std::path::PathBuf;
use std::time::Duration;
use std::vec::IntoIter;

use amethyst::audio::{AudioBundle, SourceHandle};
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::core::transform::TransformBundle;
use amethyst::ecs::prelude::*;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::prelude::*;
use amethyst::renderer::plugins::{RenderFlat2D, RenderToWindow};
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::RenderingBundle;
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::utils::application_root_dir;
use amethyst_imgui::RenderImgui;

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_HEIGHT: f32 = 16.0;

pub const BALL_RADIUS: f32 = 2.0;

const BOUNCE_SOUND: &str = "audio/sfx/bounce.ogg";
const SCORE_SOUND: &str = "audio/sfx/score.ogg";

const MUSIC_TRACKS: &[&str] = &[
    "audio/music/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
    "audio/music/Computer_Music_All-Stars_-_Albatross_v2.ogg",
];

pub struct Sounds {
    pub score_sfx: SourceHandle,
    pub bounce_sfx: SourceHandle,
}

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

#[derive(Default)]
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

fn init_logging() -> amethyst::Result<()> {
    // create the logging directory
    let logdir = PathBuf::new().join("var").join("logs");
    std::fs::create_dir_all(&logdir)?;

    amethyst::start_logger(amethyst::LoggerConfig {
        level_filter: amethyst::LogLevelFilter::Debug,
        log_file: Some(logdir.join("amethyst-hello.log")),
        ..Default::default()
    });

    Ok(())
}

fn main() -> amethyst::Result<()> {
    init_logging()?;

    let app_root = application_root_dir()?;

    // load configs
    let display_config_path = app_root.join("etc").join("display.ron");
    let binding_path = app_root.join("etc").join("bindings.ron");

    // create bundles
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;
    let audio_bundle = AudioBundle::default();
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path).with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default())
        .with_plugin(RenderImgui::<StringBindings>::default());
    let ui_bundle = UiBundle::<StringBindings>::new();

    // init base bundles / systems
    let game_data = GameDataBuilder::default()
        // transforms (must come before ui bundle)
        .with_bundle(TransformBundle::new())?
        // audio (must come before input bundle)
        .with_bundle(audio_bundle)?
        // input (must come before ui bundle)
        .with_bundle(input_bundle)?
        // ui
        .with_bundle(ui_bundle)?
        // rendering
        .with_bundle(rendering_bundle)?
        .with_barrier()
        .with(systems::DebugSystem, "debug_system", &[]);

    // start the game
    let assets_dir = app_root.join("assets");
    let mut game = Application::build(assets_dir, states::LoadingState)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            60,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
