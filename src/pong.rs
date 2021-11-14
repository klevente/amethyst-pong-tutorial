use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_HEIGHT: f32 = 16.0;

pub struct Pong;

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let sprite_sheet_handle = load_sprite_sheet(world);

        // components not used in any `System`s need to be manually registered in the `world`
        world.register::<Paddle>();

        initialise_paddles(world, sprite_sheet_handle);
        initialise_camera(world);
    }
}

#[derive(Eq, PartialEq)]
pub enum Side {
    Left,
    Right,
}

pub struct Paddle {
    pub side: Side,
    pub width: f32,
    pub height: f32,
}

impl Paddle {
    fn new(side: Side) -> Self {
        Self {
            side,
            width: PADDLE_WIDTH,
            height: PADDLE_HEIGHT,
        }
    }
}

impl Component for Paddle {
    // specify the storage type of `Paddle`s, choosing the best requires benchmarking
    type Storage = DenseVecStorage<Self>;
}

// the sprites inside the sheet are ordered based on their definitions inside the spritesheet file
fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    // asset loader `Resource`, responsible for loading in various types of assets
    let loader = world.read_resource::<Loader>();

    // `Clone`able reference to the texture
    let texture_handle = {
        // Amethyst manages `Texture`s internally, so it must be loaded into an `AssetStorage`
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Amethyst manages `SpriteSheet`s internally, so it must be loaded into an `AssetStorage`
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        // load the `.ron` file which specifies the sprites
        "texture/pong_spritesheet.ron",
        // extract sprites from the loaded texture
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialise_camera(world: &mut World) {
    // setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        // create orthographic camera useful for 2D rendering, `z` is 1.0 as sprites are at `0.0`
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

// initialises one paddle on the left and one paddle on the right
fn initialise_paddles(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut left_transform = Transform::default();
    let mut right_transform = Transform::default();

    // paddles begin in the middle vertically
    let y = ARENA_HEIGHT * 0.5;
    // the anchor point of entities are their midpoints, this is why they have to be
    // translated by half of their width in both cases
    left_transform.set_translation_xyz(PADDLE_WIDTH * 0.5, y, 0.0);
    right_transform.set_translation_xyz(ARENA_WIDTH - PADDLE_WIDTH * 0.5, y, 0.0);

    // the paddle sprite is the first one inside the sheet
    // one is enough as both paddles look exactly the same
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 0);

    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Paddle::new(Side::Left))
        .with(left_transform)
        .build();

    world
        .create_entity()
        .with(sprite_render)
        .with(Paddle::new(Side::Right))
        .with(right_transform)
        .build();
}
