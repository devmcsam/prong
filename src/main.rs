//! Rong is pong built with modern rust and the bevy game engine.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const PADDLE_SIZE: Vec2 = Vec2::new(10.0, 100.0);
const CIRCLE_RADIUS: f32 = 10.0;
const PADDLE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
// what does the timestep get mutliplied by?
const PADDLE_SPEED: f32 = 250.0;
const BALL_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
// what does the timestep get mutliplied by?
const BALL_SPEED: f32 = 100.0;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, move_paddles)
        .run();
}

#[derive(Copy, Clone, PartialEq)]
enum PaddleSide {
    Left,
    Right,
}

#[derive(Component, Copy, Clone, PartialEq)]
struct Paddle {
    side: PaddleSide,
}

#[derive(Component, Copy, Clone, PartialEq)]
struct Ball {
    position: Vec2,
    direction: Vec2,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window, With<PrimaryWindow>>,
) {
    commands.spawn(Camera2d);
    let Ok(window) = window.single() else {
        return;
    };

    let width = window.width();

    let right_edge = width / 2.0;
    let left_edge = -width / 2.0;

    let right_paddle_mesh = meshes.add(Rectangle::new(PADDLE_SIZE.x, PADDLE_SIZE.y));
    let right_paddle_material = materials.add(PADDLE_COLOR);

    let left_paddle_mesh = meshes.add(Rectangle::new(PADDLE_SIZE.x, PADDLE_SIZE.y));
    let left_paddle_material = materials.add(PADDLE_COLOR);

    let ball_mesh = meshes.add(Circle::new(CIRCLE_RADIUS));
    let ball_material = materials.add(BALL_COLOR);

    commands.spawn((
        Mesh2d(right_paddle_mesh),
        MeshMaterial2d(right_paddle_material),
        Transform::from_xyz(right_edge - PADDLE_SIZE.x, 0.0, 0.0),
        Paddle {
            side: PaddleSide::Right,
        },
    ));
    commands.spawn((
        Mesh2d(left_paddle_mesh),
        MeshMaterial2d(left_paddle_material),
        Transform::from_xyz(left_edge + PADDLE_SIZE.x, 0.0, 0.0),
        Paddle {
            side: PaddleSide::Left,
        },
    ));
    commands.spawn((
        Mesh2d(ball_mesh),
        MeshMaterial2d(ball_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Ball { position: Vec2::ZERO, direction: Vec2::ZERO }
    ));
}

fn move_paddles(
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
        let (up_key, down_key) = match paddle.side {
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