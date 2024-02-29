use bevy::prelude::Resource;

#[derive(Resource)]
pub struct GameState {
    pub score: usize,
    pub alive: bool,
    pub speed_y: f32,
}
