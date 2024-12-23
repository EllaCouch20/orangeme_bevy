use bevy::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy_ui::prelude::*;

use std::fmt::Write;

use crate::theme::fonts::FontResources;
use crate::theme::color::Display;
use crate::utils::{EXPAND, cal_font_size, usd_to_btc, text};
use crate::StateData;

#[derive(Component)]
pub struct AmountDisplayUsd;
#[derive(Component)]
pub struct AmountDisplayZeros;
#[derive(Component)]
pub struct AmountDisplayHelper;

// ===== Amount Input Widget ===== //

pub fn amount_display(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    colors: &Res<Display>,
    error: Option<&str>,
    zeros: &str,
    usd: &str,
){
    let usd_font = fonts.style.label.clone();
    let usd_font_size = cal_font_size(fonts, usd);

    let btc_font = fonts.style.text.clone();
    let btc_font_size = fonts.size.lg;

    let txt = if let Some(error) = error {
        error
    } else {
        &usd_to_btc(25.0)
    };    
    
    parent.spawn(Node {
        width: EXPAND,
        height: EXPAND,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(8.0), 
        ..default()
    }).with_children(|child| {
        child.spawn(Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            ..default()
        })
        .with_children(|child| { 
            child.spawn((
                text(usd, usd_font.clone(), usd_font_size, colors.text_heading),
                AmountDisplayUsd
            ));
            child.spawn((
                text(zeros, usd_font, usd_font_size, colors.text_secondary),
                AmountDisplayZeros
            ));
        }); 
        child.spawn((
            text(txt, btc_font, btc_font_size, colors.text_secondary),
            AmountDisplayHelper
        )); 
    });  
}
