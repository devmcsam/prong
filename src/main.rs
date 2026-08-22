//! Rong is pong built with modern rust and the bevy game engine.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const PADDLE_SIZE: Vec2 = Vec2::new(10.0, 100.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component, Copy, Clone, PartialEq)]
struct Paddle {
    // only need to store y pos as they are on the edge of the window.
    y_pos: f32,
    speed: f32,
}

struct Ball {
    position: Vec2,
    direction: Vec2,
    radius: f32,
    speed: f32,
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

    let mesh = meshes.add(Rectangle::new(PADDLE_SIZE.x, PADDLE_SIZE.y));
    let material = materials.add(Color::srgb(0.5, 0.2, 0.8));

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(right_edge - PADDLE_SIZE.x, 0.0, 0.0),
        Paddle { y_pos: 0.0, speed: 50.0 }
    ));

    let mesh = meshes.add(Rectangle::new(PADDLE_SIZE.x, PADDLE_SIZE.y));
    let material = materials.add(Color::srgb(0.5, 0.2, 0.8));

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(left_edge + PADDLE_SIZE.x, 0.0, 0.0),
        Paddle { y_pos: 0.0, speed: 50.0 }
    ));
}