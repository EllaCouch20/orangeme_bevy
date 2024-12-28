use bevy::prelude::*;

use crate::utils::text;

use crate::components::{
    interfaces::Header,
    interfaces::Bumper,
    interfaces::Interface,
    navigator::sidebar_navigator,
};

use crate::Page;
use crate::Theme;

#[derive(Component)]
pub struct OnSuccessScreen;

pub fn success_setup(
    mut commands: Commands, 
    theme: Res<Theme>,
) {

    let bumper = Bumper::new();
    let interface = Interface::new();
    let header = Header::new();

    let font = theme.fonts.style.heading.clone();
    let size = theme.fonts.size.h3;

    commands.spawn((
        interface.node,
        OnSuccessScreen,
    )).with_children(|parent| {
        sidebar_navigator(parent, &theme, 0);

        parent.spawn(interface.page_node).with_children(|parent| {
            header.stack_header(parent, &theme, Some(theme.icons.close()), "Send confirmed", Page::Home);

            parent.spawn(interface.content_centered).with_children(|parent| { 
                parent.spawn((
                    theme.icons.bitcoin(),
                    Node {
                        height: Val::Px(128.0),
                        width: Val::Px(128.0),
                        ..default()
                    },
                ));
                parent.spawn(text("You sent $10.00", font, size, theme.colors.text_heading));
            });
            
            bumper.secondary_button_bumper(parent, &theme, ("Done", Page::Home));
        });
    });
}

