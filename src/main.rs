mod bundles;
mod components;
mod gamedata;
mod input;
mod prefabs;
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
use amethyst::utils::fps_counter::FpsCounterBundle;
use amethyst_imgui::RenderImgui;

use bundles::{EngineBundle, GameBundle, MenuBundle};
use gamedata::CustomGameDataBuilder;
use input::InputBindingTypes;

// https://github.com/amethyst/amethyst/tree/v0.13.2/examples/custom_game_data
// has some examples of doing a loading UI

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

pub const PADDLE_SPEED: f32 = 50.0;
pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_HEIGHT: f32 = 16.0;

const AUDIO_BOUNCE: &str = "audio/sfx/bounce.ogg";
const AUDIO_SCORE: &str = "audio/sfx/score.ogg";

const AUDIO_MUSIC: &[&str] = &[
    "audio/music/Computer_Music_All-Stars_-_Wheres_My_Jetpack.ogg",
    "audio/music/Computer_Music_All-Stars_-_Albatross_v2.ogg",
];

#[derive(PartialEq)]
pub enum RunningState {
    Running,
    Paused,
}

impl Default for RunningState {
    fn default() -> Self {
        RunningState::Running
    }
}

pub struct SoundEffects {
    pub score: SourceHandle,
    pub bounce: SourceHandle,
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

    // TODO: remove old logs

    amethyst::start_logger(amethyst::LoggerConfig {
        level_filter: amethyst::LogLevelFilter::Info,
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

    let input_bindings_path = if cfg!(feature = "sdl_controller") {
        app_root.join("etc").join("controller_input.ron")
    } else {
        app_root.join("etc").join("keyboard_input.ron")
    };

    // create bundles
    let input_bundle =
        InputBundle::<InputBindingTypes>::new().with_bindings_from_file(input_bindings_path)?;
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)?.with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderUi::default())
        .with_plugin(RenderImgui::<StringBindings>::default());
    let ui_bundle = UiBundle::<StringBindings>::new();

    // init base bundles / systems
    let game_data = CustomGameDataBuilder::default()
        // transforms (must come before ui bundle)
        .with_engine_bundle(TransformBundle::new())
        // audio (must come before input bundle)
        .with_engine_bundle(AudioBundle::default())
        // input (must come before ui bundle)
        .with_engine_bundle(input_bundle)
        // ui
        .with_engine_bundle(ui_bundle)
        // rendering
        .with_engine_bundle(rendering_bundle)
        // fps counter
        .with_engine_bundle(FpsCounterBundle::default())
        // engine bundle
        .with_engine_bundle(EngineBundle::default())
        // menu bundles
        .with_menu_bundle(MenuBundle::default())
        // game bundles
        .with_game_bundle(GameBundle::default())
        .with_engine_barrier()
        .with_engine(systems::DebugSystem, "debug_system", &[]);

    // start the game
    let assets_dir = app_root.join("assets");
    let mut game = Application::build(assets_dir, states::MenuState::default())?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            60,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
