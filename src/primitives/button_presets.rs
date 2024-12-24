use bevy::prelude::*;
use crate::theme::fonts::FontResources;
use crate::theme::color::Display;
use bevy_ui::prelude::*;

use crate::primitives::button::{
    CustomButton, 
    ButtonWidth, 
    ButtonComponent, 
    ButtonSize, 
    InteractiveState, 
    ButtonStyle,
};

use crate::theme::icons::Icon;

use crate::Nav;

// ===== Commonplace Button Usecases ===== //

pub fn nav_button_pfp (mut label: &str) -> CustomButton {
    label = if label.len() > 10 {
        if let Some(pos) = label.find(' ') {
            &label[..pos]
        } else {
            &label[..10]
        }
    } else {
        label
    };
    
    CustomButton::new(
        label,
        None,
        Some("profile_photo.png".to_string()),
        ButtonStyle::Ghost,
        ButtonWidth::Expand,
        ButtonSize::Large,
        InteractiveState::Default,
        JustifyContent::Start,
    )
}


pub fn nav_button (label: &str, icon: Icon) -> CustomButton {
    CustomButton::new(
        label,
        Some(icon),
        None,
        ButtonStyle::Ghost,
        ButtonWidth::Expand,
        ButtonSize::Large,
        InteractiveState::Default,
        JustifyContent::Start,
    )
}

pub fn primary_default(label: &str) -> CustomButton {
    CustomButton::new(
        label,
        None,
        None,
        ButtonStyle::Primary,
        ButtonWidth::Expand,
        ButtonSize::Large,
        InteractiveState::Default,
        JustifyContent::Center,
    )
}

pub fn primary_disabled(label: &str) -> CustomButton {
    CustomButton::new(
        label,
        None,
        None,
        ButtonStyle::Primary,
        ButtonWidth::Expand,
        ButtonSize::Large,
        InteractiveState::Disabled,
        JustifyContent::Center,
    )
}

pub fn secondary_wide(label: &str) -> CustomButton {
    CustomButton::new(
        label,
        None,
        None,
        ButtonStyle::Secondary,
        ButtonWidth::Expand,
        ButtonSize::Large,
        InteractiveState::Default,
        JustifyContent::Center,
    )
}

pub fn secondary_default(label: &str, icon: Icon) -> CustomButton {
    CustomButton::new(
        label,
        Some(icon),
        None,
        ButtonStyle::Secondary,
        ButtonWidth::Hug,
        ButtonSize::Medium,
        InteractiveState::Default,
        JustifyContent::Center,
    )
}