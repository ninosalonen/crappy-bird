use std::process;

use bevy::{
    input::keyboard::KeyboardInput,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{Bird, GameState, InfoText, ScoreText};

const SCORE_TEXT: &str = "Score: ";
const MAX_SPEED: f32 = 5.0;
const MIN_SPEED: f32 = -5.0;
const SPEED_DEC: f32 = -0.2;
const BIRD_SIZE: f32 = 25.0;

pub fn update(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<InfoText>)>,
    mut text_query: Query<&mut Text, (With<InfoText>, Without<ScoreText>)>,
    mut kb_events: EventReader<KeyboardInput>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut bird_query: Query<(&mut Transform, &Bird)>,
    timer: Res<Time>,
    window: Query<&Window>,
) {
    match game_state.alive {
        true => {
            kb_events.read().for_each(|event| match event.key_code {
                KeyCode::Space => game_state.speed_y = MAX_SPEED,
                KeyCode::KeyQ => process::exit(0x0),
                _ => (),
            });

            match game_state.speed_y {
                _ if game_state.speed_y > MIN_SPEED => game_state.speed_y += SPEED_DEC,
                _ => (),
            }

            // Update bird location
            for (mut transform, _) in bird_query.iter_mut() {
                let height = window.single().resolution.height();
                let ground = -(height / 2.0);
                let roof = height / 2.0;
                let move_amount = game_state.speed_y * timer.delta_seconds() * 100.0;

                match transform.translation.y {
                    h if h + move_amount + BIRD_SIZE - 6.0 > roof => (),
                    h if h + move_amount - BIRD_SIZE + 6.0 < ground => (),
                    _ => transform.translation.y += move_amount,
                }
            }
        }
        false => {
            kb_events.read().for_each(|event| match event.key_code {
                KeyCode::Space => {
                    text_query.single_mut().sections[0].value = "".to_string();
                    score_query.single_mut().sections[0].value = SCORE_TEXT.to_owned();
                    game_state.alive = true;
                    score_query.single_mut().sections[1].value = game_state.score.to_string();

                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: Mesh2dHandle(meshes.add(Circle { radius: BIRD_SIZE })),
                            material: materials.add(Color::ORANGE),
                            transform: Transform::from_xyz(0.0, 0.0, 0.0),
                            ..default()
                        },
                        Bird,
                    ));
                }
                KeyCode::KeyQ => process::exit(0x0),
                _ => (),
            });
        }
    }
}
