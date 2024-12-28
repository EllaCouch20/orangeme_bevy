
use bevy::prelude::*;

use crate::Theme;
use crate::theme::color::ButtonColor;

// use crate::components::profile_photo::profile_photo;

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
    pub label: String,
    pub size: ButtonSize,
    pub style: ButtonStyle,
    pub photo: Option<String>,
    pub icon: Option<ImageNode>,
    pub width: ButtonWidth,
    pub alignment: JustifyContent,
    pub state: InteractiveState,
}

impl CustomButton {
    pub fn create_on<T: Bundle>(
        self,
        parent: &mut ChildBuilder,
        tag: T,
        theme: &Res<Theme>
    ) {
        let status = InteractiveState::Default;

        let colors: ButtonColor = ButtonColor::new(self.style, status);
        let font = theme.fonts.style.label.clone();

        let (button_width, flex_grow) = match self.width {
            ButtonWidth::Expand => (Val::Percent(100.0), 1.0),
            ButtonWidth::Hug => (Val::Auto, 0.0),
        };

        let (height, padding, icon_size, icon_pad, font_size) = match self.size {
            ButtonSize::Large => (48.0, 24.0, 24.0, 12.0, theme.fonts.size.lg),
            ButtonSize::Medium => (32.0, 12.0, 16.0, 4.0, theme.fonts.size.md)
        };

        parent.spawn((
            Button,
            Node {
                flex_grow,
                height: Val::Px(height),
                flex_basis: button_width,
                width: button_width,
                border: UiRect::all(Val::Px(1.0)),
                justify_content: self.alignment,
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
            self.style,
            status,
            tag,
        )).with_children(|button| {

            // === Spawn Icon === //

            if let Some(icon) = self.icon {
                button.spawn((
                    icon,
                    Node {
                        height: Val::Px(icon_size),
                        width: Val::Px(icon_size),
                        margin: UiRect::right(Val::Px(icon_pad)), 
                        ..default()
                    },
                ));
            }

            // === Spawn Photo === //

            // if let Some(photo) = self.photo.clone() {
            //     button.spawn(Node::default()).with_children(|parent| {
            //         profile_photo(parent, theme, asset_server, 32.0, &photo);
            //     });
            // }

            // === Spawn Label === //

            button.spawn((
                Text::new(self.label),
                TextFont {
                    font,
                    font_size,
                    ..default()
                },
                TextColor(colors.label),
            ));     
        });

    }
}

// ==== Handle Button Interactions ==== //

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
                    Interaction::None => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Default);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                    Interaction::Pressed => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Selected);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                }
            }
        }
    }
}