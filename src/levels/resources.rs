use bevy::prelude::{Resource, States, SystemSet};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelLoadSet;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelLoadSetPreUpdate;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelLoadSetUpdate;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct LevelLoadSetPostUpdate;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SceneId {
    LevelSandHills,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrentSceneState {
    SceneSandHills,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InitState {
    /// Never loaded (initial state)
    Uninitialised,
    /// Loading (loading level, changing level)
    Loading,
    /// Finished Loading (not loading)
    Finished,
}

#[derive(Resource)]
pub struct LevelState {
    /// Level the player is in
    pub level: SceneId,
    /// Whether the level is initialising
    pub init: InitState,
}

impl Default for LevelState {
    fn default() -> Self {
        Self {
            level: SceneId::LevelSandHills,
            init: InitState::Uninitialised,
        }
    }
}
