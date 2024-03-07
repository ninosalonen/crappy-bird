use bevy::prelude::Component;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct InfoText;

#[derive(Component)]
pub struct Bird;

#[derive(Component)]
pub struct Pipe(pub f32);

#[derive(Component)]
pub struct PipePassed(pub bool);
