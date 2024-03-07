use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub score: usize,
    pub alive: bool,
    pub speed_y: f32,
    pub game_over: bool,
    pub pipe_spawn_timer: Timer,
}
