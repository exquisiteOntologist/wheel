use bevy::app::{App, Plugin, Startup, Update};

use super::{resources::SubtitlesState, subtitles::subtitles_setup, subtitles::update_subtitles};

pub struct SubtitlesPlugin;

impl Plugin for SubtitlesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SubtitlesState>();
        app.add_systems(Startup, subtitles_setup);
        app.add_systems(Update, update_subtitles);
    }
}
