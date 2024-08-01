use bevy::{
    asset::AssetServer,
    prelude::{default, BuildChildren, Commands, EntityRef, NodeBundle, Query, Res, With, Without},
    ui::{JustifyContent, Style, Val},
};

use super::{letterbox::resources::Letterbox, subtitles::resources::Subtitles};

pub fn poststartup_nest_elements(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_letterbox: Query<EntityRef, With<Letterbox>>,
    q_subtitles: Query<(EntityRef, &Subtitles), Without<Letterbox>>,
) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: bevy::ui::Display::Block,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // these probably don't need to be parented
            // parent.spawn(letterbox);
        })
        .add_child(q_letterbox.single().id())
        .add_child(q_subtitles.single().0.id());
}
