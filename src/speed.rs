use bevy::{prelude::*, ui::FocusPolicy};

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
use crate::components::radio::radio_button;

use crate::components::{
    navigator::sidebar_navigator,
};
use crate::PageState;

#[derive(Component)]
pub struct OnSpeedScreen;

#[derive(Component)]
pub struct RadioButtonState {
    pub selected: bool,
}


pub fn speed_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    fonts: Res<FontResources>,
    colors: Res<Display>,
    mut menu_state: ResMut<NextState<PageState>>,
) {

    let bumper = Bumper::new();
    let interface = Interface::new();
    let header = Header::new();

    commands.spawn((
        interface.node,
        OnSpeedScreen,
    )).with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server, menu_state);

        parent.spawn(interface.page_node).with_children(|parent| {
            header.stack_header(parent, &fonts, &asset_server, &colors, Some(Icon::Left), "Transaction speed", Nav::Amount);

            parent.spawn(interface.content).with_children(|parent| { 
                radio_button(parent, &fonts, &colors, &asset_server, "Standard", "Arrives in ~2 hours\n$0.18 bitcoin network fee", 0, true);
                radio_button(parent, &fonts, &colors, &asset_server, "Priority", "Arrives in ~30 minutes\n$0.35 bitcoin network fee", 0, false);
            });
            
            bumper.button_bumper(parent, &fonts, &asset_server, vec![
                (primary_default("Continue"), Nav::Confirm)
            ]);
        });
    });
}

