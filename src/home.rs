use bevy::prelude::*;

use super::despawn_screen;

use crate::primitives::button_presets::primary_default;

use crate::{
    menu_plugin,
    Nav
};

use crate::theme::{
    color::Display,
    fonts::FontResources
};

use crate::interface::{
    header::Header,
    bumper::Bumper,
    interfaces::Interface
};

use crate::primitives::{
    profile_photo::profile_photo,
    button::InteractiveState,
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
    let header = Header::new();

    let balance_usd = "$0.00";
    let balance_btc = "0.00001234 BTC";

    commands.spawn((
        interface.node,
        OnHomeScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server);

        parent.spawn(interface.page_node).with_children(|parent| {
            header.home_header(parent, &fonts, &asset_server, &colors, "Wallet");

            parent.spawn(interface.content_centered).with_children(|parent| {
                balance_display(parent, &fonts, balance_usd, balance_btc);
            });

            bumper.button_bumper(parent, &fonts, &asset_server, vec![
                (primary_default("Receive"), Nav::Address), 
                (primary_default("Send"), Nav::Address)
            ]);
        });
    });
}