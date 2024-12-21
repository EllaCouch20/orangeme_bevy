use bevy::prelude::*;

use crate::{
    primitives::{
        profile_photo::profile_photo,
        button_presets::primary_default,
        button::{ButtonComponent, CustomButton},
    },
    theme::{color::Display, fonts::FontResources, icons::Icon},
    utils::{EXPAND, MAX},
    NavigateTo,
};

// ===== Bumper Instantiation ===== //

pub struct Bumper {
    bumper_content_node: Node,
    bumper_node: Node,
}

impl Default for Bumper {
    fn default() -> Self {
        Self::new()
    }
}

impl Bumper {
    pub fn new() -> Self {
        Self {
            bumper_content_node: Node {
                width: EXPAND,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(8.0),
                padding: UiRect {
                    top: Val::Px(16.0),
                    bottom: Val::Px(16.0),
                    left: Val::Px(24.0),
                    right: Val::Px(24.0)
                },
                ..default()
            },
            bumper_node: Node {
                width: EXPAND,
                max_width: MAX,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
        }
    }

    // ===== Bumper Unlimited Buttons ===== //

    pub fn button_bumper(
        self,
        parent: &mut ChildBuilder,
        fonts: &Res<FontResources>,
        asset_server: &Res<AssetServer>,
        buttons: Vec<CustomButton>,
    ) {
        let colors = Display::new();
        parent.spawn(self.bumper_node).with_children(|parent| {
            parent.spawn(self.bumper_content_node).with_children(|child| {
                for button in buttons {
                    ButtonComponent::spawn_button(child, asset_server, fonts, button);
                }
            });
        });
    }
}