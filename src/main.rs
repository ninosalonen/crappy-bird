mod components;
mod gameloop;
mod resources;
mod setup;

use std::time::Duration;

use bevy::prelude::*;

pub use components::*;
pub use gameloop::*;
pub use resources::*;
pub use setup::*;

const BG_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Crappy bird".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(GameState {
            score: 0,
            alive: false,
            speed_y: 0.0,
            game_over: false,
            pipe_spawn_timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}
