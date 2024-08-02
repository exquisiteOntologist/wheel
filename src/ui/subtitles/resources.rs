use bevy::prelude::{Component, Resource};

#[derive(Component)]
pub struct SubtitleText;

#[derive(Component)]
pub struct Subtitle;

#[derive(Component)]
pub struct Subtitles;

#[derive(Resource, Default)]
pub struct SubtitlesState {
    pub text: Vec<String>,
}
