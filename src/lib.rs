#![allow(unused)]

mod home;
mod address;
mod amount;
mod utils;
mod speed;
mod confirm;
mod success;
mod receive;

pub mod primitives { 
    pub mod button; 
    pub mod profile_photo;
    pub mod button_presets;
}

pub mod theme { 
    pub mod icons; 
    pub mod color; 
    pub mod fonts; 
}

pub mod components {
    pub mod balance_display;
    pub mod amount_display;
    pub mod numeric_keypad;
    pub mod navigator; 
    pub mod text_input;
    pub mod tip_button;
    pub mod input;
    pub mod radio;
}

pub mod interface {
    pub mod bumper;
    pub mod header;
    pub mod interfaces;
}

use bevy::prelude::*;
use bevy_ui::prelude::*;

use theme::{
    color::Colors,
    fonts::setup_fonts
};

use crate::theme::color::{Display, ButtonColor};
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};
use crate::primitives::button::{
    InteractiveState,
    ButtonStyle,
};

use crate::components::radio::toggle_radio_buttons;

use crate::home::{OnHomeScreen, home_setup};
use crate::address::{OnAddressScreen, address_setup, button_status_system, address_tip_system};
use crate::amount::{OnAmountScreen, amount_setup, amount_button_status, amount_display_system};
use crate::speed::{OnSpeedScreen, speed_setup};
use crate::confirm::{OnConfirmScreen, confirm_setup};
use crate::success::{OnSuccessScreen, success_setup};
use crate::receive::{OnReceiveScreen, receive_setup};
use crate::components::navigator::navigation_system;
use crate::components::input::keyboard_input_system;
use crate::components::text_input::text_input_visuals_system;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "orange".into(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(TextInputPlugin)
        .add_systems(Update, text_input_visuals_system.before(TextInputSystem))
        .insert_resource(ClearColor(Colors::tapa().shade1000)) 
        .add_plugins(menu_plugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

#[derive(Component, Clone, Copy)]
pub enum Nav {
    Home,
    Amount,
    Address,
    Confirm,
    Speed,
    Success,
    Receive,
    None,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PageState {
    Home,
    Amount,
    Address,
    Confirm,
    Speed,
    Success,
    Receive,
    #[default]
    Disabled,
}

#[derive(Resource)]
pub struct StateData {
    balance_usd: f32,
    usd: String,
    zeros: String,
    helper: String,
}

fn menu_plugin(app: &mut App) {
    let colors = Display::new();

    let state_data = StateData {
        usd: "0".to_string(), 
        zeros: "".to_string(), 
        balance_usd: 25.0, 
        helper: "0.00001234 BTC".to_string()
    };

    app
        .init_state::<PageState>()
        .insert_resource(state_data) 
        .insert_resource(colors)
        .add_systems(OnEnter(GameState::Menu), startup_setup)
        .add_systems(OnEnter(PageState::Home), home_setup)
        .add_systems(OnExit(PageState::Home), despawn_screen::<OnHomeScreen>)
        .add_systems(OnEnter(PageState::Address), address_setup)
        .add_systems(OnExit(PageState::Address), despawn_screen::<OnAddressScreen>)
        .add_systems(OnEnter(PageState::Amount), amount_setup)
        .add_systems(OnExit(PageState::Amount), despawn_screen::<OnAmountScreen>)
        .add_systems(OnEnter(PageState::Speed), speed_setup)
        .add_systems(OnExit(PageState::Speed), despawn_screen::<OnSpeedScreen>)
        .add_systems(OnEnter(PageState::Confirm), confirm_setup)
        .add_systems(OnExit(PageState::Confirm), despawn_screen::<OnConfirmScreen>)
        .add_systems(OnEnter(PageState::Success), success_setup)
        .add_systems(OnExit(PageState::Success), despawn_screen::<OnSuccessScreen>)
        .add_systems(OnEnter(PageState::Receive), receive_setup)
        .add_systems(OnExit(PageState::Receive), despawn_screen::<OnReceiveScreen>)
        .add_systems(PreStartup, setup_fonts)
        .add_systems(Update, amount_button_status)
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, amount_display_system)
        .add_systems(Update, button_status_system)
        .add_systems(Update, button_system)
        .add_systems(Update, toggle_radio_buttons)
        .add_systems(Update, address_tip_system)
        .add_systems(Update, navigation_system)
        .add_systems(Update, menu_action.run_if(in_state(GameState::Menu)));
}

fn startup_setup(mut menu_state: ResMut<NextState<PageState>>) {
    menu_state.set(PageState::Home);
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &Parent, &InteractiveState),
        (Changed<Interaction>, With<Button>),
    >,
    nav_query: Query<&Nav>,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<PageState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, parent, state) in &interaction_query {
        if *interaction == Interaction::Pressed && *state != InteractiveState::Disabled {
            if let Ok(nav) = nav_query.get(parent.get()) {
                match nav {
                    Nav::Home => {
                        menu_state.set(PageState::Home);
                    }
                    Nav::Address => {
                        menu_state.set(PageState::Address);
                    }
                    Nav::Amount => {
                        menu_state.set(PageState::Amount);
                    }
                    Nav::Speed => {
                        menu_state.set(PageState::Speed);
                    }
                    Nav::Confirm => {
                        menu_state.set(PageState::Confirm);
                    }
                    Nav::Success => {
                        menu_state.set(PageState::Success);
                    }
                    Nav::Receive => {
                        menu_state.set(PageState::Receive);
                    }
                    Nav::None => {}
                }
            }
        }
    }
}

pub fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&ButtonStyle>,
            &InteractiveState,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, button_style, state) in &mut interaction_query {
        if *state != InteractiveState::Disabled && *state != InteractiveState::Selected {
            if let Some(button_style) = button_style {
                match *interaction {
                    Interaction::Hovered => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Hover);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                    Interaction::Pressed => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Selected);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                    Interaction::None => {
                        let colors: ButtonColor = ButtonColor::new(*button_style, InteractiveState::Default);
                        *color = colors.background.into();
                        border_color.0 = colors.outline;
                    }
                }
            }
        }
    }
}