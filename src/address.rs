use bevy::{prelude::*, ui::FocusPolicy};

use super::despawn_screen;

use crate::{
    menu_plugin,
    NavigateTo
};

use crate::theme::{
    color::Display,
    fonts::FontResources
};

use crate::interface::{
    header::{ header, Header },
    bumper::Bumper,
    interfaces::Interface
};

use crate::primitives::{
    profile_photo::profile_photo,
    button::{
        button_system,
        primary_default,
    },
};

use crate::components::{
    text_input::text_input,
    navigator::sidebar_navigator,
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
    
    let next = primary_default("Continue", true, NavigateTo::Home);

    commands.spawn((
        interface.node,
        OnAddressScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server);

        parent.spawn((interface.page_node, Interaction::None)).with_children(|parent| {
            header(parent, &fonts, &asset_server, Header::Home, "Wallet");

            parent.spawn(interface.content).with_children(|parent| {
                text_input(parent, &fonts);
            });

            bumper.button_bumper(parent, &fonts, &asset_server, vec![next]);
        });
    });
}