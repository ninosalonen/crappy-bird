use std::{
    env,
    process::{self, Command},
};

use bevy::{
    input::keyboard::KeyboardInput,
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use rand::Rng;

use crate::{Bird, GameState, InfoText, Pipe, PipePassed, ScoreText, SpawnPipeTimer};

const SCORE_TEXT: &str = "Score: ";
const MAX_SPEED: f32 = 5.0;
const SPEED_DEC: f32 = -0.2;
const BIRD_SIZE: f32 = 25.0;
const PIPE_WIDTH: f32 = 50.0;
const GAP: f32 = 100.0;
const HALF_GAP: f32 = GAP / 2.0;
const DEATH_TEXT: &str = "You died (noob). Press Enter to restart or Esc to quit.";

pub fn update(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut timer: ResMut<SpawnPipeTimer>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<InfoText>)>,
    mut text_query: Query<&mut Text, (With<InfoText>, Without<ScoreText>)>,
    mut bird_query: Query<&mut Transform, (With<Bird>, Without<Pipe>)>,
    mut pipe_query: Query<
        (&Pipe, &mut Transform, Entity, &mut PipePassed),
        (With<Pipe>, Without<Bird>),
    >,
    mut kb_events: EventReader<KeyboardInput>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    window: Query<&Window>,
) {
    match game_state.alive {
        true => {
            let window_height = window.single().height();
            let window_width = window.single().width();
            let mut bird = bird_query.single_mut();

            // Update bird location
            let ground = -(window_height / 2.0);
            let roof = window_height / 2.0;
            let move_amount = game_state.speed_y * time.delta_seconds() * 100.0;

            match bird.translation.y {
                h if h + move_amount + BIRD_SIZE > roof => (),
                h if h + move_amount - BIRD_SIZE < ground => (),
                _ => bird.translation.y += move_amount,
            }

            // Iterate over the pipes
            for (pipe, mut transform, entity, mut pipe_passed) in pipe_query.iter_mut() {
                // Check collisions
                let aabb2d = Aabb2d::new(
                    transform.translation.truncate(),
                    Vec2::new(PIPE_WIDTH / 2.0, pipe.0 / 2.0),
                );
                if aabb2d.intersects(&BoundingCircle::new(bird.translation.truncate(), BIRD_SIZE)) {
                    game_state.alive = false;
                    game_state.game_over = true;
                }

                // Move pipes to the left
                transform.translation.x -= time.delta_seconds() * 200.0;

                // Remove pipes from left of screen
                let left_threshold = -(window_width / 2.0 + PIPE_WIDTH / 2.0);
                if transform.translation.x < left_threshold {
                    commands.entity(entity).remove::<Pipe>();
                }

                // Increment score after passing pipes
                if !pipe_passed.0 && transform.translation.x < -BIRD_SIZE {
                    pipe_passed.0 = true;
                    game_state.score += 1;
                    score_query.single_mut().sections[1].value = game_state.score.to_string();
                }
            }

            // Spawn pipes
            timer.t.tick(time.delta());
            if timer.t.finished() {
                let pipe_x_offset = window_width / 2.0 + PIPE_WIDTH / 2.0;

                let mut rng = rand::thread_rng();
                let max_offset = window_height / 4.0 - GAP;
                let gap_offset = rng.gen_range(-max_offset..max_offset) as f32;

                let top_y = window_height / 4.0 + HALF_GAP + gap_offset;
                let bottom_y = -(window_height / 4.0) - HALF_GAP + gap_offset;
                let top_pipe_height = ((window_height / 2.0) - top_y) * 2.0;
                let bottom_pipe_height = -((-(window_height / 2.0) - bottom_y) * 2.0);

                // Top pipe
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(PIPE_WIDTH, top_pipe_height))),
                        material: materials.add(Color::DARK_GREEN),
                        transform: Transform::from_xyz(pipe_x_offset, top_y, 0.0),
                        ..Default::default()
                    },
                    Pipe(top_pipe_height),
                    PipePassed(false),
                ));

                // Bottom pipe
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(
                            meshes.add(Rectangle::new(PIPE_WIDTH, bottom_pipe_height)),
                        ),
                        material: materials.add(Color::DARK_GREEN),
                        transform: Transform::from_xyz(pipe_x_offset, bottom_y, 0.0),
                        ..Default::default()
                    },
                    Pipe(bottom_pipe_height),
                    // Calculate the score from top pipes
                    PipePassed(true),
                ));
            }

            kb_events.read().for_each(|event| match event.key_code {
                KeyCode::Space => {
                    game_state.speed_y = MAX_SPEED;
                }
                KeyCode::Escape => process::exit(0x0),
                _ => (),
            });

            match game_state.speed_y {
                _ if game_state.speed_y > -MAX_SPEED => game_state.speed_y += SPEED_DEC,
                _ => (),
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
