use bevy::{
    asset::AssetServer,
    prelude::{default, BuildChildren, Commands, EntityRef, Query, Res, With, Without},
    ui::{Node, Val},
};

use super::{letterbox::resources::Letterbox, subtitles::resources::Subtitles};

pub fn poststartup_nest_elements(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    q_letterbox: Query<EntityRef, With<Letterbox>>,
    q_subtitles: Query<(EntityRef, &Subtitles), Without<Letterbox>>,
) {
    // root node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: bevy::ui::Display::Block,
            ..default()
        })
        .with_children(|_parent| {
            // these probably don't need to be parented
            // parent.spawn(letterbox);
        })
        .add_child(q_letterbox.single().id())
        .add_child(q_subtitles.single().0.id());
}
