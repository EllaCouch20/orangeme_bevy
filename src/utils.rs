use bevy::prelude::*;
use crate::theme::fonts::FontResources;

pub const EXPAND: Val = Val::Percent(100.0);
pub const MAX: Val = Val::Px(512.0);

pub fn cal_font_size(fonts: &Res<FontResources>, txt: &str) -> f32 {
    // let len = txt.len();

    // if len <= 4 {
    //     fonts.size.title
    // } else if len <= 7 {
    //     fonts.size.h1
    // } else {
    //     fonts.size.h2
    // }

    fonts.size.title
}

pub fn spacer (parent: &mut ChildBuilder) {
    parent.spawn(Node {
        width: EXPAND,
        height: EXPAND,
        ..default()
    });
}

pub fn usd_to_btc(usd: f32) -> String {
   "0.00001234 BTC".to_string()
}