use bevy::{
    asset::AssetServer,
    prelude::{default, BuildChildren, Commands, NodeBundle, Res},
    ui::{JustifyContent, Style, Val},
};

use super::letterbox::letterbox_setup;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let letterbox = letterbox_setup(&mut commands);

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
        .add_child(letterbox);
}
