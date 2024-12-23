use bevy::{prelude::*, ui::FocusPolicy};
use crate::StateData;

use super::despawn_screen;
use crate::primitives::button_presets::primary_disabled;
use crate::utils::usd_to_btc;
use crate::primitives::button::CustomButton;
use crate::ButtonStyle;
use crate::OnAddressScreen;
use crate::ButtonColor;

use crate::{
    menu_plugin,
    Nav
};

use crate::theme::{
    color::Display,
    fonts::FontResources,
    icons::Icon,
};

use crate::interface::{
    header::Header,
    bumper::Bumper,
    interfaces::Interface
};

use crate::primitives::{
    profile_photo::profile_photo,
    button::{
        InteractiveState,
        ButtonComponent,
    },
};

use crate::PageState;

use crate::components::{
    text_input::text_input,
    navigator::sidebar_navigator,
    tip_button::tip_buttons,
    amount_display::{amount_display, AmountDisplayUsd, AmountDisplayZeros, AmountDisplayHelper},
};

use crate::components::amount_display::AmountError;

#[derive(Component)]
pub struct OnAmountScreen;

pub fn amount_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fonts: Res<FontResources>,
    state_data: Res<StateData>,
    colors: Res<Display>,
    mut menu_state: ResMut<NextState<PageState>>,
) {
    let bumper = Bumper::new();
    let interface = Interface::new();
    let header = Header::new();

    commands.spawn((
        interface.node,
        OnAmountScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server, 0);

        parent.spawn(interface.page_node).with_children(|parent| {
            header.stack_header(parent, &fonts, &asset_server, &colors, Some(Icon::Left), "Send bitcoin", Nav::Address);

            parent.spawn(interface.content).with_children(|parent| {
                amount_display(parent, &fonts, &asset_server, &colors, None, &state_data.zeros, &format!("${}", &state_data.usd));
            });
            bumper.button_bumper(parent, &fonts, &asset_server, vec![
                (primary_disabled("Continue"), Nav::Speed)
            ]);
        });
    });
}

// ===== Button Color Handler ===== //

pub fn amount_button_status(
    mut usd_query: Query<&Text, With<AmountDisplayUsd>>,
    mut button_query: Query<(
        &mut CustomButton, 
        Option<&ButtonStyle>, 
        &mut BackgroundColor, 
        &mut BorderColor, 
        &mut InteractiveState, 
        &Children,
    ), With<Button>>,
    mut text_query: Query<(&mut TextColor, &mut Text), (Without<AmountDisplayUsd>, Without<AmountDisplayHelper>)>,
    mut helper_query: Query<&mut TextColor, With<AmountDisplayHelper>>,
    amount_screen_query: Query<(), With<OnAmountScreen>>,
    colors: Res<Display>,
) {
    if amount_screen_query.is_empty() {
        return;
    }

    for usd in usd_query.iter() {
        let is_zero = usd.0 == "$0" ||  usd.0 == "$0." || usd.0 == "$0.0" || usd.0 == "$0.00";
        for (mut button, button_style, mut color, mut border_color, mut state, children) in button_query.iter_mut() {
            if children.iter().any(|&child| {
                if let Ok(text) = text_query.get(child) {
                    text.1.0 == "Continue"
                } else {
                    false
                }
            }) {
                for mut text_color in helper_query.iter_mut() {
                    *state = if text_color.0 == colors.status_danger || is_zero {
                        InteractiveState::Disabled
                    } else {
                        InteractiveState::Default
                    };
                }

                if let Some(button_style) = button_style {
                    let button_colors = ButtonColor::new(*button_style, *state);
                    *color = button_colors.background.into();
                    *border_color = button_colors.outline.into();

                    for &child in children.iter() {
                        if let Ok((mut text_color, _)) = text_query.get_mut(child) {
                            *text_color = button_colors.label.into();
                        }
                    }
                }
            }
        }
    }
}


pub fn amount_display_system(
    state_data: Res<StateData>,
    mut query_set: ParamSet<(
        Query<&mut Text, With<AmountDisplayUsd>>,
        Query<&mut Text, With<AmountDisplayZeros>>,
        Query<(&mut Text, &mut TextColor), With<AmountDisplayHelper>>,
    )>,
    mut visibile_set: Query<&mut Visibility, With<AmountError>>,
) {
    let colors = Display::new();
    let mut is_error = false;
    let min = 0.31;
    let max = 25.31;
    
    if state_data.is_changed() {
        for mut usd in query_set.p0().iter_mut() {
            usd.0 = format!("${}", state_data.usd);
        }

        for mut zeros in query_set.p1().iter_mut() {
            zeros.0 = state_data.zeros.clone();
        }

        for (mut text, mut text_color) in query_set.p2().iter_mut() {
            let usd_value: f32 = state_data.usd.parse().unwrap_or(0.0);

            (text.0, is_error) = if usd_value > max {
                for mut visibility in visibile_set.iter_mut() {
                    *visibility = Visibility::Visible;
                }
                (format!("${} maximum.", max), true)
            } else if usd_value < min && usd_value != 0.0 {
                for mut visibility in visibile_set.iter_mut() {
                    *visibility = Visibility::Visible;
                }
                (format!("${} minimum.", min), true)
            } else if usd_value == 0.0 {
                for mut visibility in visibile_set.iter_mut() {
                    *visibility = Visibility::Hidden;
                }
                ("Type dollar amount.".to_string(), false)
            } else {
                for mut visibility in visibile_set.iter_mut() {
                    *visibility = Visibility::Hidden;
                }
                (usd_to_btc(usd_value), false)
            };

            text_color.0 = if !is_error {
                colors.text_secondary
            } else {
                colors.status_danger
            };
        }
    }
}