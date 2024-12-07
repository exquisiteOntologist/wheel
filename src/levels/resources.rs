use bevy::prelude::{Resource, States, SystemSet};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelLoadSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SceneId {
    LevelSandHills,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrentSceneState {
    SceneSandHills,
}

#[derive(Resource)]
pub struct LevelState {
    /// Level the player is in
    pub level: SceneId,
    /// Whether the level is initialising
    pub init: bool,
}

impl Default for LevelState {
    fn default() -> Self {
        Self {
            level: SceneId::LevelSandHills,
            init: true,
        }
    }
}
