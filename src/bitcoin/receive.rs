use bevy::prelude::*;

use crate::components::{
    interfaces::Header,
    interfaces::Bumper,
    interfaces::Interface,
    navigator::sidebar_navigator,
};

use crate::Page;
use crate::Theme;
use crate::utils::text;

#[derive(Component)]
pub struct OnReceiveScreen;

pub fn receive_setup(
    mut commands: Commands, 
    theme: Res<Theme>,
) {
    let bumper = Bumper::new();
    let interface = Interface::new();
    let header = Header::new();

    let font = theme.fonts.style.text.clone();
    let size = theme.fonts.size.md;


    commands.spawn((
        interface.node,
        OnReceiveScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &theme, 0);

        parent.spawn(interface.page_node).with_children(|parent| {
            header.stack_header(parent, &theme, Some(theme.icons.left()), "Receive bitcoin", Page::Home);

            parent.spawn((interface.content_centered, Interaction::None)).with_children(|parent| {
                parent.spawn(text("Scan to receive bitcoin.", font, size, theme.colors.text_secondary)); 
            });

            bumper.button_bumper(parent, &theme, vec![
                ("Share", Page::Amount, true),
            ]);
        });
    });
}

