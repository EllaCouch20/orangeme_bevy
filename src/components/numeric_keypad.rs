use bevy::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy_ui::prelude::*;

use std::fmt::Write;

use crate::theme::fonts::FontResources;
use crate::theme::color::Display;
use crate::utils::{EXPAND, text};
use crate::StateData;
use crate::theme::icons::Icon;
use crate::components::amount_display::{AmountDisplayUsd, AmountDisplayZeros, AmountDisplayHelper};

// ===== System Updating Display ===== //

#[derive(Component)]
pub struct KeypadButton;

pub fn numeric_keypad(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
    key_str: Option<&str>,
    key_icon: Option<Icon>,
){
    let font = fonts.style.label.clone();
    let font_size = fonts.size.lg;

    let colors = Display::new();

    let row_node = Node {
        width: EXPAND,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(16.0), 
        ..default()
    };
    
    parent.spawn(Node {
        width: EXPAND,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(16.0), 
        ..default()
    }).with_children(|parent| { 
        parent.spawn(row_node.clone()).with_children(|parent| {
            keypad_button(parent, &fonts, &asset_server, Some("1"), None);
            keypad_button(parent, &fonts, &asset_server, Some("2"), None);
            keypad_button(parent, &fonts, &asset_server, Some("3"), None);
        });
        parent.spawn(row_node.clone()).with_children(|parent| {
            keypad_button(parent, &fonts, &asset_server, Some("4"), None);
            keypad_button(parent, &fonts, &asset_server, Some("5"), None);
            keypad_button(parent, &fonts, &asset_server, Some("6"), None);
        });
        parent.spawn(row_node.clone()).with_children(|parent| {
            keypad_button(parent, &fonts, &asset_server, Some("7"), None);
            keypad_button(parent, &fonts, &asset_server, Some("8"), None);
            keypad_button(parent, &fonts, &asset_server, Some("9"), None);
        });
        parent.spawn(row_node.clone()).with_children(|parent| {
            keypad_button(parent, &fonts, &asset_server, Some("."), None);
            keypad_button(parent, &fonts, &asset_server, Some("0"), None);
            keypad_button(parent, &fonts, &asset_server, None, Some(Icon::Back));
        });
    });  
}

pub fn keypad_button(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
    key_str: Option<&str>,
    key_icon: Option<Icon>,
){
    let font = fonts.style.label.clone();
    let font_size = fonts.size.lg;

    let colors = Display::new();
    
    parent.spawn((
        Node {
            width: EXPAND,
            height: Val::Px(48.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(8.0), 
            ..default()
        }, 
        KeypadButton,
        Button,
    )).with_children(|parent| { 

        // ===== Button Content ===== //

        if let Some(icon) = key_icon {
            parent.spawn((
                Icon::new(icon, asset_server),
                Node {
                    height: Val::Px(24.0),
                    width: Val::Px(24.0),
                    ..default()
                },
            ));
        } else if let Some(key) = key_str {
            parent.spawn(
                text(key, font, font_size, colors.text_heading),
            );
        }  
    });  
}