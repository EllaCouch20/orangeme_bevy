#![allow(unused)]

use bevy::prelude::*;
use crate::theme::{
    color::ButtonColor,
    fonts::FontResources,
    icons::Icon,
};
use crate::primitives::{profile_photo::profile_photo};
use crate::Nav;

use bevy_simple_text_input::TextInputInactive;
use bevy_simple_text_input::TextInputValue;

#[derive(Copy, Clone, Component, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Ghost,
}

#[derive(Copy, Clone, Component, PartialEq, Debug)]
pub enum InteractiveState {
    Default,
    Selected,
    Hover,
    Disabled,
}

pub enum ButtonSize {
    Medium,
    Large,
}

#[derive(PartialEq)]
pub enum ButtonWidth {
    Expand,
    Hug,
}

#[derive(Component)]
pub struct ButtonInteraction {
    pub state: InteractiveState,
    pub is_selected: bool,
}

#[derive(Component, PartialEq)]
pub enum SetState{Selectable, Disablable}

#[derive(Component)]
pub struct CustomButton {
    label: String,
    icon: Option<Icon>,
    photo: Option<String>,
    style: ButtonStyle,
    width_style: ButtonWidth,
    size: ButtonSize,
    pub state: InteractiveState,
    alignment: JustifyContent,
}

impl CustomButton {
    pub fn new(
        label: &str,
        icon: Option<Icon>,
        photo: Option<String>,
        style: ButtonStyle,
        width_style: ButtonWidth,
        size: ButtonSize,
        state: InteractiveState,
        alignment: JustifyContent,
    ) -> Self {
        Self {
            label: label.to_string(),
            icon,
            photo,
            style,
            width_style,
            size,
            state,
            alignment,
        }
    }
}



pub struct ButtonComponent;

impl ButtonComponent {
    pub fn spawn_button(
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        fonts: &Res<FontResources>,
        data: CustomButton,
    ) {

        let colors: ButtonColor = ButtonColor::new(data.style, data.state);
        let font = fonts.style.label.clone();

        // ===== Mapping Info From Size ===== //

        let (button_width, flex_grow) = match data.width_style {
            ButtonWidth::Expand => (Val::Percent(100.0), 1.0),
            ButtonWidth::Hug => (Val::Auto, 0.0),
        };

        let (height, padding, icon_size, icon_pad, font_size) = match data.size {
            ButtonSize::Large => (48.0, 24.0, 24.0, 12.0, fonts.size.lg),
            ButtonSize::Medium => (32.0, 12.0, 16.0, 4.0, fonts.size.md)
        };

        // ===== Button Background ===== //

        let mut button = parent.spawn((
            Button,
            Node {
                flex_grow,
                width: button_width,
                height: Val::Px(height),
                flex_basis: button_width,
                justify_content: data.alignment,
                column_gap: Val::Px(icon_pad),
                border: UiRect::all(Val::Px(1.0)),
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                padding: UiRect {
                    left: Val::Px(padding),
                    right: Val::Px(padding),
                    ..default()
                },
                ..default()
            },
            BorderColor(colors.outline),
            BorderRadius::MAX,
            BackgroundColor(colors.background),
        ));

        // ===== Button Innerds ===== //
        
        button.with_children(|button| {

            // ===== Optional Icon ===== //
            if let Some(icon) = data.icon {
                button.spawn((
                    Icon::new(data.icon.unwrap(), asset_server),
                    Node {
                        height: Val::Px(icon_size),
                        width: Val::Px(icon_size),
                        ..default()
                    },
                ));
            }

            // ===== Option Photo ===== //
            if let Some(photo) = data.photo.clone() {
                button.spawn(Node::default()).with_children(|parent| {
                    profile_photo(parent, fonts, asset_server, 32.0, &photo);
                });
            }

            // ===== Label ===== //
            button.spawn((
                Text::new(data.label.clone()),
                TextFont {font, font_size, ..default()},
                TextColor(colors.label),
            ));     
        });

        button.insert(data.style);
        button.insert(data.state);
        button.insert(data);
    }
}