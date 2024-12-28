use bevy::prelude::*;
use crate::Theme;
use crate::utils::text;
use crate::components::button_presets::secondary_default;

// ===== List of Helper Buttons ===== //

#[derive(Component, Debug)]
pub enum Tip {
    PasteClipboard, 
    ScanQRCode, 
    SelectContact
}

pub fn tip_buttons(
    parent: &mut ChildBuilder, 
    theme: &Res<Theme>,
    buttons: Vec<(&str, ImageNode, Tip)>,
) {
    let buttons_len = buttons.len();
    let font = theme.fonts.style.heading.clone();

    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(8.0),
        ..default()
    }).with_children(|parent| {
        for (i, (name, icon, tip_variant)) in buttons.into_iter().enumerate() {

            secondary_default(name, icon).create_on(parent, tip_variant, &theme);

            if buttons_len == 2 && i == 0 {
                parent.spawn(text("or", font.clone(), theme.fonts.size.sm, theme.colors.text_secondary));
            } else if buttons_len >= 3 && i == buttons_len - 2 {
                parent.spawn(text("or", font.clone(), theme.fonts.size.sm, theme.colors.text_secondary));
            }
        }
    });
}
