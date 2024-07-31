use bevy::app::{App, Plugin, Startup, Update};

use super::{
    letterbox::{update_letterbox, LetterboxState},
    setup::setup,
};

pub struct UserInterfacePlugin;

impl Plugin for UserInterfacePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LetterboxState>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_letterbox);
    }
}
