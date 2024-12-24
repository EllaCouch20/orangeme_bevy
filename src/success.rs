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

use crate::primitives::button_presets::{secondary_wide, secondary_default};
use crate::components::radio::radio_button;
use crate::utils::text;

use crate::components::{
    navigator::sidebar_navigator,
};

use crate::PageState;

#[derive(Component)]
pub struct OnSuccessScreen;

pub fn success_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    fonts: Res<FontResources>,
    colors: Res<Display>,
    mut menu_state: ResMut<NextState<PageState>>,
) {

    let bumper = Bumper::new();
    let interface = Interface::new();
    let header = Header::new();

    let font = fonts.style.heading.clone();
    let size = fonts.size.h3;

    commands.spawn((
        interface.node,
        OnSuccessScreen,
    )).with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server, 0);

        parent.spawn(interface.page_node).with_children(|parent| {
            header.stack_header(parent, &fonts, &asset_server, &colors, Some(Icon::Close), "Send confirmed", Nav::Home);

            parent.spawn(interface.content_centered).with_children(|parent| { 
                parent.spawn((
                    Icon::new(Icon::Bitcoin, &asset_server),
                    Node {
                        height: Val::Px(128.0),
                        width: Val::Px(128.0),
                        ..default()
                    },
                ));
                parent.spawn(text("You sent $10.00", font, size, colors.text_heading));
            });
            
            bumper.button_bumper(parent, &fonts, &asset_server, vec![
                (secondary_wide("Done"), Nav::Home)
            ]);
        });
    });
}

