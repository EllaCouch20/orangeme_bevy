use bevy::{prelude::*, ui::FocusPolicy};
use bevy_simple_text_input::TextInputValue;

use super::despawn_screen;

use crate::{
    menu_plugin,
    Nav
};

use crate::theme::{
    color::Display,
    fonts::FontResources,
    icons::Icon,
};

use crate::interface::{
    header::Header,
    bumper::Bumper,
    interfaces::Interface
};

use crate::primitives::{
    profile_photo::profile_photo,
    button::{
        InteractiveState,
        ButtonComponent,
    },
};

use crate::primitives::button_presets::{primary_default, secondary_default};
use crate::components::tip_button::Tip;

use crate::components::{
    text_input::text_input,
    navigator::sidebar_navigator,
    tip_button::tip_buttons,
};

#[derive(Component)]
pub struct OnAddressScreen;

pub fn address_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    fonts: Res<FontResources>,
    colors: Res<Display>,
) {
    let bumper = Bumper::new();
    let interface = Interface::new();
    let header = Header::new();

    commands.spawn((
        interface.node,
        OnAddressScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server);

        parent.spawn(interface.page_node).with_children(|parent| {
            header.stack_header(parent, &fonts, &asset_server, &colors, Some(Icon::Left), "Bitcoin address", Nav::Home);

            parent.spawn((interface.content, Interaction::None)).with_children(|parent| { 
                text_input(parent, &fonts, "Bitcoin address...");
                tip_buttons(parent, &asset_server, &colors, &fonts, vec![
                    (secondary_default("Paste Clipboard", Icon::Paste), Tip::PasteClipboard), 
                    (secondary_default("Scan QR Code", Icon::QrCode), Tip::ScanQRCode),
                    (secondary_default("Select Contact", Icon::Profile), Tip::SelectContact),
                ]);
            });

            bumper.button_bumper(parent, &fonts, &asset_server, vec![
                (primary_default("Continue"), Nav::Amount)
            ]);
        });
    });
}
