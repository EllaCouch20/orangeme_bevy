use bevy::prelude::*;

use super::despawn_screen;

use crate::primitives::button_presets::primary_default;

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
        InteractiveState,
    },
};

use crate::components::{
    balance_display::balance_display,
    navigator::sidebar_navigator,
};


#[derive(Component)]
pub struct OnHomeScreen;

pub fn home_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    fonts: Res<FontResources>,
    colors: Res<Display>,
) {

    let bumper = Bumper::new();
    let interface = Interface::new();
    
    let send = primary_default("Send", false, InteractiveState::Default, NavigateTo::Address);
    let receive = primary_default("Receive", false, InteractiveState::Default, NavigateTo::Address);

    commands.spawn((
        interface.node,
        OnHomeScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server);

        parent.spawn(interface.page_node).with_children(|parent| {
            header(parent, &fonts, &asset_server, &colors, Header::Home, "Wallet");

            parent.spawn(interface.content_centered).with_children(|parent| {
                balance_display(parent, &fonts, "$0.00", "0.00000000 BTC");
            });

            bumper.button_bumper(parent, &fonts, &asset_server, vec![receive, send]);
        });
    });
}