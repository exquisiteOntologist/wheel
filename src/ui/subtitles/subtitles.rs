use bevy::{
    asset::AssetServer,
    color::Color,
    prelude::{
        default, BuildChildren, Commands, NodeBundle, Parent, Query, Res, ResMut, TextBundle, With,
        Without,
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

    // To change dynamically
    let subtitle_content = "";
    let subtitle_text_style = TextStyle {
        font: asset_server.load(FONT_PATH),
        font_size: 32.0,
        color: Color::WHITE,
        ..default()
    };

    let subtitle_style = Style {
        align_items: AlignItems::Center,
        align_content: AlignContent::Center,
        max_width: Val::Percent(100.),
        bottom: Val::Px(0.),
        left: Val::Px(0.),
        ..default()
    };

    let sub_text_bundle = (
        TextBundle {
            text: Text::from_section(subtitle_content, subtitle_text_style.clone())
                .with_justify(JustifyText::Center),
            style: Style {
                ..subtitle_style.clone()
            },
            ..default()
        },
        SubtitleText,
    );

    let sub_text = commands.spawn(sub_text_bundle).id();

    let subtitle_text_shade_style = TextStyle {
        color: Color::srgba(0., 0., 0., 0.5),
        ..subtitle_text_style
    };

    let sub_text_shade_bundle = (
        TextBundle {
            text: Text::from_section(subtitle_content, subtitle_text_shade_style)
                .with_justify(JustifyText::Center),
            style: Style {
                bottom: Val::Px(-1.),
                left: Val::Px(1.),
                position_type: bevy::ui::PositionType::Absolute,
                ..subtitle_style
            },
            ..default()
        },
        SubtitleText,
    );

    let sub_text_shade = commands.spawn(sub_text_shade_bundle).id();

    let sub_text_wrapper_bundle = NodeBundle {
        style: Style {
            display: Display::Block,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let sub_text_wrapper = commands
        .spawn(sub_text_wrapper_bundle)
        .add_child(sub_text_shade)
        .add_child(sub_text)
        .id();

    let subtitle_bundle = NodeBundle {
        style: Style {
            max_width: Val::Percent(70.),
            max_height: Val::Px(100.),
            bottom: Val::Percent(100.),
            padding: UiRect::axes(Val::Px(30.), Val::Px(10.)),
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            // To change dynamically
            display: Display::None,
            ..default()
        },
        background_color: Color::srgba(0., 0., 0., 0.).into(),
        border_radius: BorderRadius::all(Val::Px(3.)),
        ..default()
    };

    let subtitle = commands
        .spawn((subtitle_bundle, Subtitle))
        .add_child(quote_left)
        .add_child(sub_text_wrapper)
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
    mut q_boxes: Query<(&mut Subtitle, &mut Style, &Parent, &Node), With<Subtitle>>,
    mut q_text: Query<
        (&mut SubtitleText, &mut Text, &mut Style, &Parent, &Node),
        Without<Subtitle>,
    >,
    mut subtitles_state: ResMut<SubtitlesState>,
) {
    for (mut letterbox, mut style, parent, box_node) in &mut q_boxes {
        style.display = Display::Flex;
    }

    // There are 2 Subtitle nodes; the foreground and background shade.
    // This loop will set the text for both.
    for (mut letterbox, mut text, mut style, parent, box_node) in &mut q_text {
        // we only use 1 section at a time
        text.sections[0].value = "Then, whispy and mean, the wind took them".into();
    }
}
