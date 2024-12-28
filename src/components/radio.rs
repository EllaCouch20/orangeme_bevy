use bevy::prelude::*;

use crate::Theme;
use crate::utils::{EXPAND, text};
use crate::bitcoin::speed::RadioButtonState;

// ===== System Updating Display ===== //

#[derive(Component)]
pub struct RadioButton;
pub fn radio_button(
    parent: &mut ChildBuilder,
    theme: &Res<Theme>,
    title: &str,
    subtitle: &str,
    selected: bool,
) {
    let title_font = theme.fonts.style.heading.clone();
    let title_size = theme.fonts.size.h5;

    let sub_font = theme.fonts.style.text.clone();
    let sub_size = theme.fonts.size.xs;

    parent.spawn((
        Node {
            width: EXPAND,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.0),
            padding: UiRect {
                top: Val::Px(16.0),
                bottom: Val::Px(16.0),
                ..default()
            },
            ..default()
        },
        Button,
        RadioButtonState { selected },
        RadioButton,
    ))
    .with_children(|parent| {
        parent.spawn((

            if selected {
                theme.icons.radio_filled()
            } else {
                theme.icons.radio()
            },

            Node {
                height: Val::Px(32.0),
                width: Val::Px(32.0),
                ..default()
            },
        ));

        parent.spawn(Node {
            width: EXPAND,
            height: EXPAND,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(text(title, title_font, title_size, theme.colors.text_heading));
            parent.spawn(text(subtitle, sub_font, sub_size, theme.colors.text_secondary));
        });
    });
}

pub fn toggle_radio_buttons(
    theme: Res<Theme>,
    mut interaction_query: Query<(
        &Interaction,
        &mut RadioButtonState,
    ), (Changed<Interaction>, With<Button>)>,
    mut image_query: Query<(&mut ImageNode, &Parent)>,
    parent_query: Query<&RadioButton>,
) {
    for (interaction, mut state) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            for (mut image_node, parent) in &mut image_query {
                if let Ok(_radio_button_state) = parent_query.get(parent.get()) {
                    state.selected = !state.selected;
                    *image_node = theme.icons.radio_filled();
                    if state.selected {
                        *image_node = theme.icons.radio();
                    }
                }
            }
        }
    }
}
