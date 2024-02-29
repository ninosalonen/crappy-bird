mod components;
mod gameloop;
mod resources;
mod setup;

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
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}
