use bevy::{app::AppExit, color::palettes::css::CRIMSON, prelude::*};

use super::despawn_screen;
use bevy_ui::prelude::*;

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
use crate::theme::color::Display;
use crate::theme::fonts::{FontResources, FontSizes, Style, setup_fonts};
use crate::interface::header::Header;
use crate::primitives::profile_photo::profile_photo;

// ==== Components ==== //

use crate::components::{
    balance_display::balance_display,
    navigator::sidebar_navigator,
};


#[derive(Component)]
pub struct OnHomeScreen;

pub fn home_setup(mut commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontResources>) {

    let colors = Display::new();

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        OnHomeScreen,
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
        ))
        .with_children(|parent| {
            let mut header = Header::new(
                "Wallet",
                fonts.size.h3,
                None,
                None,
            );
            
            header.header_home(parent, &fonts, &asset_server);

            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    max_width: Val::Px(512.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            )).with_children(|parent| {
                balance_display(parent, &fonts);
            });

            parent.spawn((Node {
                width: Val::Percent(100.0),
                max_width: Val::Px(512.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            })).with_children(|parent| {
    
                let send = primary_default("Send", true, NavigateTo::Address);
                let receive = primary_default("Receive", true, NavigateTo::BackToMainMenu);
    
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
                    ButtonComponent::spawn_button(child, &asset_server, &fonts, receive);
                    ButtonComponent::spawn_button(child, &asset_server, &fonts, send);
                });
            });
        });
    });
}
