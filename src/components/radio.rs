use bevy::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy_ui::prelude::*;

use std::fmt::Write;

use crate::theme::fonts::FontResources;
use crate::theme::color::Display;
use crate::utils::{EXPAND, text};
use crate::theme::icons::Icon;
use crate::StateData;
use crate::speed::RadioButtonState;

use crate::components::amount_display::{AmountDisplayUsd, AmountDisplayZeros, AmountDisplayHelper};

// ===== System Updating Display ===== //

#[derive(Component)]
pub struct RadioButton;
pub fn radio_button(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    colors: &Res<Display>,
    asset_server: &Res<AssetServer>,
    title: &str,
    subtitle: &str,
    index: u8,
    selected: bool,
) {
    let title_font = fonts.style.heading.clone();
    let title_size = fonts.size.h5;

    let sub_font = fonts.style.text.clone();
    let sub_size = fonts.size.xs;

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
                Icon::new(Icon::RadioFilled, asset_server)
            } else {
                Icon::new(Icon::Radio, asset_server)
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
            parent.spawn(text(title, title_font, title_size, colors.text_heading));
            parent.spawn(text(subtitle, sub_font, sub_size, colors.text_secondary));
        });
    });
}

pub fn toggle_radio_buttons(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<(
        Entity,
        &Interaction,
        &mut RadioButtonState,
    ), (Changed<Interaction>, With<Button>)>,
    mut image_query: Query<(&mut ImageNode, &Parent)>,
    parent_query: Query<&RadioButton>,
) {
    for (entity, interaction, mut state) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            for (mut image_node, parent) in &mut image_query {
                if let Ok(radio_button_state) = parent_query.get(parent.get()) {
                    state.selected = !state.selected;
                    *image_node = Icon::new(Icon::RadioFilled, &asset_server);
                    if state.selected {
                        *image_node = Icon::new(Icon::Radio, &asset_server);
                    }
                }
            }
        }
    }
}
