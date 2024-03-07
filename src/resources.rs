use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub score: usize,
    pub alive: bool,
    pub speed_y: f32,
    pub game_over: bool,
}

#[derive(Resource)]
pub struct SpawnPipeTimer {
    pub t: Timer,
}
