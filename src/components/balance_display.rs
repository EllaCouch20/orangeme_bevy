use bevy::prelude::*;
use crate::theme::fonts::FontResources;
use crate::theme::color::Display;
use bevy_ui::prelude::*;


pub fn balance_display(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
){
    let usd_font = fonts.style.label.clone();
    let usd_font_size = fonts.size.title;

    let btc_font = fonts.style.text.clone();
    let btc_font_size = fonts.size.lg;

    let colors = Display::new();

    //font_size: txtL <= 4 ? 'title' : txtL <= 7 ? 'h1' : 'h2',
    
    parent.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(32.0), 
        ..default()
    })
    .with_children(|child| {
        child.spawn((
            Text::new("$0.00"),
            TextFont {
                font: usd_font,
                font_size: usd_font_size,
                ..default()
            },
            TextColor(colors.text_heading),
        ));  
        child.spawn((
            Text::new("0.00000000 BTC"),
            TextFont {
                font: btc_font,
                font_size: btc_font_size,
                ..default()
            },
            TextColor(colors.text_secondary),
        ));  
    });  
}