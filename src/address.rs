use bevy::{prelude::*, ui::FocusPolicy};

use super::despawn_screen;
use crate::theme::color::Display;

use crate::primitives::button::{
    CustomButton, 
    ButtonWidth, 
    ButtonComponent, 
    ButtonSize, 
    InteractiveState, 
    ButtonStyle, 
    button_system,
    primary_default,
};


use crate::menu_plugin;

use crate::NavigateTo;
use crate::theme::icons::Icon;
use crate::theme::fonts::{FontResources, FontSizes, Style, setup_fonts};

use bevy_simple_text_input::{
    TextInput, 
    TextInputTextFont,
    TextInputTextColor,
    TextInputPlaceholder,
    TextInputInactive,
};

// ==== Components ==== //

use crate::components::{
    balance_display::balance_display,
    navigator::sidebar_navigator,
    text_input::text_input,
};


#[derive(Component)]
pub struct OnAddressScreen;

pub fn address_setup(mut commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontResources>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        OnAddressScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server);
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            Interaction::None,
        ))
        .with_children(|parent| {
            parent.spawn((Node {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                max_width: Val::Px(512.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                margin: UiRect {
                    right: Val::Px(24.0),
                    left: Val::Px(24.0),
                    top: Val::Px(16.0),
                    bottom: Val::Px(16.0),
                }, 
                ..default()
            })).with_children(|parent| {
                text_input(parent, &fonts);
            });

            parent.spawn((Node {
                width: Val::Percent(100.0),
                max_width: Val::Px(512.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            })).with_children(|parent| {
    
                let done = primary_default("Continue", true, NavigateTo::BackToMainMenu);
    
                parent.spawn(Node {
                    width: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    padding: UiRect {
                        top: Val::Px(16.0),
                        bottom: Val::Px(16.0),
                        left: Val::Px(24.0),
                        right: Val::Px(24.0),
                        ..default()
                    },
                    ..default()
                }).with_children(|child| {
                    ButtonComponent::spawn_button(child, &asset_server, &fonts, done);
                });
            });
        });
    });
}
