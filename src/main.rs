use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod pong;

use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    // stores the game setup by the way of `System`s and bundles,
    // which are essentially collection of `System`s providing certain features to the engine
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // provides scaffolding for creating a window and drawing to it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
                )
                // plugin used to render entities with a `SpriteRender` component
                .with_plugin(RenderFlat2D::default()),
        )?
        // `TransformBundle` handles the tracking of entity positions
        .with_bundle(TransformBundle::new())?;

    let assets_dir = app_root.join("assets");

    let mut game = Application::new(assets_dir, Pong, game_data)?;
    game.run();
    Ok(())
}
