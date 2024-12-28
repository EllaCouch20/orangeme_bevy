use bevy::prelude::*;
use crate::StateData;

use crate::Page;
use crate::Theme;

use crate::components::amount_display::AmountDisplayHelper;
use crate::utils::usd_to_btc;

use crate::components::{
    interfaces::Header,
    interfaces::Bumper,
    interfaces::Interface,
    navigator::sidebar_navigator,
    amount_display::amount_display
};

#[derive(Component)]
pub struct OnAmountScreen;

pub fn amount_setup(
    mut commands: Commands,
    theme: Res<Theme>,
    state_data: Res<StateData>,
) {
    let bumper = Bumper::new();
    let interface = Interface::new();
    let header = Header::new();

    commands.spawn((
        interface.node,
        OnAmountScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &theme, 0);

        parent.spawn(interface.page_node).with_children(|parent| {
            header.stack_header(parent, &theme, Some(theme.icons.left()), "Send bitcoin", Page::Address);

            parent.spawn(interface.content).with_children(|parent| {
                amount_display(parent, &theme, None, &state_data.zeros, &format!("${}", &state_data.usd));
            });
            bumper.button_bumper(parent, &theme, vec![
                ("Continue", Page::Speed, false)
            ]);
        });
    });
}

pub fn amount_display_system(
    mut amount_display_query: Query<(&mut Text, &mut TextColor), With<AmountDisplayHelper>>,
    amount_screen_query: Query<(), With<OnAmountScreen>>,
    state_data: Res<StateData>,
    theme: Res<Theme>,
) {
    let is_amount_screen = !amount_screen_query.is_empty();
    let min = 0.31;
    let max = 25.31;

    if is_amount_screen {
        for (mut text, mut text_color) in amount_display_query.iter_mut() {
            let usd_value: f32 = state_data.usd.parse().unwrap_or(0.0);

            let (display_text, is_error) = if usd_value > max {
                (format!("${} maximum.", max), true)
            } else if usd_value < min && usd_value != 0.0 {
                (format!("${} minimum.", min), true)
            } else if usd_value == 0.0 {
                ("Type dollar amount.".to_string(), false)
            } else {
                (usd_to_btc(usd_value), false)
            };

            text.0 = display_text;
            text_color.0 = if is_error {
                theme.colors.status_danger
            } else {
                theme.colors.text_secondary
            };
        }
    }
}
