use bevy::prelude::*;

use crate::components::{
    interfaces::Header,
    interfaces::Bumper,
    interfaces::Interface,
    balance_display::balance_display,
    navigator::sidebar_navigator,
};

use crate::Page;
use crate::Theme;

#[derive(Component)]
pub struct OnHomeScreen;

pub fn home_setup(
    mut commands: Commands,
    theme: Res<Theme>,
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
        sidebar_navigator(parent, &theme, 0);

        parent.spawn(interface.page_node).with_children(|parent| {
            header.home_header(parent, &theme, "Wallet");

            parent.spawn(interface.content_centered).with_children(|parent| {
                balance_display(parent, &theme, balance_usd, balance_btc);
            });

            bumper.button_bumper(parent, &theme, vec![
                ("Receive", Page::Receive, true), 
                ("Send", Page::Address, true),
            ]);
        });
    });
}