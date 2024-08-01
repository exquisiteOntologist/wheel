use bevy::app::{App, Plugin, PostStartup};

use super::{letterbox::plugin::LetterboxPlugin, setup::poststartup_nest_elements};

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LetterboxPlugin);
        app.add_systems(PostStartup, poststartup_nest_elements);
    }
}
