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
){
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
        RadioButton,
        Button,
    )).with_children(|parent| { 
        parent.spawn((
            Icon::new(Icon::Radio, asset_server),
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
        }).with_children(|parent| {
            parent.spawn(
                text(title, title_font, title_size, colors.text_heading),
            );
            parent.spawn(
                text(subtitle, sub_font, sub_size, colors.text_secondary),
            );
        });  
    });  
}
