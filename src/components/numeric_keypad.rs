use bevy::prelude::*;

use crate::Theme;
use crate::utils::{EXPAND, text};

// ===== System Updating Display ===== //

#[derive(Component)]
pub struct KeypadButton;

pub fn numeric_keypad(
    parent: &mut ChildBuilder,
    theme: &Res<Theme>,
){

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
            keypad_button(parent, &theme, Some("1"), None);
            keypad_button(parent, &theme, Some("2"), None);
            keypad_button(parent, &theme, Some("3"), None);
        });
        parent.spawn(row_node.clone()).with_children(|parent| {
            keypad_button(parent, &theme, Some("4"), None);
            keypad_button(parent, &theme, Some("5"), None);
            keypad_button(parent, &theme, Some("6"), None);
        });
        parent.spawn(row_node.clone()).with_children(|parent| {
            keypad_button(parent, &theme, Some("7"), None);
            keypad_button(parent, &theme, Some("8"), None);
            keypad_button(parent, &theme, Some("9"), None);
        });
        parent.spawn(row_node.clone()).with_children(|parent| {
            keypad_button(parent, &theme, Some("."), None);
            keypad_button(parent, &theme, Some("0"), None);
            keypad_button(parent, &theme, None, Some(theme.icons.back()));
        });
    });  
}

pub fn keypad_button(
    parent: &mut ChildBuilder,
    theme: &Res<Theme>,
    key_str: Option<&str>,
    key_icon: Option<ImageNode>,
){
    let font = theme.fonts.style.label.clone();
    let font_size = theme.fonts.size.lg;

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
                icon,
                Node {
                    height: Val::Px(24.0),
                    width: Val::Px(24.0),
                    ..default()
                },
            ));
        } else if let Some(key) = key_str {
            parent.spawn(
                text(key, font, font_size, theme.colors.text_heading),
            );
        }  
    });  
}