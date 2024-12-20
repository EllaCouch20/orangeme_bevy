use bevy::prelude::*;

use crate::theme::{
    color::Display,
    fonts::FontResources,
};

use crate::primitives::button::{CustomButton, ButtonComponent};

pub fn tip_buttons(
    parent: &mut ChildBuilder, 
    asset_server: &Res<AssetServer>, 
    fonts: &Res<FontResources>, 
    buttons: Vec<CustomButton>,
) {
    let buttons_len = buttons.len();

    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(8.0),
        ..default()
    }).with_children(|parent| {
        for (i, button) in buttons.into_iter().enumerate() {
            ButtonComponent::spawn_button(parent, &asset_server, &fonts, button);

            if buttons_len == 2 && i == 0 {
                separator_text(parent, &fonts);
            } else if buttons_len >= 3 && i == buttons_len - 2 {
                separator_text(parent, &fonts);
            }
        }
    });
}

pub fn separator_text(parent: &mut ChildBuilder, fonts: &Res<FontResources>){
    let font = fonts.style.text.clone();
    let colors = Display::new();
    parent.spawn((
        Text::new("or"),
        TextFont{font, font_size: fonts.size.sm, ..default()},
        TextColor(colors.text_secondary),
    ));
}