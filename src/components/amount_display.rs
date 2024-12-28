use bevy::prelude::*;
use crate::utils::{EXPAND, cal_font_size, usd_to_btc, text};
use crate::Theme;

#[derive(Component)]
pub struct AmountDisplayUsd;
#[derive(Component)]
pub struct AmountDisplayZeros;
#[derive(Component)]
pub struct AmountDisplayHelper;
#[derive(Component)]
pub struct AmountError;

// ===== Amount Input Widget ===== //

pub fn amount_display(
    parent: &mut ChildBuilder,
    theme: &Res<Theme>,
    error: Option<&str>,
    zeros: &str,
    usd: &str,
){
    let usd_font = theme.fonts.style.label.clone();
    let usd_font_size = cal_font_size(theme, usd);

    let btc_font = theme.fonts.style.text.clone();
    let btc_font_size = theme.fonts.size.lg;

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
                text(usd, usd_font.clone(), usd_font_size, theme.colors.text_heading),
                AmountDisplayUsd
            ));
            child.spawn((
                text(zeros, usd_font, usd_font_size, theme.colors.text_secondary),
                AmountDisplayZeros
            ));
        }); 
        child.spawn((
            Node {
                width: EXPAND,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(6.0), 
                ..default()
            },
        )).with_children(|parent| {
            parent.spawn((
                Visibility::Hidden,
                AmountError,
                Node {
                    width: Val::Px(24.0),
                    height: Val::Px(24.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                theme.icons.error(),
            )); 
            parent.spawn((
                text(txt, btc_font, btc_font_size, theme.colors.text_secondary),
                AmountDisplayHelper
            )); 
        }); 
    });  
}
