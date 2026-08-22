//! Prong is a game similar to pong built with modern Rust and the Bevy game engine.

mod ball;
mod paddle;
mod ui;

use ball::{Ball, finish_serve_delay, move_ball};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};
use paddle::{Paddle, PaddleSide, move_paddles};
use ui::{
    Score, exit_button, hide_pause_menu, pause_with_escape, resume_button,
    setup_ui, show_pause_menu, toggle_fps, update_fps, update_scoreboard,
};

const PADDLE_SIZE: Vec2 = Vec2::new(10.0, 100.0);
const CIRCLE_RADIUS: f32 = 10.0;
const PADDLE_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);

const BALL_COLOR: Color = Color::srgb(1.0, 1.0, 1.0);
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(1.0, 0.35);

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Score::new(0, 0))

        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Prong".to_string(),
                    mode: WindowMode::BorderlessFullscreen(
                        MonitorSelection::Primary,
                    ),
                    ..default()
                }),
                ..default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .init_state::<GameState>()
        .add_systems(Startup, (setup, setup_ui))
        .add_systems(
            Update,
            (
                (
                    finish_serve_delay,
                    move_paddles,
                    move_ball,
                    update_scoreboard,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
                pause_with_escape,
                resume_button.run_if(in_state(GameState::Paused)),
                toggle_fps,
                update_fps,
                exit_button,
            ),
        )
        .add_systems(OnEnter(GameState::Paused), show_pause_menu)
        .add_systems(OnExit(GameState::Paused), hide_pause_menu)
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

    let right_paddle_mesh =
        meshes.add(Rectangle::new(PADDLE_SIZE.x, PADDLE_SIZE.y));
    let right_paddle_material = materials.add(PADDLE_COLOR);

    let left_paddle_mesh =
        meshes.add(Rectangle::new(PADDLE_SIZE.x, PADDLE_SIZE.y));
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
        Ball::new(
            Vec2::ZERO,
            INITIAL_BALL_DIRECTION.normalize(),
        ),
    ));
}