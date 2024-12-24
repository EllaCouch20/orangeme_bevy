use bevy::prelude::*;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy_ui::prelude::*;

use std::fmt::Write;

use crate::theme::fonts::FontResources;
use crate::theme::color::Display;
use crate::utils::{EXPAND, cal_font_size, usd_to_btc};
use crate::StateData;
use crate::OnAmountScreen;

use crate::components::amount_display::{AmountDisplayUsd, AmountDisplayZeros, AmountDisplayHelper};

// ===== Keyboard Input System ===== //

pub fn keyboard_input_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut state_data: ResMut<StateData>,
    amount_screen_query: Query<(), With<OnAmountScreen>>,
) {
    if amount_screen_query.is_empty() {
        return;
    }

    for event in keyboard_input_events.read() {
        if event.state == ButtonState::Pressed {
            let (updated_amount, valid_input, needed_placeholders) = 
                if event.key_code == KeyCode::Backspace {
                    backspace_amount(state_data.usd.clone(), event.key_code)
                } else {
                    update_amount(state_data.usd.clone(), event.logical_key.clone())
                };

            if valid_input {
                state_data.usd = updated_amount;
                state_data.zeros = needed_placeholders;
            }
        }
    }
}

// ===== Update On Backspace ===== //

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

// ===== Update On Digit Input ===== //

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

// ===== Calculate Needed Placeholders ===== //

pub fn update_placeholders(updated_amount: String) -> String {
    if updated_amount.contains('.') {

        let sep: Vec<_> = updated_amount
            .split('.')
            .collect();

        let fractional_length = sep.get(1)
            .unwrap_or(&"")
            .len();

        match 2 - fractional_length {
            1 => "0",
            2 => "00",
            _ => "",
        }

    } else {""}.to_string()
}