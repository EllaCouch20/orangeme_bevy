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

use crate::ButtonColor;
use crate::ButtonStyle;
use crate::PageState;

// ===== Desktop Sidebar Navigation ===== //

#[derive(Component, Debug)]
pub struct BitcoinNavigationButton;
#[derive(Component, Debug)]
pub struct MessagesNavigationButton;
#[derive(Component, Debug)]
pub struct ProfileNavigationButton;

pub fn sidebar_navigator (
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>, 
    mut menu_state: ResMut<NextState<PageState>>,
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
                child.spawn((Node {
                    width: EXPAND,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                }, BitcoinNavigationButton)).with_children(|child| {
                    ButtonComponent::spawn_button(child, asset_server, fonts, wallet);
                });
                child.spawn((Node {
                    width: EXPAND,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                }, MessagesNavigationButton)).with_children(|child| {
                    ButtonComponent::spawn_button(child, asset_server, fonts, message);
                });
            });

            spacer(child);

            // ===== Profile Button ===== //
            
            child.spawn((Node {
                width: EXPAND,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            }, ProfileNavigationButton)).with_children(|child| {
                ButtonComponent::spawn_button(child, asset_server, fonts, profile);
            });
        });
    });
}


pub fn navigation_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &Parent,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&ButtonStyle>,
            &mut InteractiveState,
        ),
        (With<Button>),
    >,
    nav_bit_query: Query<&BitcoinNavigationButton>,
    nav_mes_query: Query<&MessagesNavigationButton>,
    nav_pro_query: Query<&ProfileNavigationButton>,
    mut menu_state: ResMut<NextState<PageState>>,
) {
    let mut selected_button = None;

    for (interaction, parent, mut color, mut border_color, button_style, mut state) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            selected_button = Some(parent.get());

            if let Ok(nav) = nav_bit_query.get(parent.get()) {
                menu_state.set(PageState::Home);
            }
            if let Ok(nav) = nav_mes_query.get(parent.get()) {
                menu_state.set(PageState::Home);
            }
            if let Ok(nav) = nav_pro_query.get(parent.get()) {
                menu_state.set(PageState::Home);
            }

            if let Some(button_style) = button_style {
                let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Selected);
                *color = colors.background.into();
                border_color.0 = colors.outline;
            }

            *state = InteractiveState::Selected;
        }
    }

    if let Some(selected_parent) = selected_button {
        for (interaction, parent, mut color, mut border_color, button_style, mut state) in &mut interaction_query {
            if parent.get() != selected_parent {
                if let Some(button_style) = button_style {
                    let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Default);
                    *color = colors.background.into();
                    border_color.0 = colors.outline;
                }

                *state = InteractiveState::Default;
            }
        }
    }
}


pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&ButtonStyle>,
            &InteractiveState,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, button_style, state) in &mut interaction_query {
        if *state != InteractiveState::Disabled && *state != InteractiveState::Selected {
            if let Some(button_style) = button_style {
                match *interaction {
                    Interaction::Hovered => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Hover);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                    Interaction::Pressed => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Selected);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                    Interaction::None => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Default);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                }
            }
        }
    }
}