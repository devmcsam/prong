use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use crate::paddle::PaddleSide;

#[derive(Resource, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Score {
    left: u16,
    right: u16,
}

impl Score {
    #[inline(always)]
    pub const fn new(left: u16, right: u16) -> Self {
        Self { left, right }
    }

    #[inline(always)]
    pub const fn get_left_score(&self) -> u16 {
        self.left
    }

    #[inline(always)]
    pub const fn get_right_score(&self) -> u16 {
        self.right
    }

    #[inline(always)]
    pub const fn increment(&mut self, side: PaddleSide) {
        match side {
            PaddleSide::Left => self.left += 1,
            PaddleSide::Right => self.right += 1,
        }
    }
}

#[derive(Component)]
pub struct ScoreboardText;

#[derive(Component)]
pub struct ShowFpsButton;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct ExitButton;

pub fn setup_ui(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            top: px(24),
            width: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            row_gap: px(12),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("0   0"),
                TextFont {
                    font_size: FontSize::Px(48.0),
                    ..default()
                },
                TextColor(Color::WHITE),
                ScoreboardText,
            ));

            parent
                .spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(px(24), px(10)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ShowFpsButton,
                ))
                .with_child((
                    Text::new("Show FPS"),
                    TextFont {
                        font_size: FontSize::Px(24.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));

            parent
                .spawn((
                    Button,
                    Node {
                        padding: UiRect::axes(px(24), px(10)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                    ExitButton,
                ))
                .with_child((
                    Text::new("Exit"),
                    TextFont {
                        font_size: FontSize::Px(24.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
        });

    commands.spawn((
        Text::new("FPS: --"),
        TextFont {
            font_size: FontSize::Px(24.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: px(16),
            right: px(16),
            ..default()
        },
        Visibility::Hidden,
        FpsText,
    ));
}

pub fn update_scoreboard(
    score: Res<Score>,
    mut scoreboard: Query<&mut Text, With<ScoreboardText>>,
) {
    if !score.is_changed() {
        return;
    }

    let Ok(mut text) = scoreboard.single_mut() else {
        return;
    };

    text.0 = format!(
        "{}   {}",
        score.get_left_score(),
        score.get_right_score()
    );
}

pub fn toggle_fps(
    interactions: Query<
        &Interaction,
        (Changed<Interaction>, With<ShowFpsButton>),
    >,
    mut fps_text: Query<&mut Visibility, With<FpsText>>,
) {
    let Ok(mut visibility) = fps_text.single_mut() else {
        return;
    };

    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            *visibility = match *visibility {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden,
            };
        }
    }
}

pub fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text: Query<(&mut Text, &Visibility), With<FpsText>>,
) {
    let Ok((mut text, visibility)) = fps_text.single_mut() else {
        return;
    };

    if *visibility == Visibility::Hidden {
        return;
    }

    let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.smoothed())
    else {
        return;
    };

    text.0 = format!("FPS: {fps:.0}");
}

pub fn exit_button(
    interactions: Query<
        &Interaction,
        (Changed<Interaction>, With<ExitButton>),
    >,
    mut exit: MessageWriter<AppExit>,
) {
    for interaction in &interactions {
        if *interaction == Interaction::Pressed {
            exit.write(AppExit::Success);
        }
    }
}