//! The `Paddle`, `PaddleSide` and logic related to the paddles.

use bevy::input::ButtonInput;
use bevy::prelude::{Component, KeyCode, Query, Res, Time, Transform, Window, With};
use bevy::window::PrimaryWindow;
use crate::PADDLE_SIZE;

// what does the timestep get mutliplied by?
const PADDLE_SPEED: f32 = 200.0;

#[derive(Copy, Clone, PartialEq)]
pub enum PaddleSide {
    Left,
    Right,
}

#[derive(Component, Copy, Clone, PartialEq)]
pub struct Paddle {
    side: PaddleSide,
}

impl Paddle {
    #[inline(always)]
    pub const fn new(side: PaddleSide) -> Self {
        Self { side }
    }

    #[inline(always)]
    pub const fn side(&self) -> PaddleSide {
        self.side
    }
}

pub fn move_paddles(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut paddles: Query<(&mut Transform, &Paddle)>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let half_paddle_height = PADDLE_SIZE.y / 2.0;
    let min_y = -window.height() / 2.0 + half_paddle_height;
    let max_y = window.height() / 2.0 - half_paddle_height;

    for (mut transform, paddle) in &mut paddles {
        let (up_key, down_key) = match paddle.side() {
            PaddleSide::Left => (KeyCode::KeyW, KeyCode::KeyS),
            PaddleSide::Right => (KeyCode::ArrowUp, KeyCode::ArrowDown),
        };

        let direction =
            keyboard_input.pressed(up_key) as i8 - keyboard_input.pressed(down_key) as i8;

        transform.translation.y +=
            direction as f32 * PADDLE_SPEED * time.delta_secs();

        transform.translation.y = transform.translation.y.clamp(min_y, max_y);
    }
}