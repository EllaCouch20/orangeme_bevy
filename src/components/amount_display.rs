use bevy::prelude::*;
use crate::theme::fonts::FontResources;
use crate::theme::color::Display;
use bevy_ui::prelude::*;

use bevy::input::keyboard::{Key, KeyboardInput};
use std::fmt::Write; 

use crate::StateData;

use crate::utils::cal_font_size;

#[derive(Component)]
pub struct AmountDisplayUsd;
#[derive(Component)]
pub struct AmountDisplayZeros;
#[derive(Component)]
pub struct AmountDisplayHelper;


pub fn amount_display(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    usd: &str,
    zeros: &str,
    helper: &str,
){
    let usd_font = fonts.style.label.clone();
    let usd_font_size = cal_font_size(&fonts, usd);

    let btc_font = fonts.style.text.clone();
    let btc_font_size = fonts.size.lg;

    let colors = Display::new();
    
    parent.spawn(Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(8.0), 
        ..default()
    })
    .with_children(|child| {
        child.spawn(Node {
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Row,
            ..default()
        })
        .with_children(|child| {
            child.spawn((
                Text::new(usd),
                TextFont {
                    font: usd_font.clone(),
                    font_size: usd_font_size,
                    ..default()
                },
                TextColor(colors.text_heading),
                AmountDisplayUsd
            ));  
            child.spawn((
                Text::new(zeros),
                TextFont {
                    font: usd_font,
                    font_size: usd_font_size,
                    ..default()
                },
                TextColor(colors.text_secondary),
                AmountDisplayZeros
            )); 
        }); 

        child.spawn((
            Text::new(helper),
            TextFont {
                font: btc_font,
                font_size: btc_font_size,
                ..default()
            },
            TextColor(colors.text_secondary),
            AmountDisplayHelper
        ));  
    });  
}

pub fn amount_display_system(
    state_data: Res<StateData>,
    fonts: Res<FontResources>,
    mut query_set: ParamSet<(
        Query<(&mut Text, &mut TextFont), With<AmountDisplayUsd>>,
        Query<(&mut Text, &mut TextFont), With<AmountDisplayZeros>>,
        Query<(&mut Text, &mut TextColor), With<AmountDisplayHelper>>,
    )>,
) {
    let colors = Display::new();
    
    if state_data.is_changed() {
        for (mut usd, mut text_font) in query_set.p0().iter_mut() {
            usd.0 = format!("${}", state_data.usd);
            text_font.font_size = cal_font_size(&fonts, &state_data.usd);
        }

        for (mut zeros, mut text_font) in query_set.p1().iter_mut() {
            zeros.0 = state_data.zeros.clone();
            text_font.font_size = cal_font_size(&fonts, &state_data.usd);
        }

        for (mut text, mut text_color) in query_set.p2().iter_mut() {
            let usd_value: f32 = state_data.usd.parse().unwrap_or(0.0);

            text.0 = if usd_value < state_data.balance_usd {
                usd_to_btc(usd_value)
            } else {
                "Amount exceeds balance".to_string()
            };

            text_color.0 = if usd_value < state_data.balance_usd {
                colors.text_secondary
            } else {
                colors.status_danger
            };
        }
    }
}

fn usd_to_btc(usd: f32) -> String {
    return "0.00001234 BTC".to_string()
}

use bevy::input::ButtonState;

pub fn keyboard_input_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut state_data: ResMut<StateData>,
) {
    for event in keyboard_input_events.read() {
        if event.state == ButtonState::Pressed {
            if event.key_code == KeyCode::Backspace {
                println!("Backspace: {:?}", event.logical_key);
                println!("Backspace: {:?}", event.key_code);
                let (updated_amount, valid_input, needed_placeholders) =
                    backspace_amount(state_data.usd.clone(), event.key_code.clone());

                if valid_input {
                    state_data.usd = updated_amount;
                    state_data.zeros = needed_placeholders;
                }
            } else {

                println!("{:?}", event.logical_key);
                println!("{:?}", event.key_code);

                let (updated_amount, valid_input, needed_placeholders) =
                    update_amount(state_data.usd.clone(), event.logical_key.clone());

                if valid_input {
                    state_data.usd = updated_amount;
                    state_data.zeros = needed_placeholders;
                }
            }
        }
    }
}

pub fn backspace_amount(amount: String, key: KeyCode) -> (String, bool, String) {
    let is_zero = || amount == "0";
    let zero = "0".to_string();

    let (updated_amount, valid_input) = if is_zero() {
        (zero, false)
    } else if amount.len() == 1 {
        (zero, true)
    } else {
        (amount[..amount.len() - 1].to_string(), true)
    };
    
    let needed_placeholders = update_placeholders(updated_amount.clone());

    (updated_amount, valid_input, needed_placeholders)
}

pub fn update_placeholders(updated_amount: String) -> String {
    if updated_amount.contains('.') {
        let split: Vec<&str> = updated_amount.split('.').collect();
        let fractional_length = split.get(1).unwrap_or(&"").len();
        match 2 - fractional_length {
            1 => "0",
            2 => "00",
            _ => "",
        }
    } else {
        ""
    }.to_string()
}

pub fn update_amount(amount: String, key: Key) -> (String, bool, String) {
    let is_zero = || amount == "0";
    let zero = "0".to_string();

    let (updated_amount, valid_input) = match key {
        Key::Character(c) if c == "." => {
            if !amount.contains('.') && amount.len() <= 7 {
                (format!("{}{}", amount, "."), true)
            } else {
                (amount.clone(), false)
            }
        },
        _ => {
            let input = match key {
                Key::Character(c) if c == "0" => '0',
                Key::Character(c) if c == "1" => '1',
                Key::Character(c) if c == "2" => '2',
                Key::Character(c) if c == "3" => '3',
                Key::Character(c) if c == "4" => '4',
                Key::Character(c) if c == "5" => '5',
                Key::Character(c) if c == "6" => '6',
                Key::Character(c) if c == "7" => '7',
                Key::Character(c) if c == "8" => '8',
                Key::Character(c) if c == "9" => '9',
                _ => return (
                    amount.clone(), 
                    false, 
                    update_placeholders(amount.clone())
                )
            };
            if is_zero() {
                (input.to_string(), true)
            } else if amount.contains('.') {
                let split: Vec<&str> = amount.split('.').collect();
                if amount.len() < 11 && split[1].len() < 2 {
                    (format!("{}{}", amount, input), true)
                } else {
                    (amount.clone(), false)
                }
            } else if amount.len() < 10 {
                (format!("{}{}", amount, input), true)
            } else {
                (amount.clone(), false)
            }
        }
    };

    let needed_placeholders = update_placeholders(updated_amount.clone());

    (updated_amount, valid_input, needed_placeholders)
}
