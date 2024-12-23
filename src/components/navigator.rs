use bevy::prelude::*;

use crate::{
    theme::{
        fonts::FontResources,
        color::Display,
        icons::Icon,
    },
    primitives::{
        button_presets::{nav_button, nav_button_pfp},
        button::ButtonComponent,
    },
    utils::{EXPAND, spacer},
    Nav,
    InteractiveState,
};

// ===== Desktop Sidebar Navigation ===== //

pub fn sidebar_navigator (
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>, 
) {

    let font = fonts.style.label.clone();
    let font_size = fonts.size.title;
    let colors = Display::new();

    parent.spawn(Node {
        width: Val::Px(300.0),
        height: EXPAND,
        align_items: AlignItems::Start,
        justify_content: JustifyContent::Start,
        ..default()
    }).with_children(|parent| {

        // ===== Instanitate Buttons ===== //

        let wallet = nav_button("Bitcoin", Icon::Wallet);
        let message = nav_button("Message", Icon::Message);
        let profile = nav_button_pfp("Ella Couch");

        parent.spawn((
            Node {
                width: EXPAND,
                height: EXPAND,
                border: UiRect::right(Val::Px(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(32.0), 
                padding: UiRect {
                    top: Val::Px(32.0),
                    bottom: Val::Px(32.0),
                    left: Val::Px(16.0),
                    right: Val::Px(16.0),
                    ..default()
                },
                ..default()
            },
            BorderColor(colors.outline_secondary),
        )).with_children(|child| {

            // ===== orange Logo ===== //

            child.spawn((
                ImageNode::new(asset_server.load("wordmark.png")),
                Node { 
                    width: Val::Px(90.0), 
                    ..default() 
                },
            ));

            // ===== Button List ===== //

            child.spawn(Node {
                width: EXPAND,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0), 
                ..default()
            }).with_children(|child| {
                ButtonComponent::spawn_button(child, asset_server, fonts, wallet);
                ButtonComponent::spawn_button(child, asset_server, fonts, message);
            });

            spacer(child);

            // ===== Profile Button ===== //
            
            child.spawn(Node { 
                width: EXPAND, ..default()
            }).with_children(|child| {
                ButtonComponent::spawn_button(child, asset_server, fonts, profile);
            });
        });
    });
}
