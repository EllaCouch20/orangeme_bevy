#![allow(unused)]

mod home;
mod address;
mod amount;

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
    pub mod navigator; 
    pub mod text_input;
    pub mod tip_button;
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

use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};
use crate::primitives::button::{button_system, InteractiveState};

use crate::home::{OnHomeScreen, home_setup};
use crate::address::{OnAddressScreen, address_setup};
use crate::amount::{OnAmountScreen, amount_setup, keyboard_input_system, amount_display_system};
use crate::components::text_input::focus;

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
        .add_systems(Update, focus.before(TextInputSystem))
        .insert_resource(ClearColor(Colors::tapa().shade1000)) 
        .add_plugins((menu_plugin))
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
pub enum NavigateTo {
    Home,
    Amount,
    Address,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PageState {
    Home,
    Amount,
    Address,
    #[default]
    Disabled,
}

#[derive(Resource)]
pub struct StateData {
    usd: String,
}

pub fn menu_plugin(app: &mut App) {
    let state_data = StateData {usd: "0".to_string()};
    app
        .init_state::<PageState>()
        .insert_resource(state_data) 
        .add_systems(OnEnter(GameState::Menu), startup_setup)
        .add_systems(OnEnter(PageState::Home), home_setup)
        .add_systems(OnExit(PageState::Home), despawn_screen::<OnHomeScreen>)
        .add_systems(OnEnter(PageState::Address), address_setup)
        .add_systems(OnExit(PageState::Address), despawn_screen::<OnAddressScreen>)
        .add_systems(OnEnter(PageState::Amount), amount_setup)
        .add_systems(OnExit(PageState::Amount), despawn_screen::<OnAmountScreen>)
        .add_systems(PreStartup, setup_fonts)
        .add_systems(Update, button_system)
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, amount_display_system)
        .add_systems(Update, (menu_action, button_system).run_if(in_state(GameState::Menu)));
}

pub fn startup_setup(mut menu_state: ResMut<NextState<PageState>>) {
    menu_state.set(PageState::Home);
}

pub fn menu_action(
    interaction_query: Query<
        (&Interaction, &NavigateTo),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<PageState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                NavigateTo::Home => menu_state.set(PageState::Home),
                NavigateTo::Address => menu_state.set(PageState::Address),
                NavigateTo::Amount => menu_state.set(PageState::Amount)
            }
        }
    }
}
