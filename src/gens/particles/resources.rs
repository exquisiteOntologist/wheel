use bevy::prelude::Component;
use bevy_hanabi::Spawner;

#[derive(Component)]
pub struct MyParticleSpawner {
    pub spawner: Spawner,
}
