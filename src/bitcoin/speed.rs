use bevy::prelude::*;

use crate::components::{
    radio::radio_button,
    interfaces::Header,
    interfaces::Bumper,
    interfaces::Interface,
    navigator::sidebar_navigator,
};

use crate::Page;
use crate::Theme;

#[derive(Component)]
pub struct OnSpeedScreen;

#[derive(Component)]
pub struct RadioButtonState {
    pub selected: bool,
}


pub fn speed_setup(
    mut commands: Commands, 
    theme: Res<Theme>,
) {

    let bumper = Bumper::new();
    let interface = Interface::new();
    let header = Header::new();

    commands.spawn((
        interface.node,
        OnSpeedScreen,
    )).with_children(|parent| {
        sidebar_navigator(parent, &theme, 0);

        parent.spawn(interface.page_node).with_children(|parent| {
            header.stack_header(parent, &theme, Some(theme.icons.left()), "Transaction speed", Page::Amount);

            parent.spawn(interface.content).with_children(|parent| { 
                radio_button(parent, &theme, "Standard", "Arrives in ~2 hours\n$0.18 bitcoin network fee", true);
                radio_button(parent, &theme, "Priority", "Arrives in ~30 minutes\n$0.35 bitcoin network fee", false);
            });
            
            bumper.button_bumper(parent, &theme, vec![
                ("Continue", Page::Confirm, true),
            ]);
        });
    });
}

