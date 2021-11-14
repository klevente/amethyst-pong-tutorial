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
                if mv_amount != 0.0 {
                    let side_name = match paddle.side {
                        Side::Left => "left",
                        Side::Right => "right",
                    };
                    println!("Side {:?} moving {}", side_name, mv_amount);
                }
            }
        }
    }
}
