use amethyst::{
    core::{SystemDesc, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

// `SystemDesc` provides a recipe for how to instantiate this `System`
#[derive(SystemDesc)]
pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
    // data that the system operates on
    type SystemData = (
        // it mutates `Transform` components
        WriteStorage<'s, Transform>,
        // it reads `Paddle` components
        ReadStorage<'s, Paddle>,
        // accesses current inputs
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        // iterate over entities that have *both* a `Paddle` and `Transform` component
        // `par_join` can be used to join in parallel, but it is not worth doing here
        for (paddle, transform) in (&paddles, &mut transforms).join() {
            let movement = match paddle.side {
                Side::Left => input.axis_value("left_paddle"),
                Side::Right => input.axis_value("right_paddle"),
            };
            if let Some(mv_amount) = movement {
                let scaled_amount = 1.2 * mv_amount as f32;
                let paddle_y = transform.translation().y;
                transform.set_translation_y(
                    (paddle_y + scaled_amount)
                        // clamp to top
                        .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
                        // clamp to bottom
                        .max(PADDLE_HEIGHT * 0.5),
                );
            }
        }
    }
}
