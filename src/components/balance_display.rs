use bevy::prelude::*;
use crate::Theme;
use crate::utils::{EXPAND, text};


// ===== Home Balance Widget ===== //

pub fn balance_display(
    parent: &mut ChildBuilder,
    theme: &Res<Theme>,
    usd: &str,
    btc: &str,
){
    let font = theme.fonts.style.label.clone();
    let font_size = theme.fonts.size.title;

    let btc_font = theme.fonts.style.text.clone();
    let btc_font_size = theme.fonts.size.lg;
    
    parent.spawn(Node {
        width: EXPAND,
        height: EXPAND,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(8.0), 
        ..default()
    })
    .with_children(|child| {
        child.spawn(
            text(usd, font, font_size, theme.colors.text_heading),
        );
        child.spawn(
            text(btc, btc_font, btc_font_size, theme.colors.text_secondary),
        ); 
    });  
}