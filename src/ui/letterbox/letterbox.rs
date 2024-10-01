use bevy::{
    prelude::{default, BuildChildren, Commands, NodeBundle, Parent, Query, Res, ResMut},
    time::Time,
    ui::{BackgroundColor, JustifyContent, Node, PositionType, Style, Val},
};

use super::{
    constants::{COLOR_BLACK, LETTERBOX_HEIGHT},
    resources::{Letterbox, LetterboxBottom, LetterboxSide, LetterboxState, LetterboxTop},
};

pub fn letterbox_setup(mut commands: Commands) {
    let letterbox_style = Style {
        width: Val::Percent(100.0),
        height: Val::Px(LETTERBOX_HEIGHT),
        position_type: PositionType::Absolute,
        left: Val::Px(0.),
        ..default()
    };

    let node_background: BackgroundColor = COLOR_BLACK.into();

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

    let box_top = commands.spawn((top, LetterboxTop, LetterboxSide)).id();
    let box_bottom = commands
        .spawn((bottom, LetterboxBottom, LetterboxSide))
        .id();
    let mut letterbox = commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        },
        Letterbox,
    ));

    letterbox.add_child(box_top);
    letterbox.add_child(box_bottom);
}

pub fn update_letterbox(
    time: Res<Time>,
    mut q_boxes: Query<(&mut LetterboxSide, &mut Style, &Parent, &Node)>,
    lb_state: ResMut<LetterboxState>,
) {
    for (letterbox, mut style, parent, box_node) in &mut q_boxes {
        let lb_height = box_node.size().y;
        // println!("lb height {}", lb_height);
        // println!("active {}", lb_state.active);
        // println!("air delta {}", time.delta_seconds());

        // to be letterbox, not fully letterbox
        if lb_state.active && lb_height < LETTERBOX_HEIGHT {
            let n_h = lb_height + (50. * time.delta_seconds());
            style.height = Val::Px(n_h);
        }
        // to not be letterbox, letterbox still visible
        else if !lb_state.active && lb_height > 0. {
            let n_h = lb_height - (50. * time.delta_seconds());
            style.height = Val::Px(n_h);
        }
    }
}
