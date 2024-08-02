use bevy::{
    asset::AssetServer,
    color::Color,
    prelude::{
        default, BuildChildren, Commands, NodeBundle, Parent, Query, Res, ResMut, TextBundle,
    },
    text::{JustifyText, Text, TextStyle},
    time::Time,
    ui::{
        AlignContent, AlignItems, BorderRadius, Display, JustifyContent, Node, Style, UiImage,
        UiRect, Val,
    },
};

use super::{
    constants::FONT_PATH,
    resources::{Subtitle, SubtitleText, Subtitles, SubtitlesState},
};

pub fn subtitles_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let quote_style = Style {
        width: Val::Px(22. / 2.),
        height: Val::Px(19. / 2.),
        ..default()
    };

    let quote_left_bundle = (
        NodeBundle {
            style: Style {
                margin: UiRect::new(Val::Px(0.), Val::Px(10.), Val::Px(0.), Val::Px(16.)),
                ..quote_style.clone()
            },
            ..default()
        },
        UiImage::new(asset_server.load("glyphs/quote_left@2x.png")),
    );

    let quote_right_bundle = (
        NodeBundle {
            style: Style {
                margin: UiRect::new(Val::Px(12.), Val::Px(0.), Val::Px(0.), Val::Px(16.)),
                ..quote_style
            },
            ..default()
        },
        UiImage::new(asset_server.load("glyphs/quote_right@2x.png")),
    );

    let quote_left = commands.spawn(quote_left_bundle).id();
    let quote_right = commands.spawn(quote_right_bundle).id();

    let sub_text_bundle = (
        TextBundle {
            text: Text::from_section(
                "Then, whispy and mean, the wind took them",
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 32.0,
                    ..default()
                },
            )
            .with_justify(JustifyText::Center),
            style: Style {
                align_items: AlignItems::Center,
                align_content: AlignContent::Center,
                max_width: Val::Percent(100.),
                // max_height: Val::Px(50.),
                bottom: Val::Percent(0.),
                ..default()
            },
            ..default()
        },
        SubtitleText,
    );

    let sub_text = commands.spawn(sub_text_bundle).id();

    let subtitle_bundle = NodeBundle {
        style: Style {
            max_width: Val::Percent(70.),
            // max_width: Val::Px(300.),
            max_height: Val::Px(100.),
            bottom: Val::Percent(100.),
            padding: UiRect::axes(Val::Px(30.), Val::Px(10.)),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            ..default()
        },
        background_color: Color::srgba(0., 0., 0., 0.).into(),
        border_radius: BorderRadius::all(Val::Px(3.)),
        ..default()
    };

    let subtitle = commands
        .spawn((subtitle_bundle, Subtitle))
        .add_child(quote_left)
        .add_child(sub_text)
        .add_child(quote_right)
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
