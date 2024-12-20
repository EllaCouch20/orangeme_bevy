use bevy::prelude::*;
use crate::theme::fonts::FontResources;
use crate::theme::color::Display;
use bevy_ui::prelude::*;

use crate::primitives::button_presets::{nav_button, nav_button_pfp};
use crate::primitives::button::ButtonComponent;
use crate::InteractiveState;
use crate::theme::icons::Icon;

use crate::NavigateTo;

pub fn sidebar_navigator (
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>, 
) {

    let font = fonts.style.label.clone();
    let font_size = fonts.size.title;
    let colors = Display::new();

    parent.spawn((
        Node {
            width: Val::Px(300.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            ..default()
        }
    ))
    .with_children(|parent| {

        let wallet = nav_button("Bitcoin", InteractiveState::Selected, Icon::Wallet);
        let message = nav_button("Message", InteractiveState::Default, Icon::Message);
        let profile = nav_button_pfp("Ella Couch", InteractiveState::Default);

        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
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
            child.spawn((
                ImageNode::new(asset_server.load("wordmark.png")),
                Node { 
                    width: Val::Px(90.0), 
                    ..default() 
                },
            ));

            child.spawn(Node {
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0), 
                ..default()
            }).with_children(|child| {
                ButtonComponent::spawn_button(child, &asset_server, &fonts, wallet);
                ButtonComponent::spawn_button(child, &asset_server, &fonts, message);
            });

            spacer(child);
            
            child.spawn(Node {
                width: Val::Percent(100.0),
                ..default()
            }).with_children(|child| {
                ButtonComponent::spawn_button(child, &asset_server, &fonts, profile);
            });
        });
    });
}

fn spacer (parent: &mut ChildBuilder) {
    parent.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    });
}
