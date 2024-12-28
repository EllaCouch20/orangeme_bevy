mod theme;
mod bitcoin;
mod utils;

pub mod components {
    pub mod balance_display;
    pub mod amount_display;
    pub mod numeric_keypad;
    pub mod navigator; 
    pub mod text_input;
    pub mod tip_button;
    pub mod input;
    pub mod radio;
    pub mod interfaces;
    pub mod profile_photo;
    pub mod button;
    pub mod button_presets;
}

use bevy::prelude::*;

use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};

use theme::Theme;

use crate::theme::color::Colors;
use crate::components::button::InteractiveState;
use crate::components::button::ButtonStyle;
use crate::components::radio::toggle_radio_buttons;
use crate::components::input::keyboard_input_system;
use crate::components::button::button_system;
use crate::components::text_input::text_input_visuals_system;

use crate::components::navigator::sidebar_navigation_system;

use crate::bitcoin::{
    home::{OnHomeScreen, home_setup},
    address::{OnAddressScreen, address_setup, address_input_system},
    amount::{OnAmountScreen, amount_setup, amount_display_system},
    speed::{OnSpeedScreen, speed_setup},
    confirm::{OnConfirmScreen, confirm_setup},
    success::{OnSuccessScreen, success_setup},
    receive::{OnReceiveScreen, receive_setup},
};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States, Component)]
pub enum Page {
    #[default]
    Home,
    Amount,
    Address,
    Confirm,
    Speed,
    Success,
    Receive,
}

#[derive(Resource)]
pub struct StateData {
    balance_usd: f32,
    usd: String,
    zeros: String,
    helper: String,
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "orange".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Colors::tapa().shade1000))
        .add_plugins(TextInputPlugin)
        .add_systems(Update, navigation_system)
        .add_systems(PreStartup, startup_system)
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, toggle_radio_buttons)
        .add_systems(Update, button_system)
        .add_systems(Update, sidebar_navigation_system)
        .add_systems(Update, address_input_system)
        .add_systems(Update, amount_display_system)
        .add_systems(Update, text_input_visuals_system.before(TextInputSystem))
        .add_systems(OnEnter(Page::Home), home_setup)
        .add_systems(OnExit(Page::Home), despawn_screen::<OnHomeScreen>)
        .add_systems(OnEnter(Page::Address), address_setup)
        .add_systems(OnExit(Page::Address), despawn_screen::<OnAddressScreen>)
        .add_systems(OnEnter(Page::Amount), amount_setup)
        .add_systems(OnExit(Page::Amount), despawn_screen::<OnAmountScreen>)
        .add_systems(OnEnter(Page::Speed), speed_setup)
        .add_systems(OnExit(Page::Speed), despawn_screen::<OnSpeedScreen>)
        .add_systems(OnEnter(Page::Confirm), confirm_setup)
        .add_systems(OnExit(Page::Confirm), despawn_screen::<OnConfirmScreen>)
        .add_systems(OnEnter(Page::Success), success_setup)
        .add_systems(OnExit(Page::Success), despawn_screen::<OnSuccessScreen>)
        .add_systems(OnEnter(Page::Receive), receive_setup)
        .add_systems(OnExit(Page::Receive), despawn_screen::<OnReceiveScreen>)
        .run();
}

fn startup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let state_data = StateData {
        usd: "0".to_string(), 
        zeros: "".to_string(), 
        balance_usd: 25.0, 
        helper: "0.00001234 BTC".to_string()
    };

    commands.spawn(Camera2d);
    commands.insert_resource(Theme::new(&asset_server));
    commands.insert_resource(state_data);
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn navigation_system(
    interaction_query: Query<
        (&Interaction, &Parent, &InteractiveState),
        (Changed<Interaction>, With<Button>),
    >,
    nav_query: Query<&Page>,
    mut page_state: ResMut<NextState<Page>>,
) {
    for (interaction, parent, state) in &interaction_query {
        if *interaction == Interaction::Pressed && *state != InteractiveState::Disabled {
            if let Ok(navigate_to) = nav_query.get(parent.get()) {
                page_state.set(*navigate_to);
            }
        }
    }
}
