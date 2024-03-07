use std::{
    env,
    process::{self, Command},
};

use bevy::{
    input::keyboard::KeyboardInput,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{Bird, GameState, InfoText, Pipe, ScoreText, SpawnPipeTimer};

const SCORE_TEXT: &str = "Score: ";
const MAX_SPEED: f32 = 5.0;
const MIN_SPEED: f32 = -5.0;
const SPEED_DEC: f32 = -0.2;
const BIRD_SIZE: f32 = 25.0;
const PIPE_WIDTH: f32 = 50.0;
const DEATH_TEXT: &str = "You died (noob). Press Enter to restart or Esc to quit.";

pub fn update(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut timer: ResMut<SpawnPipeTimer>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<InfoText>)>,
    mut text_query: Query<&mut Text, (With<InfoText>, Without<ScoreText>)>,
    mut bird_query: Query<(&Bird, &mut Transform), (With<Bird>, Without<Pipe>)>,
    mut pipe_query: Query<(&Pipe, &mut Transform, Entity), (With<Pipe>, Without<Bird>)>,
    mut kb_events: EventReader<KeyboardInput>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    window: Query<&Window>,
) {
    match game_state.alive {
        true => {
            // Check for collision
            for (_, bird_transform) in bird_query.iter_mut() {
                for (_, pipe_transform, _) in pipe_query.iter_mut() {
                    if check_collision(bird_transform.translation, pipe_transform.translation) {
                        game_state.alive = false;
                        game_state.game_over = true;
                    }
                }
            }

            // Remove pipes
            for (_, transform, entity) in pipe_query.iter_mut() {
                let window_width = window.single().width();

                let left_threshold = -(window_width / 2.0 + PIPE_WIDTH / 2.0);

                if transform.translation.x < left_threshold {
                    commands.entity(entity).remove::<Pipe>();
                }
            }

            // Spawn pipes
            timer.t.tick(time.delta());
            if timer.t.finished() {
                let gap = 100.0;
                let window_height = window.single().height();
                let window_width = window.single().width();

                let pipe_x_offset = window_width / 2.0 + PIPE_WIDTH / 2.0;

                let available_height = window_height - gap;

                let top_pipe_height = available_height / 2.0;
                let bottom_pipe_height = available_height / 2.0;

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(PIPE_WIDTH, top_pipe_height))),
                        material: materials.add(Color::DARK_GREEN),
                        transform: Transform::from_xyz(pipe_x_offset, window_height / 2.0, 0.0),
                        ..Default::default()
                    },
                    Pipe,
                ));

                // Spawn bottom pipe
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(
                            meshes.add(Rectangle::new(PIPE_WIDTH, bottom_pipe_height)),
                        ),
                        material: materials.add(Color::DARK_GREEN),
                        transform: Transform::from_xyz(pipe_x_offset, -(window_height / 2.0), 0.0),
                        ..Default::default()
                    },
                    Pipe,
                ));
            }

            kb_events.read().for_each(|event| match event.key_code {
                KeyCode::Space => game_state.speed_y = MAX_SPEED,
                KeyCode::Escape => process::exit(0x0),
                _ => (),
            });

            match game_state.speed_y {
                _ if game_state.speed_y > MIN_SPEED => game_state.speed_y += SPEED_DEC,
                _ => (),
            }

            // Update bird location
            for (_, mut transform) in bird_query.iter_mut() {
                let height = window.single().height();
                let ground = -(height / 2.0);
                let roof = height / 2.0;
                let move_amount = game_state.speed_y * time.delta_seconds() * 100.0;

                match transform.translation.y {
                    h if h + move_amount + BIRD_SIZE > roof => (),
                    h if h + move_amount - BIRD_SIZE < ground => (),
                    _ => transform.translation.y += move_amount,
                }
            }

            // Move pipes
            for (_, mut transform, _) in pipe_query.iter_mut() {
                let width = window.single().resolution.width();

                transform.translation.x -= time.delta_seconds() * 200.0;

                if transform.translation.x + PIPE_WIDTH < -(width / 2.0) {
                    transform.translation.x = width / 2.0;
                }
            }
        }
        false => match game_state.game_over {
            false => {
                kb_events.read().for_each(|event| match event.key_code {
                    KeyCode::Space => {
                        text_query.single_mut().sections[0].value = "".to_string();
                        score_query.single_mut().sections[0].value = SCORE_TEXT.to_owned();
                        score_query.single_mut().sections[1].value = game_state.score.to_string();
                        game_state.alive = true;

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
                    KeyCode::Escape => process::exit(0x0),
                    _ => (),
                });
            }
            true => {
                text_query.single_mut().sections[0].value = DEATH_TEXT.to_string();
                kb_events.read().for_each(|event| match event.key_code {
                    KeyCode::Enter => {
                        let _ = Command::new(env::current_exe().unwrap()).spawn();
                        process::exit(0x0);
                    }
                    KeyCode::Escape => process::exit(0x0),
                    _ => (),
                });
            }
        },
    }
}

fn check_collision(bird_pos: Vec3, pipe_pos: Vec3) -> bool {
    let bird_size = Vec3::splat(BIRD_SIZE);
    let pipe_size = Vec3::new(PIPE_WIDTH, 300.0, 0.0);
    let min_bird = bird_pos - bird_size / 2.0;
    let max_bird = bird_pos + bird_size / 2.0;
    let min_pipe = pipe_pos - pipe_size / 2.0;
    let max_pipe = pipe_pos + pipe_size / 2.0;

    let x_overlap = min_bird.x < max_pipe.x && max_bird.x > min_pipe.x;
    let y_overlap = min_bird.y < max_pipe.y && max_bird.y > min_pipe.y;

    x_overlap && y_overlap
}
