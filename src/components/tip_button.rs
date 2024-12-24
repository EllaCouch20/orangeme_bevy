use bevy::prelude::*;
use crate::utils::text;
use crate::theme::{color::Display, fonts::FontResources};
use crate::primitives::button::{CustomButton, ButtonComponent};

// ===== List of Helper Buttons ===== //

#[derive(Component, Debug)]
pub enum Tip {
    PasteClipboard, 
    ScanQRCode, 
    SelectContact
}

pub fn tip_buttons(
    parent: &mut ChildBuilder, 
    asset_server: &Res<AssetServer>, 
    colors: &Res<Display>,
    fonts: &Res<FontResources>, 
    buttons: Vec<(CustomButton, Tip)>,
) {
    let buttons_len = buttons.len();
    let font = fonts.style.heading.clone();

    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(8.0),
        ..default()
    }).with_children(|parent| {
        for (i, (button, tip_variant)) in buttons.into_iter().enumerate() {

            parent.spawn((Node::default(), tip_variant)).with_children(|parent|{
                ButtonComponent::spawn_button(parent, asset_server, fonts, button);
            });

            if buttons_len == 2 && i == 0 {
                parent.spawn(text("or", font.clone(), fonts.size.sm, colors.text_secondary));
            } else if buttons_len >= 3 && i == buttons_len - 2 {
                parent.spawn(text("or", font.clone(), fonts.size.sm, colors.text_secondary));
            }
        }
    });
}
