use bevy::{
    asset::AssetServer,
    color::Color,
    prelude::{
        default, BuildChildren, Commands, ImageNode, Parent, Query, Res, ResMut, Text, With,
        Without,
    },
    text::{JustifyText, TextColor, TextFont, TextLayout},
    time::Time,
    ui::{
        AlignContent, AlignItems, BackgroundColor, BorderRadius, Display, JustifyContent, Node,
        UiRect, Val,
    },
};

use super::{
    constants::FONT_PATH,
    resources::{Subtitle, SubtitleText, Subtitles, SubtitlesState},
};

pub fn subtitles_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let quote_node = Node {
        width: Val::Px(22. / 2.),
        height: Val::Px(19. / 2.),
        ..default()
    };

    let quote_left_bundle = (
        Node {
            margin: UiRect::new(Val::Px(0.), Val::Px(10.), Val::Px(0.), Val::Px(16.)),
            ..quote_node.clone()
        },
        ImageNode::new(asset_server.load("glyphs/quote_left@2x.png")),
    );

    let quote_right_bundle = (
        Node {
            margin: UiRect::new(Val::Px(12.), Val::Px(0.), Val::Px(0.), Val::Px(16.)),
            ..quote_node
        },
        ImageNode::new(asset_server.load("glyphs/quote_right@2x.png")),
    );

    let quote_left = commands.spawn(quote_left_bundle).id();
    let quote_right = commands.spawn(quote_right_bundle).id();

    // To change dynamically
    let subtitle_content = "";
    let subtitle_text_font = TextFont {
        font: asset_server.load(FONT_PATH),
        font_size: 32.0,
        ..default()
    };
    let subtitle_text_colour = TextColor::WHITE;
    let subtitle_text_layout = TextLayout {
        justify: JustifyText::Center,
        ..default()
    };

    let subtitle_node = Node {
        align_items: AlignItems::Center,
        align_content: AlignContent::Center,
        max_width: Val::Percent(100.),
        bottom: Val::Px(0.),
        left: Val::Px(0.),
        ..default()
    };

    // let subtitle_color = TextColor { ..default() };

    let sub_text_bundle = (
        Text::new(subtitle_content),
        subtitle_text_layout,
        subtitle_text_font.clone(),
        // subtitle_color,
        subtitle_text_colour,
        subtitle_node.clone(),
        SubtitleText,
    );

    let sub_text = commands.spawn(sub_text_bundle).id();

    let subtitle_text_shade_font = TextFont {
        ..subtitle_text_font.clone()
    };
    let subtitle_text_shade_colour = Color::srgba(0., 0., 0., 0.5);

    let sub_text_shade_bundle = (
        Text::from(subtitle_content),
        subtitle_text_shade_font,
        TextColor::from(subtitle_text_shade_colour),
        subtitle_text_layout,
        Node {
            bottom: Val::Px(-1.),
            left: Val::Px(1.),
            position_type: bevy::ui::PositionType::Absolute,
            ..subtitle_node.clone()
        },
        SubtitleText,
    );

    let sub_text_shade = commands.spawn(sub_text_shade_bundle).id();

    let sub_text_wrapper_bundle = Node {
        display: Display::Block,
        justify_content: JustifyContent::Center,
        ..default()
    };

    let sub_text_wrapper = commands
        .spawn(sub_text_wrapper_bundle)
        .add_child(sub_text_shade)
        .add_child(sub_text)
        .id();

    let subtitle_background = Color::srgba(0., 0., 0., 0.);
    let subtitle_radius = BorderRadius::all(Val::Px(3.));
    let subtitle_bundle = (
        Node {
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
        BackgroundColor::from(subtitle_background),
        subtitle_radius,
        Subtitle,
    );

    let subtitle = commands
        .spawn(subtitle_bundle)
        .add_child(quote_left)
        .add_child(sub_text_wrapper)
        .add_child(quote_right)
        .id();

    let subtitles_bundle = Node {
        width: Val::Percent(100.),
        height: Val::Percent(30.),
        bottom: Val::Percent(30.),
        display: Display::Flex,
        justify_content: JustifyContent::Center,
        ..default()
    };

    commands
        .spawn((subtitles_bundle, Subtitles))
        .add_child(subtitle);
}

pub fn update_subtitles(
    _time: Res<Time>,
    mut q_boxes: Query<(&mut Subtitle, &mut Node, &Parent), With<Subtitle>>,
    mut q_text: Query<(&mut SubtitleText, &mut Text, &mut Node, &Parent), Without<Subtitle>>,
    subtitles_state: ResMut<SubtitlesState>,
) {
    for (_letterbox, mut node, _parent) in &mut q_boxes {
        node.display = match subtitles_state.text.len() {
            0 => Display::None,
            _ => Display::Flex,
        }
    }

    // There are 2 Subtitle nodes; the foreground and background shade.
    // This loop will set the text for both.
    for (_letterbox, mut text_query, _style, _parent) in &mut q_text {
        // we only use 1 section at a time
        text_query.clear();
        text_query.push_str(
            subtitles_state
                .text
                .first()
                .or(Some(&String::new()))
                .unwrap(),
        );
    }
}
