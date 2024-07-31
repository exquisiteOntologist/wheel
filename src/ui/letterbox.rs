use bevy::{
    color::Color,
    prelude::{default, BuildChildren, Commands, Component, Entity, NodeBundle},
    ui::{BackgroundColor, JustifyContent, PositionType, Style, Val},
};

#[derive(Component)]
struct LetterboxTop {}

#[derive(Component)]
struct LetterboxBottom {}

#[derive(Component)]
struct Letterbox {}

pub fn letterbox_setup(commands: &mut Commands) -> Entity {
    let letterbox_style = Style {
        width: Val::Percent(100.0),
        height: Val::Px(100.0),
        position_type: PositionType::Absolute,
        left: Val::Px(0.),
        ..default()
    };

    let node_background: BackgroundColor = Color::srgb(0.4, 0.4, 1.).into();

    let top = NodeBundle {
        style: Style {
            top: Val::Px(0.),
            ..letterbox_style.clone()
        },
        background_color: node_background.clone(),
        ..default()
    };

    let bottom = NodeBundle {
        style: Style {
            bottom: Val::Px(0.),
            ..letterbox_style
        },
        background_color: node_background,
        ..default()
    };

    let box_top = commands.spawn((top /* LetterboxTop */,)).id();
    let box_bottom = commands.spawn((bottom /* LetterboxBottom */,)).id();
    let mut letterbox = commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        ..default()
    });

    letterbox.add_child(box_top);
    letterbox.add_child(box_bottom);

    letterbox.id()
}
