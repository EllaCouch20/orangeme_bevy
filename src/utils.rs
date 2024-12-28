use bevy::prelude::*;
use crate::Theme;

pub const EXPAND: Val = Val::Percent(100.0);
pub const MAX: Val = Val::Px(512.0);

pub fn cal_font_size(theme: &Res<Theme>, _txt: &str) -> f32 {
    // let len = txt.len();

    // if len <= 4 {
    //     fonts.size.title
    // } else if len <= 7 {
    //     fonts.size.h1
    // } else {
    //     fonts.size.h2
    // }

    theme.fonts.size.title
}

pub fn spacer (parent: &mut ChildBuilder) {
    parent.spawn(Node {
        width: EXPAND,
        height: EXPAND,
        ..default()
    });
}

pub fn usd_to_btc(_usd: f32) -> String {
   "0.00001234 BTC".to_string()
}

pub fn text(
    text: &str,
    font: Handle<Font>,
    font_size: f32,
    color: Color,
) -> (Text, TextFont, TextColor) {
    (
        Text::new(text),
        TextFont {
            font,
            font_size,
            ..Default::default()
        },
        TextColor(color),
    )
}
