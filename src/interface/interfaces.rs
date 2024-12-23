use bevy::prelude::*;
use crate::primitives::profile_photo::profile_photo;
use crate::utils::EXPAND;

use crate::theme::{
    color::Display,
    fonts::FontResources,
    icons::Icon,
};

use crate::Nav;
use crate::primitives::button::ButtonComponent;
use crate::primitives::button::CustomButton;

pub struct Interface {
    pub node: Node,
    pub page_node: Node,
    pub content: Node,
    pub content_centered: Node,
}

impl Default for Interface {
    fn default() -> Self {
        Self::new()
    }
}

// ===== Commonplace Interface Nodes ===== //

impl Interface {
    pub fn new() -> Self {
        Self {
            node: Node {
                width: EXPAND,
                height: EXPAND,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            page_node: Node {
                width: EXPAND,
                height: EXPAND,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            content: Node {
                width: EXPAND,
                height: EXPAND,
                max_width: Val::Px(512.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(24.0),
                padding: UiRect {
                    left: Val::Px(24.0),
                    right: Val::Px(24.0),
                    top: Val::Px(16.0),
                    bottom: Val::Px(16.0),
                },
                ..default()
            },
            content_centered: Node {
                width: EXPAND,
                height: EXPAND,
                max_width: Val::Px(512.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(24.0),
                ..default()
            },
        }
    }
}
