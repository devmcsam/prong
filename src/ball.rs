//! The `Ball` and logic related to the ball.

use bevy::math::Vec2;
use bevy::prelude::{Component, Query, Res, ResMut, Time, Transform, Window, With, Without};
use bevy::window::PrimaryWindow;
use crate::paddle::{Paddle, PaddleSide};
use crate::{CIRCLE_RADIUS, INITIAL_BALL_DIRECTION, PADDLE_SIZE};
use crate::ui::Score;


// what does the timestep get mutliplied by?
const BALL_SPEED: f32 = 250.0;

#[derive(Component, Copy, Clone, PartialEq)]
pub struct Ball {
    position: Vec2,
    direction: Vec2,
}

impl Ball {
    #[inline(always)]
    pub const fn new(position: Vec2, direction: Vec2) -> Self {
        Self { position, direction }
    }

    #[inline(always)]
    pub fn position(&self) -> Vec2 {
        self.position
    }

    #[inline(always)]
    pub fn direction(&self) -> Vec2 {
        self.direction
    }
}

pub fn move_ball(
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
    paddles: Query<(&Transform, &Paddle), Without<Ball>>,
    mut balls: Query<(&mut Transform, &mut Ball), Without<Paddle>>,
    mut score: ResMut<Score>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;
    let half_paddle_width = PADDLE_SIZE.x / 2.0;
    let half_paddle_height = PADDLE_SIZE.y / 2.0;

    for (mut ball_transform, mut ball) in &mut balls {
        let movement = ball.direction * BALL_SPEED * time.delta_secs();
        ball.position += movement;

        if ball.position.y + CIRCLE_RADIUS >= half_height {
            ball.position.y = half_height - CIRCLE_RADIUS;
            ball.direction.y = -ball.direction.y.abs();
        } else if ball.position.y - CIRCLE_RADIUS <= -half_height {
            ball.position.y = -half_height + CIRCLE_RADIUS;
            ball.direction.y = ball.direction.y.abs();
        }

        for (paddle_transform, paddle) in &paddles {
            let paddle_position = paddle_transform.translation.truncate();

            let overlaps_horizontally =
                ball.position().x + CIRCLE_RADIUS >= paddle_position.x - half_paddle_width
                    && ball.position().x - CIRCLE_RADIUS
                    <= paddle_position.x + half_paddle_width;

            let overlaps_vertically =
                ball.position().y + CIRCLE_RADIUS >= paddle_position.y - half_paddle_height
                    && ball.position().y - CIRCLE_RADIUS
                    <= paddle_position.y + half_paddle_height;

            let moving_toward_paddle = match paddle.side() {
                PaddleSide::Left => ball.direction().x < 0.0,
                PaddleSide::Right => ball.direction().x > 0.0,
            };

            if overlaps_horizontally && overlaps_vertically && moving_toward_paddle {
                // A hit near a paddle's edge creates a steeper bounce.
                let hit_offset =
                    ((ball.position().y - paddle_position.y) / half_paddle_height)
                        .clamp(-1.0, 1.0);

                let horizontal_direction = match paddle.side() {
                    PaddleSide::Left => 1.0,
                    PaddleSide::Right => -1.0,
                };

                ball.direction =
                    Vec2::new(horizontal_direction, hit_offset * 0.75).normalize();

                // Move the ball outside the paddle so it can't collide repeatedly
                ball.position.x = match paddle.side() {
                    PaddleSide::Left => {
                        paddle_position.x + half_paddle_width + CIRCLE_RADIUS
                    }
                    PaddleSide::Right => {
                        paddle_position.x - half_paddle_width - CIRCLE_RADIUS
                    }
                };

                break;
            }
        }

        if ball.position().x - CIRCLE_RADIUS > half_width {
            // The left player scored. Serve toward the right player.
            score.increment(PaddleSide::Left);
            ball.position = Vec2::ZERO;
            ball.direction = INITIAL_BALL_DIRECTION.normalize();
        } else if ball.position().x + CIRCLE_RADIUS < -half_width {
            // The right player scored. Serve toward the left player.
            score.increment(PaddleSide::Right);
            ball.position = Vec2::ZERO;
            ball.direction =
                Vec2::new(-INITIAL_BALL_DIRECTION.x, INITIAL_BALL_DIRECTION.y)
                    .normalize();
        }

        ball_transform.translation.x = ball.position().x;
        ball_transform.translation.y = ball.position().y;
    }
}