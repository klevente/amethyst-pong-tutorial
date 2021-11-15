use amethyst::{
    audio::AudioBundle,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod audio;
mod pong;
mod systems;

use crate::pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    // stores the game setup by the way of `System`s and bundles,
    // which are essentially collection of `System`s providing certain features to the engine
    let game_data = GameDataBuilder::default()
        // `TransformBundle` handles the tracking of entity positions
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // provides scaffolding for creating a window and drawing to it
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                // plugin used to render entities with a `SpriteRender` component
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default()),
        )?
        // needs to come after `InputBundle`
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        // provide an instance of the `System`, a string name and a list of dependencies
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system", &[])
        .with(
            systems::BounceSystem,
            "collision_system",
            &["paddle_system", "ball_system"],
        )
        .with(systems::WinnerSystem, "winner_system", &["ball_system"]);

    let assets_dir = app_root.join("assets");

    let mut game = Application::new(assets_dir, Pong::default(), game_data)?;
    game.run();
    Ok(())
}
