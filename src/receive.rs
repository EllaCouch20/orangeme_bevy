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

use qrcode::{QrCode, render::svg};
use image::{DynamicImage, RgbaImage, ImageBuffer, Rgba};
use std::path::Path;

use qrcode::types::Color;
use image::Luma;
use std::fs::File;
use std::env;


use crate::components::{
    text_input::text_input,
    navigator::sidebar_navigator,
    tip_button::tip_buttons,
};

use crate::utils::text;
use crate::PageState;

#[derive(Component)]
pub struct OnReceiveScreen;

pub fn receive_setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    fonts: Res<FontResources>,
    colors: Res<Display>,
    mut menu_state: ResMut<NextState<PageState>>,
) {
    let bumper = Bumper::new();
    let interface = Interface::new();
    let header = Header::new();

    let font = fonts.style.text.clone();
    let size = fonts.size.md;


    commands.spawn((
        interface.node,
        OnReceiveScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server, 0);

        parent.spawn(interface.page_node).with_children(|parent| {
            header.stack_header(parent, &fonts, &asset_server, &colors, Some(Icon::Left), "Receive bitcoin", Nav::Home);

            parent.spawn((interface.content_centered, Interaction::None)).with_children(|parent| {
                parent.spawn(
                    Node {
                        height: Val::Px(300.0),
                        width: Val::Px(300.0),
                        ..default()
                    },

                ).with_children(|parent| {
                    let qr_code_image = generate_qr_code_image("https://example.com");

                    ImageNode::new(asset_server.load(qr_code_image));
                });
                parent.spawn((
                    ImageNode::new(asset_server.load("app_icon.png")),
                    Node {
                        height: Val::Px(72.0),
                        width: Val::Px(72.0),
                        ..default()
                    },
                ));
                parent.spawn(text("Scan to receive bitcoin.", font, size, colors.text_secondary)); 
            });

            bumper.button_bumper(parent, &fonts, &asset_server, vec![
                (primary_default("Share"), Nav::Amount)
            ]);
        });
    });
}


fn generate_qr_code_image(data: &str) -> String {
    let code = QrCode::new(data).unwrap();

    let image = code.render::<Luma<u8>>().build();

    let temp_dir = env::temp_dir();
    let file_path = temp_dir.join("qr_code.png");
    let file_path_str = file_path.to_str().unwrap().to_string();

    image.save(&file_path).unwrap();

    file_path_str
}
