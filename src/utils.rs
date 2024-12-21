use bevy::prelude::*;
use crate::theme::fonts::FontResources;

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
