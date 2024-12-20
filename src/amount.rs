use bevy::{prelude::*, ui::FocusPolicy};

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
    balance_display::balance_display,
};

use bevy_simple_text_input::{
    TextInput, 
    TextInputTextFont,
    TextInputTextColor,
    TextInputPlaceholder,
    TextInputInactive,
};

#[derive(Component)]
pub struct OnAmountScreen;

pub fn amount_setup(mut commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontResources>) {

    let colors = Display::new();
    let bumper = Bumper::new();
    let interface = Interface::new();

    let next = primary_default("Continue", false, InteractiveState::Default, NavigateTo::Home);

    let updated: &str = "0.00";

    let usd: &str = &format!("${}", updated);

    commands.spawn((
        interface.node,
        OnAmountScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server);

        parent.spawn((interface.page_node, Interaction::None)).with_children(|parent| {
            header(parent, &fonts, &asset_server, Header::Stack, "Send bitcoin");
            parent.spawn(interface.content).with_children(|parent| {
                balance_display(parent, &fonts, usd, "Type bitcoin amount.");
            });
            bumper.button_bumper(parent, &fonts, &asset_server, vec![next]);
        });
    });
}