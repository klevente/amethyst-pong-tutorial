use amethyst::core::Time;
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, LineMode, TtfFormat, UiText, UiTransform},
};

pub const ARENA_WIDTH: f32 = 100.0;
pub const ARENA_HEIGHT: f32 = 100.0;

pub const PADDLE_WIDTH: f32 = 4.0;
pub const PADDLE_HEIGHT: f32 = 16.0;

pub const BALL_VELOCITY_X: f32 = 75.0;
pub const BALL_VELOCITY_Y: f32 = 50.0;
pub const BALL_RADIUS: f32 = 2.0;

#[derive(Default)]
pub struct Pong {
    // counts down to `Some(0)` then gets replaced with `None`
    ball_spawn_timer: Option<f32>,
    // only populated in `on_start`
    sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

impl SimpleState for Pong {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // wait one second before spawning the ball
        self.ball_spawn_timer.replace(1.0);

        // `Clone`able reference to the `SpriteSheet`
        self.sprite_sheet_handle.replace(load_sprite_sheet(world));

        // components not used in any `System`s need to be manually registered in the `world`
        // as `Paddle` is used in `PaddleSystem`, this is no longer necessary
        // world.register::<Paddle>();

        // `clone` is required as the function consumes this handle
        initialise_paddles(world, self.sprite_sheet_handle.clone().unwrap());
        initialise_camera(world);

        initialise_scoreboard(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if let Some(mut timer) = self.ball_spawn_timer.take() {
            // if timer has not expired, subtract elapsed time (`take` consumes the `Option`, leaving `None`)
            {
                let time = data.world.fetch::<Time>();
                timer -= time.delta_seconds();
            }

            if timer <= 0.0 {
                // if the timer expired, spawn the ball in
                initialise_ball(data.world, self.sprite_sheet_handle.clone().unwrap());
            } else {
                // if the timer has not expired, put its current value back in the state
                self.ball_spawn_timer.replace(timer);
            }
        }
        Trans::None
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

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

// initialises one ball in the middle of the screen
fn initialise_ball(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 0.0);

    // the ball is the second sprite in the sheet
    let sprite_render = SpriteRender::new(sprite_sheet_handle, 1);

    world
        .create_entity()
        .with(sprite_render)
        .with(Ball {
            radius: BALL_RADIUS,
            velocity: [BALL_VELOCITY_X, BALL_VELOCITY_Y],
        })
        .with(transform)
        .build();
}

// contains score data
#[derive(Default)] // important!
pub struct ScoreBoard {
    pub score_left: i32,
    pub score_right: i32,
}

// contains UI components that display the score
pub struct ScoreText {
    pub p1_score: Entity,
    pub p2_score: Entity,
}

fn initialise_scoreboard(world: &mut World) {
    let font = world.read_resource::<Loader>().load(
        "font/square.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );
    let p1_transform = UiTransform::new(
        "P1".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        -50.,
        -50.,
        1.,
        200.,
        50.,
    );
    let p2_transform = UiTransform::new(
        "P2".to_string(),
        Anchor::TopMiddle,
        Anchor::TopMiddle,
        50.,
        -50.,
        1.,
        200.,
        50.,
    );

    let p1_score = world
        .create_entity()
        .with(p1_transform)
        .with(UiText::new(
            font.clone(),
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    let p2_score = world
        .create_entity()
        .with(p2_transform)
        .with(UiText::new(
            font,
            "0".to_string(),
            [1., 1., 1., 1.],
            50.,
            LineMode::Single,
            Anchor::Middle,
        ))
        .build();

    // add this as a `Resource` to the `world`, so it can be accessed by other functions
    world.insert(ScoreText { p1_score, p2_score });
}
