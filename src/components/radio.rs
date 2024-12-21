use bevy::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy_ui::prelude::*;

use std::fmt::Write;

use crate::theme::fonts::FontResources;
use crate::theme::color::Display;
use crate::utils::{EXPAND, cal_font_size, usd_to_btc};
use crate::StateData;

use crate::components::amount_display::{AmountDisplayUsd, AmountDisplayZeros, AmountDisplayHelper};

// ===== System Updating Display ===== //

pub struct RadioButton;

pub fn radio_button(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>
    title: &str,
    subtitle: &str,
    index: u8,
){
    let title_font = fonts.style.header.clone();
    let title_size = fonts.size.h5;

    let sub_font = fonts.style.text.clone();
    let sub_size = fonts.size.xs;

    let colors = Display::new();

    parent.spawn(
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
    ).with_children(|parent| { 
        parent.spawn(Node {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            ..default()
        }).with_child(|parent| {
            Icon::new(Icon::Radio, &asset_server)
        });
        parent.spawn(Node {
            width: EXPAND,
            height: EXPAND,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(4.0), 
            ..default()
        }).with_children(|child| {
            child.spawn((
                Text::new(usd),
                TextFont {
                    font: title_font,
                    font_size: title_size,
                    ..default()
                },
                TextColor(colors.text_heading),
            ));  
            child.spawn((
                Text::new(btc),
                TextFont {
                    font: sub_font,
                    font_size: sub_size,
                    ..default()
                },
                TextColor(colors.text_secondary),
            ));  
        });  
    });  
}
