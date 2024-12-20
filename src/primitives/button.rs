#![allow(unused)]

use bevy::prelude::*;

use crate::theme::{
    color::ButtonColor,
    fonts::FontResources,
    icons::Icon,
};

use crate::primitives::{
    profile_photo::profile_photo
};

use crate::NavigateTo;

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
        println!("Button state: {:?}", data.state);

        let colors: ButtonColor = ButtonColor::new(data.style, data.state);
        let font = fonts.style.label.clone();

        let (button_width, flex_grow) = match data.width_style {
            ButtonWidth::Expand => (Val::Percent(100.0), 1.0),
            ButtonWidth::Hug => (Val::Auto, 0.0),
        };

        let (height, padding, icon_size, icon_pad, font_size) = match data.size {
            ButtonSize::Large => (48.0, 24.0, 24.0, 12.0, fonts.size.lg),
            ButtonSize::Medium => (32.0, 12.0, 16.0, 4.0, fonts.size.md)
        };

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
        
        button.with_children(|button| {
            if let Some(icon) = data.icon {
                button.spawn((
                    Icon::new(data.icon.unwrap(), asset_server),
                    Node {
                        height: Val::Px(icon_size),
                        width: Val::Px(icon_size),
                        margin: UiRect::right(Val::Px(icon_pad)), 
                        ..default()
                    },
                ));
            }
            if let Some(photo) = data.photo.clone() {
                button.spawn(Node {
                    margin: UiRect::right(Val::Px(icon_pad)), 
                    ..default()
                }).with_children(|parent| {
                    profile_photo(parent, &fonts, &asset_server, &photo);
                });
            }

            button.spawn((
                Text::new(data.label.clone()),
                TextFont {
                    font,
                    font_size,
                    ..default()
                },
                TextColor(colors.label),
            ));     
        });

        if data.selected { button.insert(SetState::Selectable); }
        if data.disabled { button.insert(SetState::Disablable); }
        button.insert(data.style);
        button.insert(data);
    }
}


#[derive(Component)]
pub struct ButtonInteraction {
    pub state: InteractiveState,
    pub is_selected: bool,
}

#[derive(Component, PartialEq)]
pub enum SetState{Selectable, Disablable}

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
                        border_color.0 = colors.outline.into();
                    }
                    Interaction::Pressed => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Selected);
                        *color = colors.background.into();
                        border_color.0 = colors.outline.into();
                    }
                    _ => {}
                }
            }
        }
    }
}