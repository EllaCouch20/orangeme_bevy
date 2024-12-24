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

use crate::primitives::button::CustomButton;
use crate::ButtonStyle;
use crate::ButtonColor;
use crate::PageState;
use crate::primitives::button_presets::{primary_disabled, secondary_default};
use crate::components::tip_button::Tip;
use crate::components::{
    text_input::text_input,
    navigator::sidebar_navigator,
    tip_button::tip_buttons,
};

#[derive(Component)]
pub struct OnAddressScreen;

pub fn address_setup(
    mut menu_state: ResMut<NextState<PageState>>,
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
        sidebar_navigator(parent, &fonts, &asset_server, 0);

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
                (primary_disabled("Continue"), Nav::Amount)
            ]);
        });
    });
}

// ===== Button Color Handler ===== //


pub fn address_tip_system(
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &Parent,
            &mut BackgroundColor,
            &mut BorderColor,
            Option<&ButtonStyle>,
            &mut InteractiveState,
        ),
        (With<Button>, Changed<Interaction>),
    >,
    tip_query: Query<&Tip>,
    address_screen_query: Query<(), With<OnAddressScreen>>
) {
    if address_screen_query.is_empty() {return};

    for (entity, interaction, parent, mut color, mut border_color, button_style, mut state) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            println!("Interaction Query Entity: {:?}", entity);
            if let Ok(tip) = tip_query.get(parent.get()) {
                println!("TIP: {:?}", tip);
            }
        }
    }
}

pub fn button_status_system(
    mut param_set: ParamSet<(
        Query<(
            Entity,
            &TextInputValue,
        ), Changed<TextInputValue>>,
    )>,
    mut p_set: ParamSet<(
        Query<(
            &mut CustomButton, 
            Option<&ButtonStyle>,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut InteractiveState,
            &Children,
        ), With<Button>>,
    )>,
    mut text_query: Query<(&mut TextColor, &mut Text, &Parent), With<Text>>,
    address_screen_query: Query<(), With<OnAddressScreen>>
) {
    if address_screen_query.is_empty() {return};
    
    for (_, input_value) in param_set.p0().iter() {
        
        let mut button_query = p_set.p0();
        for (mut data, button_style, mut color, mut border_color, mut state, children) in button_query.iter_mut() {
            for &child in children.iter() {
                if let Ok(text) = text_query.get(child) {
                    if text.1.0 == "Continue" {

                        *state = if input_value.0.is_empty() {
                            InteractiveState::Disabled
                        } else {
                            InteractiveState::Default
                        };
            
                        if let Some(button_style) = button_style {
                            let button_colors: ButtonColor = ButtonColor::new(*button_style, *state);
                            *color = button_colors.background.into();
                            *border_color = button_colors.outline.into();
                            for child in children.iter() {
                                if let Ok((mut text_color, _, _)) = text_query.get_mut(*child) {
                                    *text_color = button_colors.label.into();
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
