use bevy::app::{App, Plugin, Startup, Update};

use super::{letterbox::letterbox_setup, letterbox::update_letterbox, resources::LetterboxState};

pub struct LetterboxPlugin;

impl Plugin for LetterboxPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LetterboxState>();
        app.add_systems(Startup, letterbox_setup);
        app.add_systems(Update, update_letterbox);
    }
}
