use bevy::{
    asset::AssetServer,
    color::Color,
    prelude::{
        default, BuildChildren, Commands, NodeBundle, Parent, Query, Res, ResMut, TextBundle,
    },
    text::{JustifyText, Text, TextStyle},
    time::Time,
    ui::{Display, JustifyContent, Node, Style, UiRect, Val},
};

use super::resources::{Subtitle, SubtitleText, Subtitles, SubtitlesState};

pub fn subtitles_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sub_text_bundle = (
        TextBundle {
            text: Text::from_section(
                "This is not a game or art",
                TextStyle {
                    font: asset_server
                        .load("fonts/Edu_AU_VIC_WA_NT_Hand/EduAUVICWANTHand-VariableFont_wght.ttf"),
                    font_size: 30.0,

                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            style: Style {
                align_items: bevy::ui::AlignItems::Center,
                align_content: bevy::ui::AlignContent::Center,
                max_width: Val::Px(300.),
                max_height: Val::Px(50.),
                ..default()
            },
            ..default()
        },
        SubtitleText,
    );

    let sub_text = commands.spawn(sub_text_bundle).id();

    let subtitle_bundle = NodeBundle {
        style: Style {
            max_width: Val::Px(300.),
            max_height: Val::Px(50.),
            bottom: Val::Percent(30.),
            padding: UiRect::axes(Val::Px(30.), Val::Px(10.)),
            align_items: bevy::ui::AlignItems::Center,
            align_content: bevy::ui::AlignContent::Center,
            ..default()
        },
        background_color: Color::srgba(0., 0., 0., 0.8).into(),
        ..default()
    };

    let subtitle = commands
        .spawn((subtitle_bundle, Subtitle))
        .add_child(sub_text)
        .id();

    let subtitles_bundle = NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(30.),
            bottom: Val::Percent(30.),
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    commands
        .spawn((subtitles_bundle, Subtitles))
        .add_child(subtitle);
}

pub fn update_subtitles(
    time: Res<Time>,
    mut q_boxes: Query<(&mut Subtitle, &mut Style, &Parent, &Node)>,
    mut subtitles_state: ResMut<SubtitlesState>,
) {
    for (mut letterbox, mut style, parent, box_node) in &mut q_boxes {
        let lb_height = box_node.size().y;
        //
    }
}
