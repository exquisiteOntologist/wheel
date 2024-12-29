use bevy::prelude::{Component, Resource};

use super::constants::LETTERBOX_HEIGHT;

#[derive(Component)]
pub struct LetterboxTop;

#[derive(Component)]
pub struct LetterboxBottom;

#[derive(Component)]
pub struct LetterboxSide;

#[derive(Component)]
pub struct Letterbox;

#[derive(Resource)]
pub struct LetterboxState {
    pub active: bool,
    pub height: f32,
}

impl Default for LetterboxState {
    fn default() -> Self {
        Self {
            active: true,
            height: LETTERBOX_HEIGHT,
        }
    }
}
