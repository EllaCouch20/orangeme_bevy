use bevy::{prelude::*, ui::FocusPolicy};

use super::despawn_screen;

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

use crate::primitives::button_presets::{primary_default, secondary_default};

use crate::components::{
    text_input::text_input,
    navigator::sidebar_navigator,
    tip_button::tip_buttons,
};

use bevy_simple_text_input::{
    TextInput, 
    TextInputTextFont,
    TextInputTextColor,
    TextInputPlaceholder,
    TextInputInactive,
};

#[derive(Component)]
pub struct OnAddressScreen;

pub fn address_setup(mut commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontResources>) {

    let colors = Display::new();
    let bumper = Bumper::new();
    let interface = Interface::new();

    let next = primary_default("Continue", true, InteractiveState::Disabled, NavigateTo::Amount);

    commands.spawn((
        interface.node,
        OnAddressScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server);

        parent.spawn(interface.page_node).with_children(|parent| {
            header(parent, &fonts, &asset_server, Header::Stack, "Bitcoin address");

            parent.spawn((interface.content, Interaction::None)).with_children(|parent| { 
                radio_button(parent, &fonts, &asset_server, "Standard", "Arrives in ~2 hours\n$0.18 bitcoin network fee", 0);
                radio_button(parent, &fonts, &asset_server, "Priority", "Arrives in ~30 minutes\n$0.35 bitcoin network fee", 0);
            });
            
            bumper.button_bumper(parent, &fonts, &asset_server, vec![next]);
        });
    });
}