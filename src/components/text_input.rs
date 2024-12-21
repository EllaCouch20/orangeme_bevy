use bevy::{prelude::*, ui::FocusPolicy};

use crate::{
    theme::{fonts::FontResources, color::{Display, ButtonColor}},
    primitives::button::{CustomButton, InteractiveState, ButtonStyle, SetState},
    utils::EXPAND,
};

use bevy_simple_text_input::{
    TextInput, 
    TextInputValue, 
    TextInputInactive, 
    TextInputPlaceholder, 
    TextInputPlugin, 
    TextInputSystem, 
    TextInputTextColor, 
    TextInputTextFont,
};


// ===== Instanitate Widget ===== //

pub fn text_input(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    placeholder: &str,
) {
    let font = fonts.style.text.clone();
    let font_size = fonts.size.md;
    
    let colors = Display::new();

    parent.spawn((
        Node {
            border: UiRect::all(Val::Px(1.0)),
            height: Val::Px(48.0),
            width: EXPAND,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Start,
            padding: UiRect::all(Val::Px(16.0)), 
            ..default()
        },
        BorderColor(colors.outline_secondary),
        BackgroundColor(colors.bg_primary),
        BorderRadius::all(Val::Px(8.0)),
        FocusPolicy::Block,
        TextInput,
        TextInputTextFont(TextFont {
            font,
            font_size,
            ..default()
        }),
        TextInputTextColor(TextColor(colors.text_primary)),
        TextInputPlaceholder {
            value: placeholder.to_string(),
            ..default()
        },
        TextInputInactive(true),
    ));
}

// ===== Text Input Visual Handler ===== //

pub fn focus(
    mut interaction_query: Query<
        (Entity, &Interaction), 
        (Changed<Interaction>, With<TextInputInactive>),
    >,
    mut param_set: ParamSet<(
        Query<(
            Entity, 
            &mut TextInputInactive, 
            &mut BorderColor,
            &TextInputValue,
        )>,
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
    let colors = Display::new();

    {
        let mut text_input_query = param_set.p0();
        for (interaction_entity, interaction) in interaction_query.iter_mut() {
            if *interaction == Interaction::Pressed {
                for (entity, mut inactive, mut input_border_color, _input_value) in text_input_query.iter_mut() {
                    if entity == interaction_entity {
                        inactive.0 = false;
                        *input_border_color = colors.outline_primary.into();
                    } else {
                        inactive.0 = true;
                        *input_border_color = colors.outline_secondary.into();
                    }
                }
            }
        }
    }

    let is_any_input_empty = {
        let text_input_query = param_set.p0();
        text_input_query.iter().any(|(_, _, _, input_value)| input_value.0.is_empty())
    };

    {
        let mut button_query = param_set.p1();
        for (mut data, set_state, button_style, mut color, mut border_color, children) in button_query.iter_mut() {
            if *set_state == SetState::Disablable {
                data.state = if is_any_input_empty {
                    InteractiveState::Disabled
                } else {
                    InteractiveState::Default
                };

                if let Some(button_style) = button_style {
                    let button_colors: ButtonColor = ButtonColor::new(*button_style, data.state);
                    *color = button_colors.background.into();
                    *border_color = button_colors.outline.into();
                    for child in children.iter() {
                        if let Ok((mut text_color, _parent)) = text_query.get_mut(*child) {
                            *text_color = button_colors.label.into();
                        }
                    }
                }
            }
        }
    }
}
