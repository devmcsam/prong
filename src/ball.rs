//! The `Ball` and logic related to the ball.

use crate::paddle::{Paddle, PaddleSide};
use crate::ui::Score;
use crate::{CIRCLE_RADIUS, INITIAL_BALL_DIRECTION, PADDLE_SIZE};
use bevy::prelude::{
    Commands, Component, Entity, Query, Res, ResMut, Time, Timer, TimerMode,
    Transform, Vec2, Visibility, Window, With, Without,
};
use bevy::window::PrimaryWindow;

const BALL_SPEED: f32 = 250.0;
const SERVE_DELAY_SECONDS: f32 = 1.0;

#[derive(Component)]
pub struct Serving {
    timer: Timer,
}

impl Serving {
    fn new() -> Self {
        Self {
            timer: Timer::from_seconds(
                SERVE_DELAY_SECONDS,
                TimerMode::Once,
            ),
        }
    }
}

#[derive(Component, Copy, Clone, PartialEq)]
pub struct Ball {
    position: Vec2,
    direction: Vec2,
}

impl Ball {
    #[inline(always)]
    pub const fn new(position: Vec2, direction: Vec2) -> Self {
        Self {
            position,
            direction,
        }
    }
}

/// Advances the countdown for balls waiting to be served.
///
/// Once the countdown finishes, the ball is shown and its `Serving` component
/// is removed. Without that component, `move_ball` can process it again.
pub fn finish_serve_delay(
    mut commands: Commands,
    time: Res<Time>,
    mut serving_balls: Query<(
        Entity,
        &mut Serving,
        &mut Visibility,
    )>,
) {
    for (entity, mut serving, mut visibility) in &mut serving_balls {
        serving.timer.tick(time.delta());

        if serving.timer.is_finished() {
            *visibility = Visibility::Visible;
            commands.entity(entity).remove::<Serving>();
        }
    }
}

pub fn move_ball(
    mut commands: Commands,
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
    paddles: Query<(&Transform, &Paddle), Without<Ball>>,
    mut balls: Query<
        (Entity, &mut Transform, &mut Ball, &mut Visibility),
        (Without<Paddle>, Without<Serving>),
    >,
    mut score: ResMut<Score>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;
    let half_paddle_width = PADDLE_SIZE.x / 2.0;
    let half_paddle_height = PADDLE_SIZE.y / 2.0;

    for (entity, mut ball_transform, mut ball, mut visibility) in &mut balls {
        let movement =
            ball.direction * BALL_SPEED * time.delta_secs();

        ball.position += movement;

        if ball.position.y + CIRCLE_RADIUS >= half_height {
            ball.position.y = half_height - CIRCLE_RADIUS;
            ball.direction.y = -ball.direction.y.abs();
        } else if ball.position.y - CIRCLE_RADIUS <= -half_height {
            ball.position.y = -half_height + CIRCLE_RADIUS;
            ball.direction.y = ball.direction.y.abs();
        }

        for (paddle_transform, paddle) in &paddles {
            let paddle_position =
                paddle_transform.translation.truncate();

            let overlaps_horizontally =
                ball.position.x + CIRCLE_RADIUS
                    >= paddle_position.x - half_paddle_width
                    && ball.position.x - CIRCLE_RADIUS
                    <= paddle_position.x + half_paddle_width;

            let overlaps_vertically =
                ball.position.y + CIRCLE_RADIUS
                    >= paddle_position.y - half_paddle_height
                    && ball.position.y - CIRCLE_RADIUS
                    <= paddle_position.y + half_paddle_height;

            let moving_toward_paddle = match paddle.side() {
                PaddleSide::Left => ball.direction.x < 0.0,
                PaddleSide::Right => ball.direction.x > 0.0,
            };

            if overlaps_horizontally
                && overlaps_vertically
                && moving_toward_paddle
            {
                let hit_offset = (
                    (ball.position.y - paddle_position.y)
                        / half_paddle_height
                )
                    .clamp(-1.0, 1.0);

                let horizontal_direction = match paddle.side() {
                    PaddleSide::Left => 1.0,
                    PaddleSide::Right => -1.0,
                };

                ball.direction = Vec2::new(
                    horizontal_direction,
                    hit_offset * 0.75,
                )
                    .normalize();

                // Push the ball outside the paddle so that it cannot collide
                // with the same paddle repeatedly.
                ball.position.x = match paddle.side() {
                    PaddleSide::Left => {
                        paddle_position.x
                            + half_paddle_width
                            + CIRCLE_RADIUS
                    }
                    PaddleSide::Right => {
                        paddle_position.x
                            - half_paddle_width
                            - CIRCLE_RADIUS
                    }
                };

                break;
            }
        }

        if ball.position.x - CIRCLE_RADIUS > half_width {
            // The left player scored. Serve toward the right player.
            score.increment(PaddleSide::Left);

            ball.position = Vec2::ZERO;
            ball.direction = INITIAL_BALL_DIRECTION.normalize();

            *visibility = Visibility::Hidden;
            commands.entity(entity).insert(Serving::new());
        } else if ball.position.x + CIRCLE_RADIUS < -half_width {
            // The right player scored. Serve toward the left player.
            score.increment(PaddleSide::Right);

            ball.position = Vec2::ZERO;
            ball.direction = Vec2::new(
                -INITIAL_BALL_DIRECTION.x,
                INITIAL_BALL_DIRECTION.y,
            )
                .normalize();

            *visibility = Visibility::Hidden;
            commands.entity(entity).insert(Serving::new());
        }

        ball_transform.translation.x = ball.position.x;
        ball_transform.translation.y = ball.position.y;
    }
}