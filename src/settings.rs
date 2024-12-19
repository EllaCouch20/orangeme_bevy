use bevy::{app::AppExit, color::palettes::css::CRIMSON, prelude::*};
use bevy_ui::*;

use super::despawn_screen;

use crate::primitives::button::{
    CustomButton, 
    ButtonWidth, 
    ButtonComponent, 
    ButtonSize, 
    InteractiveState, 
    ButtonStyle, 
    button_system,
};

use crate::menu_plugin;

use crate::NavigateTo;
use crate::theme::icons::Icon;
use crate::theme::fonts::{FontResources, FontSizes, Style, setup_fonts};


#[derive(Component)]
pub struct OnSettingsMenuScreen;


pub fn settings_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontResources>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSettingsMenuScreen,
        ))
        .with_children(|parent| {
            let back_button = CustomButton::new(
                "Back",
                None,
                ButtonStyle::Primary,
                true,
                ButtonWidth::Hug,
                ButtonSize::Large,
                NavigateTo::BackToMainMenu,
            );

            parent.spawn(Node {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|child| {
                ButtonComponent::spawn_button(child, &asset_server, &fonts, back_button);
            });
        });
}
