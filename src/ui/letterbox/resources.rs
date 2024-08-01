use bevy::prelude::{Component, Resource};

#[derive(Component)]
pub struct LetterboxTop;

#[derive(Component)]
pub struct LetterboxBottom;

#[derive(Component)]
pub struct LetterboxSide;

#[derive(Component)]
pub struct Letterbox;

#[derive(Resource, Default)]
pub struct LetterboxState {
    pub active: bool,
}
