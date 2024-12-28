use bevy::{prelude::*, ui::FocusPolicy};

use crate::{
    Theme,
    utils::EXPAND,
};

use bevy_simple_text_input::{
    TextInput, 
    TextInputValue, 
    TextInputInactive, 
    TextInputPlaceholder,
    TextInputTextColor, 
    TextInputTextFont,
};

pub fn text_input(
    parent: &mut ChildBuilder,
    theme: &Res<Theme>,
    placeholder: &str,
) {
    let font = theme.fonts.style.text.clone();
    let font_size = theme.fonts.size.md;
    
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
        BorderColor(theme.colors.outline_secondary),
        BackgroundColor(theme.colors.bg_primary),
        BorderRadius::all(Val::Px(8.0)),
        FocusPolicy::Block,
        TextInput,
        TextInputTextFont(TextFont {
            font,
            font_size,
            ..default()
        }),
        TextInputTextColor(TextColor(theme.colors.text_primary)),
        TextInputPlaceholder {
            value: placeholder.to_string(),
            ..default()
        },
        TextInputInactive(true),
    ));
}

// ===== Text Input Visual Handler ===== //

pub fn text_input_visuals_system(
    mut interaction_query: Query<(Entity, &Interaction), (Changed<Interaction>, With<TextInputInactive>)>,
    mut text_input_query: Query<(
        Entity, 
        &mut TextInputInactive, 
        &mut BorderColor,
        &TextInputValue,
    )>,
    theme: Res<Theme>,
) {
    for (interaction_entity, interaction) in interaction_query.iter_mut() {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut input_border_color, _input_value) in text_input_query.iter_mut() {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *input_border_color = theme.colors.outline_primary.into();
                } else {
                    inactive.0 = true;
                    *input_border_color = theme.colors.outline_secondary.into();
                }
            }
        }
    }
}
