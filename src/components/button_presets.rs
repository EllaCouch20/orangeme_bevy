use bevy::prelude::*;

use crate::components::button::{
    CustomButton, 
    ButtonWidth,
    ButtonSize, 
    InteractiveState, 
    ButtonStyle,
};

// ===== Commonplace Button ===== //

pub fn nav_button_pfp(mut label: &str, preset: bool) -> CustomButton {
    label = if label.len() > 10 {
        if let Some(pos) = label.find(' ') {
            &label[..pos]
        } else {
            &label[..10]
        }
    } else {
        label
    };
    
    CustomButton {
        label: label.to_string(),
        size: ButtonSize::Large,
        style: ButtonStyle::Ghost,
        photo: Some("profile_photo.png".to_string()),
        icon: None,
        width: ButtonWidth::Expand,
        alignment: JustifyContent::Start,
        state: if preset {InteractiveState::Selected} else {InteractiveState::Default},
    }
}

pub fn nav_button(label: &str, icon: ImageNode, preset: bool) -> CustomButton {
    CustomButton {
        label: label.to_string(),
        icon: Some(icon),
        photo: None,
        style: ButtonStyle::Ghost,
        width: ButtonWidth::Expand,
        size: ButtonSize::Large,
        state: if preset {InteractiveState::Selected} else {InteractiveState::Default},
        alignment: JustifyContent::Start,
    }
}

pub fn primary_default(label: &str) -> CustomButton {
    CustomButton {
        label: label.to_string(),
        photo: None,
        icon: None,
        style: ButtonStyle::Primary,
        width: ButtonWidth::Expand,
        size: ButtonSize::Large,
        state: InteractiveState::Default,
        alignment: JustifyContent::Center,
    }
}

pub fn primary_disabled(label: &str) -> CustomButton {
    CustomButton {
        label: label.to_string(),
        photo: None,
        icon: None,
        style: ButtonStyle::Primary,
        width: ButtonWidth::Expand,
        size: ButtonSize::Large,
        state: InteractiveState::Disabled,
        alignment: JustifyContent::Center,
    }
}

pub fn secondary_wide(label: &str) -> CustomButton {
    CustomButton {
        label: label.to_string(),
        photo: None,
        icon: None,
        style: ButtonStyle::Secondary,
        width: ButtonWidth::Expand,
        size: ButtonSize::Large,
        state: InteractiveState::Default,
        alignment: JustifyContent::Center,
    }
}

pub fn secondary_default(label: &str, icon: ImageNode) -> CustomButton {
    CustomButton {
        label: label.to_string(),
        icon: Some(icon),
        photo: None,
        style: ButtonStyle::Secondary,
        width: ButtonWidth::Hug,
        size: ButtonSize::Medium,
        state: InteractiveState::Default,
        alignment: JustifyContent::Center,
    }
}

