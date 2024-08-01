use bevy::{
    asset::AssetServer,
    prelude::{default, BuildChildren, Commands, EntityRef, NodeBundle, Query, Res, With},
    ui::{JustifyContent, Style, Val},
};

use super::letterbox::resources::Letterbox;

pub fn poststartup_nest_elements(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    q_letterbox: Query<EntityRef, With<Letterbox>>,
) {
    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // these probably don't need to be parented
            // parent.spawn(letterbox);
        })
        .add_child(q_letterbox.single().id());
}
