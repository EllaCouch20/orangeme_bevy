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
    button_system,
};

use crate::theme::icons::Icon;

use crate::NavigateTo;


pub fn nav_button (label: &str, status: InteractiveState, icon: Icon) -> CustomButton {
    CustomButton::new(
        label,
        Some(icon),
        None,
        ButtonStyle::Ghost,
        ButtonWidth::Expand,
        ButtonSize::Large,
        status,
        NavigateTo::Home,
        JustifyContent::Start,
        true,
        false,
    )
}


pub fn nav_button_pfp (mut label: &str, status: InteractiveState) -> CustomButton {
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
        status,
        NavigateTo::Home,
        JustifyContent::Start,
        true,
        false,
    )
}

pub fn primary_default(label: &str, disabled: bool, state: InteractiveState, navigate_to: NavigateTo) -> CustomButton {
    return CustomButton::new(
        label,
        None,
        None,
        ButtonStyle::Primary,
        ButtonWidth::Expand,
        ButtonSize::Large,
        state,
        navigate_to,
        JustifyContent::Center,
        false,
        disabled,
    );
}

pub fn secondary_default(label: &str, icon: Icon, navigate_to: NavigateTo) -> CustomButton {
    return CustomButton::new(
        label,
        Some(icon),
        None,
        ButtonStyle::Secondary,
        ButtonWidth::Hug,
        ButtonSize::Medium,
        InteractiveState::Default,
        navigate_to,
        JustifyContent::Center,
        false,
        false,
    );
}