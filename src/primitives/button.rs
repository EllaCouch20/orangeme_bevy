#![allow(unused)]

use bevy::prelude::*;
use crate::theme::{
    color::ButtonColor,
    fonts::FontResources,
    icons::Icon,
};
use crate::primitives::{profile_photo::profile_photo};
use crate::NavigateTo;

use bevy_simple_text_input::TextInputInactive;
use bevy_simple_text_input::TextInputValue;

#[derive(Copy, Clone, Component, Debug)]
pub enum ButtonStyle {
    Primary,
    Secondary,
    Ghost,
}

#[derive(Copy, Clone, Component, PartialEq, Debug)]
pub enum InteractiveState {
    Default,
    Selected,
    Hover,
    Disabled,
}

pub enum ButtonSize {
    Medium,
    Large,
}

#[derive(PartialEq)]
pub enum ButtonWidth {
    Expand,
    Hug,
}

#[derive(Component)]
pub struct ButtonInteraction {
    pub state: InteractiveState,
    pub is_selected: bool,
}

#[derive(Component, PartialEq)]
pub enum SetState{Selectable, Disablable}

#[derive(Component)]
pub struct CustomButton {
    label: String,
    icon: Option<Icon>,
    photo: Option<String>,
    style: ButtonStyle,
    width_style: ButtonWidth,
    size: ButtonSize,
    pub state: InteractiveState,
    action: NavigateTo,
    alignment: JustifyContent,
    selected: bool,
    disabled: bool,
}

impl CustomButton {
    pub fn new(
        label: &str,
        icon: Option<Icon>,
        photo: Option<String>,
        style: ButtonStyle,
        width_style: ButtonWidth,
        size: ButtonSize,
        state: InteractiveState,
        action: NavigateTo,
        alignment: JustifyContent,
        selected: bool,
        disabled: bool,
    ) -> Self {
        Self {
            label: label.to_string(),
            icon,
            photo,
            style,
            width_style,
            size,
            state,
            action,
            alignment,
            selected,
            disabled,
        }
    }
}

pub struct ButtonComponent;

impl ButtonComponent {
    pub fn spawn_button(
        parent: &mut ChildBuilder,
        asset_server: &Res<AssetServer>,
        fonts: &Res<FontResources>,
        data: CustomButton,
    ) {

        let colors: ButtonColor = ButtonColor::new(data.style, data.state);
        let font = fonts.style.label.clone();

        // ===== Mapping Info From Size ===== //

        let (button_width, flex_grow) = match data.width_style {
            ButtonWidth::Expand => (Val::Percent(100.0), 1.0),
            ButtonWidth::Hug => (Val::Auto, 0.0),
        };

        let (height, padding, icon_size, icon_pad, font_size) = match data.size {
            ButtonSize::Large => (48.0, 24.0, 24.0, 12.0, fonts.size.lg),
            ButtonSize::Medium => (32.0, 12.0, 16.0, 4.0, fonts.size.md)
        };

        // ===== Button Background ===== //

        let mut button = parent.spawn((
            Button,
            Node {
                flex_grow,
                height: Val::Px(height),
                flex_basis: button_width,
                width: button_width,
                border: UiRect::all(Val::Px(1.0)),
                justify_content: data.alignment,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(icon_pad),
                padding: UiRect {
                    left: Val::Px(padding),
                    right: Val::Px(padding),
                    ..default()
                },
                ..default()
            },
            BorderColor(colors.outline),
            BorderRadius::MAX,
            BackgroundColor(colors.background),
            data.action,
        ));

        // ===== Button Innerds ===== //
        
        button.with_children(|button| {

            // ===== Optional Icon ===== //
            if let Some(icon) = data.icon {
                button.spawn((
                    Icon::new(data.icon.unwrap(), asset_server),
                    Node {
                        height: Val::Px(icon_size),
                        width: Val::Px(icon_size),
                        ..default()
                    },
                ));
            }

            // ===== Option Photo ===== //
            if let Some(photo) = data.photo.clone() {
                button.spawn(Node::default()).with_children(|parent| {
                    profile_photo(parent, fonts, asset_server, 32.0, &photo);
                });
            }

            // ===== Label ===== //
            button.spawn((
                Text::new(data.label.clone()),
                TextFont {font, font_size, ..default()},
                TextColor(colors.label),
            ));     
        });

        if data.selected { button.insert(SetState::Selectable); }
        if data.disabled { button.insert(SetState::Disablable); }
        button.insert(data.style);
        button.insert(data);
    }
}

// ===== Button Color Handler ===== //

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
                    _ => {}
                }
            }
        }
    }
}

pub fn button_status_system(
    mut param_set: ParamSet<(
        Query<(
            Entity, 
            &mut TextInputInactive, 
            &mut BorderColor,
            &TextInputValue,
        ), Changed<TextInputValue>>,
        Query<(
            &mut CustomButton, 
            &SetState,
            Option<&ButtonStyle>,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ), With<Button>>,
    )>,
    mut text_query: Query<(&mut TextColor, &Parent), With<Text>>,
) {
    // // Step 1: Check if any input field has an empty value
    // let is_any_input_empty = {
    //     let text_input_query = param_set.p0();
    //     text_input_query.iter().any(|(_, _, _, input_value)| input_value.0.is_empty())
    // };

    // // Step 2: Update button state and colors only if TextInputValue has changed
    // let mut button_query = param_set.p1();
    // for (mut data, set_state, button_style, mut color, mut border_color, children) in button_query.iter_mut() {
    //     if *set_state == SetState::Disablable {
    //         let new_state = if is_any_input_empty {
    //             InteractiveState::Disabled
    //         } else {
    //             InteractiveState::Default
    //         };

    //         if data.state != new_state {
    //             data.state = new_state;

    //             if let Some(button_style) = button_style {
    //                 let button_colors: ButtonColor = ButtonColor::new(*button_style, data.state);
    //                 *color = button_colors.background.into();
    //                 *border_color = button_colors.outline.into();
    //                 for child in children.iter() {
    //                     if let Ok((mut text_color, _parent)) = text_query.get_mut(*child) {
    //                         *text_color = button_colors.label.into();
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
}
