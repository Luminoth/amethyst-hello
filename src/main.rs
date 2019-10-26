mod components;
mod states;
mod systems;
mod utils;

use std::path::PathBuf;

use amethyst::core::transform::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::prelude::*;
use amethyst::renderer::plugins::{RenderFlat2D, RenderToWindow};
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::RenderingBundle;
use amethyst::utils::application_root_dir;
use amethyst_imgui::RenderImgui;

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

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
    let rendering_bundle = RenderingBundle::<DefaultBackend>::new()
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path).with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderFlat2D::default())
        .with_plugin(RenderImgui::<StringBindings>::default());

    let game_data = GameDataBuilder::default()
        // input
        .with_bundle(input_bundle)?
        // rendering
        .with_bundle(rendering_bundle)?
        // transforms
        .with_bundle(TransformBundle::new())?
        // systems
        .with(
            systems::PaddleInputSystem,
            "paddle_input_system",
            &["input_system"],
        )
        .with(systems::BallMovementSystem, "ball_movement_system", &[])
        .with(
            systems::BallCollisionSystem,
            "ball_collision_system",
            &["paddle_input_system", "ball_movement_system"],
        )
        .with_barrier()
        .with(systems::DebugSystem, "debug_system", &[]);

    // load assets
    let assets_dir = app_root.join("assets");

    // start the game
    Application::new(assets_dir, states::LoadingState, game_data)?.run();

    Ok(())
}
