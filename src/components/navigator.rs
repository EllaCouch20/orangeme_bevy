use bevy::prelude::*;

use crate::components::button_presets::{nav_button, nav_button_pfp};

use crate::utils::{EXPAND, spacer};

use crate::ButtonStyle;
use crate::theme::color::ButtonColor;
use crate::InteractiveState;
use crate::Page;
use crate::Theme;

// ===== Desktop Sidebar Navigation ===== //


#[derive(Component, Debug)]
pub enum SidebarButton {
    Bitcoin,
    Messages,
    Profile,
}

pub fn sidebar_navigator (
    parent: &mut ChildBuilder,
    theme: &Res<Theme>,
    preset: u8,
) {

    parent.spawn(Node {
        width: Val::Px(300.0),
        height: EXPAND,
        align_items: AlignItems::Start,
        justify_content: JustifyContent::Start,
        ..default()
    }).with_children(|parent| {
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
            BorderColor(theme.colors.outline_secondary),
        )).with_children(|child| {

            // ===== orange Logo ===== //

            child.spawn((
                theme.icons.wordmark(),
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
            }).with_children(|parent| {
                nav_button("Bitcoin", theme.icons.wallet(), preset == 0).create_on(parent, SidebarButton::Bitcoin, &theme);
                nav_button("Messages", theme.icons.message(), preset == 1).create_on(parent, SidebarButton::Messages, &theme);
            });

            spacer(child);

            // ===== Profile Button ===== //
            
            nav_button_pfp("Ella Couch", preset == 2).create_on(child, SidebarButton::Profile, &theme);
        });
    });
}

pub fn sidebar_navigation_system(
    mut interaction_query: Query<(
        &Parent,
        &mut BackgroundColor,
        &mut BorderColor,
        Option<&ButtonStyle>,
        &mut InteractiveState,
    ), With<Button>>,
    sidebar_query: Query<&SidebarButton>,
    mut menu_state: ResMut<NextState<Page>>,
) {
    let mut selected_button = None;

    if let Some(selected_parent) = selected_button {
        for (parent, mut color, mut border_color, button_style, mut state) in &mut interaction_query {
            if parent.get() != selected_parent && sidebar_query.get(parent.get()).is_ok() {
                if let Some(button_style) = button_style {
                    let colors = ButtonColor::new(*button_style, InteractiveState::Default);
                    *color = colors.background.into();
                    border_color.0 = colors.outline;
                }

                *state = InteractiveState::Default;
            }
        }
    }
}
