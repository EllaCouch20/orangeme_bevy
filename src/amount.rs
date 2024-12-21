use bevy::{prelude::*, ui::FocusPolicy};
use crate::StateData;

use super::despawn_screen;
use crate::primitives::button_presets::primary_default;

use crate::{
    menu_plugin,
    NavigateTo
};

use crate::theme::{
    color::Display,
    fonts::FontResources,
    icons::Icon,
};

use crate::interface::{
    header::{ header, Header },
    bumper::Bumper,
    interfaces::Interface
};

use crate::primitives::{
    profile_photo::profile_photo,
    button::{
        InteractiveState,
        ButtonComponent,
        button_system,
    },
};

use crate::components::{
    text_input::text_input,
    navigator::sidebar_navigator,
    tip_button::tip_buttons,
    amount_display::{amount_display, AmountDisplay},
};

#[derive(Component)]
pub struct OnAmountScreen;

use bevy::input::keyboard::{Key, KeyboardInput};
use std::fmt::Write; 

pub fn amount_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fonts: Res<FontResources>,
    state_data: Res<StateData>,
) {
    let colors = Display::new();
    let bumper = Bumper::new();
    let interface = Interface::new();

    let next = primary_default("Continue", false, InteractiveState::Default, NavigateTo::Home);

    commands.spawn((
        interface.node,
        OnAmountScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server);

        parent.spawn((interface.page_node)).with_children(|parent| {
            header(parent, &fonts, &asset_server, Header::Stack, "Send bitcoin");
            parent.spawn(interface.content).with_children(|parent| {
                println!("Updating Balance Display");
                amount_display(parent, &fonts, &format!("${}", &state_data.usd), "0.00000000 BTC");
            });
            bumper.button_bumper(parent, &fonts, &asset_server, vec![next]);
        });
    });
    
}

pub fn amount_display_system(
    state_data: Res<StateData>,
    mut query: Query<&mut Text, With<AmountDisplay>>,
) {
    if state_data.is_changed() {
        println!("{:?}", state_data.usd);
        for mut text in query.iter_mut() {
            text.0 = format!("${}", state_data.usd);
        }
    }
}

use bevy::input::ButtonState;

pub fn keyboard_input_system(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut state_data: ResMut<StateData>,
) {
    for event in keyboard_input_events.read() {
        if event.state == ButtonState::Pressed {
            println!("{:?}", event.key_code);
            match event.key_code {
                KeyCode::Digit0 |
                KeyCode::Digit1 |
                KeyCode::Digit2 |
                KeyCode::Digit3 |
                KeyCode::Digit4 |
                KeyCode::Digit5 |
                KeyCode::Digit6 |
                KeyCode::Digit7 |
                KeyCode::Digit8 |
                KeyCode::Digit9 |
                KeyCode::Period |
                KeyCode::Backspace => {
                    let (updated_amount, valid_input, _needed_placeholders) =
                        update_amount(state_data.usd.clone(), event.key_code);
                    if valid_input {
                        state_data.usd = updated_amount;
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn update_amount(amount: String, key: KeyCode) -> (String, bool, u8) {
    let is_zero = || amount == "0";
    let zero = "0".to_string();

    let (updated_amount, valid_input) = match key {
        KeyCode::Backspace => {
            if is_zero() {
                (zero, false)
            } else if amount.len() == 1 {
                (zero, true)
            } else {
                (amount[..amount.len() - 1].to_string(), true)
            }
        },
        KeyCode::Period => {
            if !amount.contains('.') && amount.len() <= 7 {
                (format!("{}{}", amount, "."), true)
            } else {
                (amount.clone(), false)
            }
        },
        _ => {
            let input = match key {
                KeyCode::Digit0 => '0',
                KeyCode::Digit1 => '1',
                KeyCode::Digit2 => '2',
                KeyCode::Digit3 => '3',
                KeyCode::Digit4 => '4',
                KeyCode::Digit5 => '5',
                KeyCode::Digit6 => '6',
                KeyCode::Digit7 => '7',
                KeyCode::Digit8 => '8',
                KeyCode::Digit9 => '9',
                _ => unreachable!(),
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

    let needed_placeholders = if updated_amount.contains('.') {
        let split: Vec<&str> = updated_amount.split('.').collect();
        2 - split.get(1).unwrap_or(&"").len() as u8
    } else { 0 };

    (updated_amount, valid_input, needed_placeholders)
}
