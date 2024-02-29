use bevy::{input::keyboard::KeyboardInput, prelude::*};
use std::process;

const BG_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
const FONT_SIZE: f32 = 24.0;
const START_TEXT: &str = "Press Space to start and jump. Press Q to quit.";
const SCORE_TEXT: &str = "Score: ";
const MAX_SPEED: f32 = 5.0;
const MIN_SPEED: f32 = -5.0;
const JUMP_SPEED_INC: f32 = 2.5;
const SPEED_DEC: f32 = -0.3;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(GameState {
            score: 0,
            alive: false,
            velocity_x: 0.0,
            velocity_y: 0.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(mut commands: Commands) {
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

fn update(
    mut game_state: ResMut<GameState>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<InfoText>)>,
    mut text_query: Query<&mut Text, (With<InfoText>, Without<ScoreText>)>,
    mut kb_events: EventReader<KeyboardInput>,
) {
    match game_state.alive {
        true => {
            kb_events.read().for_each(|event| match event.key_code {
                KeyCode::Space => match game_state.velocity_y {
                    _ if game_state.velocity_y < MAX_SPEED => {
                        game_state.velocity_y += JUMP_SPEED_INC
                    }
                    _ => (),
                },
                KeyCode::KeyQ => process::exit(0x0),
                _ => (),
            });
            match game_state.velocity_y {
                _ if game_state.velocity_y > MIN_SPEED => game_state.velocity_y += SPEED_DEC,
                _ => (),
            }
        }
        false => {
            kb_events.read().for_each(|event| match event.key_code {
                KeyCode::Space => {
                    text_query.single_mut().sections[0].value = "".to_string();
                    score_query.single_mut().sections[0].value = SCORE_TEXT.to_owned();
                    game_state.alive = true;
                    game_state.velocity_x = 5.0;
                    score_query.single_mut().sections[1].value = game_state.score.to_string();
                }
                KeyCode::KeyQ => process::exit(0x0),
                _ => (),
            });
        }
    }
}

#[derive(Resource)]
struct GameState {
    score: usize,
    alive: bool,
    velocity_y: f32,
    velocity_x: f32,
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct InfoText;
