mod components;
mod states;

use std::path::PathBuf;

use amethyst::core::transform::TransformBundle;
use amethyst::input::{InputBundle, StringBindings};
use amethyst::prelude::*;
use amethyst::renderer::plugins::{RenderFlat2D, RenderToWindow};
use amethyst::renderer::types::DefaultBackend;
use amethyst::renderer::RenderingBundle;
use amethyst::utils::application_root_dir;
use amethyst_imgui::RenderImgui;

use states::*;

fn main() -> amethyst::Result<()> {
    let logdir = PathBuf::new().join("var").join("logs");
    std::fs::create_dir_all(&logdir)?;

    amethyst::start_logger(amethyst::LoggerConfig {
        level_filter: amethyst::LogLevelFilter::Debug,
        log_file: Some(logdir.join("amethyst-hello.log")),
        ..Default::default()
    });

    let app_root = application_root_dir()?;

    // load configs
    let display_config_path = app_root.join("etc").join("display.ron");

    let game_data = GameDataBuilder::default()
        // input
        .with_bundle(InputBundle::<StringBindings>::default())?
        // rendering
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderImgui::<StringBindings>::default()),
        )?
        // transforms
        .with_bundle(TransformBundle::new())?;

    // load assets
    let assets_dir = app_root.join("assets");

    // start the game
    Application::new(assets_dir, LoadingState, game_data)?.run();

    Ok(())
}
