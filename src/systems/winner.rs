use amethyst::{
    core::transform::Transform,
    derive::SystemDesc,
    ecs::{Join, System, SystemData, WriteStorage},
};

use crate::pong::{Ball, ARENA_HEIGHT, ARENA_WIDTH};

#[derive(SystemDesc)]
pub struct WinnerSystem;

impl<'s> System<'s> for WinnerSystem {
    type SystemData = (WriteStorage<'s, Ball>, WriteStorage<'s, Transform>);

    fn run(&mut self, (mut balls, mut transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            let ball_x = transform.translation().x;

            let did_hit = if ball_x <= ball.radius {
                // right player scored on the left side
                println!("Player 2 scores!");
                true
            } else if ball_x >= ARENA_WIDTH - ball.radius {
                // left player scored on the right side
                println!("Player 1 scores!");
                true
            } else {
                false
            };

            if did_hit {
                // reverse direction
                ball.velocity[0] = -ball.velocity[0];
                // reset position
                transform.set_translation_x(ARENA_WIDTH * 0.5);
                transform.set_translation_y(ARENA_HEIGHT * 0.5);
            }
        }
    }
}
