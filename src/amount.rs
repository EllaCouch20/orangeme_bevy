use bevy::{prelude::*, ui::FocusPolicy};
use crate::StateData;

use super::despawn_screen;
use crate::primitives::button_presets::primary_default;
use crate::utils::usd_to_btc;

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
    amount_display::{amount_display, AmountDisplayUsd, AmountDisplayZeros, AmountDisplayHelper},
};

#[derive(Component)]
pub struct OnAmountScreen;

pub fn amount_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    fonts: Res<FontResources>,
    state_data: Res<StateData>,
    colors: Res<Display>,
) {
    let bumper = Bumper::new();
    let interface = Interface::new();

    let next = primary_default("Continue", false, InteractiveState::Default, NavigateTo::Speed);

    commands.spawn((
        interface.node,
        OnAmountScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server);

        parent.spawn(interface.page_node).with_children(|parent| {
            header(parent, &fonts, &asset_server, &colors, Header::Stack, "Send bitcoin");
            parent.spawn(interface.content).with_children(|parent| {
                amount_display(parent, &fonts, &colors, None, &state_data.zeros, &state_data.helper);
            });
            bumper.button_bumper(parent, &fonts, &asset_server, vec![next]);
        });
    });
    
}

pub fn amount_display_system(
    state_data: Res<StateData>,
    mut query_set: ParamSet<(
        Query<&mut Text, With<AmountDisplayUsd>>,
        Query<&mut Text, With<AmountDisplayZeros>>,
        Query<(&mut Text, &mut TextColor), With<AmountDisplayHelper>>,
    )>,
) {
    let colors = Display::new();
    
    if state_data.is_changed() {
        for mut usd in query_set.p0().iter_mut() {
            usd.0 = format!("${}", state_data.usd);
        }

        for mut zeros in query_set.p1().iter_mut() {
            zeros.0 = state_data.zeros.clone();
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