use bevy::prelude::*;
use crate::theme::fonts::FontResources;
use crate::theme::color::Display;
use bevy_ui::prelude::*;
use crate::utils::{EXPAND, text};


// ===== Home Balance Widget ===== //

pub fn balance_display(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    usd: &str,
    btc: &str,
){
    let font = fonts.style.label.clone();
    let font_size = fonts.size.title;

    let btc_font = fonts.style.text.clone();
    let btc_font_size = fonts.size.lg;

    let colors = Display::new();
    
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
            text(usd, font, font_size, colors.text_heading),
        );
        child.spawn(
            text(btc, btc_font, btc_font_size, colors.text_secondary),
        ); 
    });  
}