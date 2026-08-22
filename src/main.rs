//! Rong is pong built with modern rust and the bevy game engine.

mod paddle;
use paddle::{Paddle, PaddleSide, move_paddles};
mod ball;
use ball::{Ball, move_ball};

use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};

const PADDLE_SIZE: Vec2 = Vec2::new(10.0, 100.0);
const CIRCLE_RADIUS: f32 = 10.0;
const PADDLE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

const BALL_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(1.0, 0.35);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rong".to_string(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_paddles, move_ball).chain())
        .run();
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
        Transform::from_xyz(
            right_edge - PADDLE_SIZE.x / 2.0,
            0.0,
            0.0,
        ),
        Paddle::new(PaddleSide::Right),
    ));
    commands.spawn((
        Mesh2d(left_paddle_mesh),
        MeshMaterial2d(left_paddle_material),
        Transform::from_xyz(
            left_edge + PADDLE_SIZE.x / 2.0,
            0.0,
            0.0,
        ),
        Paddle::new(PaddleSide::Left),
    ));
    commands.spawn((
        Mesh2d(ball_mesh),
        MeshMaterial2d(ball_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Ball::new(Vec2::ZERO, INITIAL_BALL_DIRECTION.normalize()),
    ));
}