use bevy::prelude::*;

use crate::{InfoText, ScoreText};

const FONT_SIZE: f32 = 24.0;
const START_TEXT: &str = "Press Space to start and jump. Press Esc to quit.";

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        ScoreText,
        TextBundle::from_sections([
            TextSection::from_style(TextStyle {
                font_size: FONT_SIZE,
                color: Color::WHITE,
                ..default()
            }),
            TextSection::from_style(TextStyle {
                font_size: FONT_SIZE,
                color: Color::WHITE,
                ..default()
            }),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
    ));

    commands.spawn((
        InfoText,
        TextBundle::from_sections([TextSection::new(
            START_TEXT,
            TextStyle {
                font_size: FONT_SIZE,
                color: Color::WHITE,
                ..default()
            },
        )])
        .with_style(Style {
            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,
            ..default()
        }),
    ));
}
