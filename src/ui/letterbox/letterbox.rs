use bevy::{
    prelude::{default, BuildChildren, Commands, Query, Res, ResMut},
    time::Time,
    ui::{BackgroundColor, JustifyContent, Node, PositionType, Val},
};

use super::{
    constants::{COLOR_BLACK, LETTERBOX_HEIGHT},
    resources::{Letterbox, LetterboxBottom, LetterboxSide, LetterboxState, LetterboxTop},
};

pub fn letterbox_setup(mut commands: Commands) {
    let letterbox_style = Node {
        width: Val::Percent(100.0),
        height: Val::Px(LETTERBOX_HEIGHT),
        position_type: PositionType::Absolute,
        left: Val::Px(0.),
        ..default()
    };

    let node_background: BackgroundColor = COLOR_BLACK.into();

    let top = (
        Node {
            top: Val::Px(0.),
            ..letterbox_style.clone()
        },
        node_background.clone(),
        LetterboxTop,
        LetterboxSide,
    );

    let bottom = (
        Node {
            bottom: Val::Px(0.),
            ..letterbox_style
        },
        node_background,
        LetterboxBottom,
        LetterboxSide,
    );

    let box_top = commands.spawn(top).id();
    let box_bottom = commands.spawn(bottom).id();
    let mut letterbox = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        },
        Letterbox,
    ));

    letterbox.add_child(box_top);
    letterbox.add_child(box_bottom);
}

pub fn update_letterbox(
    time: Res<Time>,
    mut q_boxes: Query<(&mut LetterboxSide, &mut Node)>,
    mut lb_state: ResMut<LetterboxState>,
) {
    // there are two letterbox bars this loop goes over
    for (_letterbox, mut node) in &mut q_boxes {
        // to be letterbox, not fully letterbox
        if lb_state.active && lb_state.height < LETTERBOX_HEIGHT {
            lb_state.height += 50. * time.delta_secs();
        }
        // to not be letterbox, letterbox still visible
        else if !lb_state.active && lb_state.height > 0. {
            lb_state.height -= 50. * time.delta_secs();
        }

        node.height = Val::Px(lb_state.height);
    }
}
