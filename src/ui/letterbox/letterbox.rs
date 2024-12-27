use bevy::{
    prelude::{default, BuildChildren, Commands, Parent, Query, Res, ResMut},
    time::Time,
    ui::{BackgroundColor, ComputedNode, JustifyContent, Node, PositionType, Val},
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

    // let top = NodeBundle {
    //     style: Style {
    //         top: Val::Px(0.),
    //         ..letterbox_style.clone()
    //     },
    //     background_color: node_background.clone(),
    //     ..default()
    // };
    let top = (
        Node {
            top: Val::Px(0.),
            ..letterbox_style.clone()
        },
        node_background.clone(),
        LetterboxTop,
        LetterboxSide,
    );

    // let bottom = NodeBundle {
    //     style: Style {
    //         bottom: Val::Px(0.),
    //         ..letterbox_style
    //     },
    //     background_color: node_background,
    //     ..default()
    // };
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
    mut q_boxes: Query<(&mut LetterboxSide, &mut Node, &ComputedNode, &Parent, &Node)>,
    lb_state: ResMut<LetterboxState>,
) {
    for (_letterbox, mut node, computed_node, _parent, _box_node) in &mut q_boxes {
        let lb_height = computed_node.size().y;
        // println!("lb height {}", lb_height);
        // println!("active {}", lb_state.active);
        // println!("air delta {}", time.delta_secs());

        // to be letterbox, not fully letterbox
        if lb_state.active && lb_height < LETTERBOX_HEIGHT {
            let n_h = lb_height + (50. * time.delta_secs());
            node.height = Val::Px(n_h);
        }
        // to not be letterbox, letterbox still visible
        else if !lb_state.active && lb_height > 0. {
            let n_h = lb_height - (50. * time.delta_secs());
            node.height = Val::Px(n_h);
        }
    }
}
