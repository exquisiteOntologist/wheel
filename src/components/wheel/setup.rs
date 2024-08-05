use bevy::prelude::{Commands, ResMut};

use crate::{constants::FORWARD_SPEED, resources::Game};

pub fn setup_wheel(_: Commands, mut game: ResMut<Game>) {
    game.player_wheel.speed_z = FORWARD_SPEED * 10.;
}
