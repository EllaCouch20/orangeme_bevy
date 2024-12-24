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

use crate::utils::{EXPAND, text};

use crate::components::{
    navigator::sidebar_navigator,
};

use crate::PageState;

#[derive(Component)]
pub struct OnConfirmScreen;

pub fn confirm_setup(
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
        OnConfirmScreen,
    )).with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server, menu_state);

        parent.spawn(interface.page_node).with_children(|parent| {
            header.stack_header(parent, &fonts, &asset_server, &colors, Some(Icon::Left), "Confirm send", Nav::Amount);

            parent.spawn(interface.content).with_children(|parent| { 
                confirm_address(parent, &fonts, &asset_server, &colors);
                confirm_amount(parent, &fonts, &asset_server, &colors);
            });
            
            bumper.button_bumper(parent, &fonts, &asset_server, vec![
                (primary_default("Confirm & Send"), Nav::Success)
            ]);
        });
    });
}

pub fn confirm_address(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
    colors: &Res<Display>,
) {
    let h_font = fonts.style.heading.clone();
    let t_font = fonts.style.text.clone();

    let h6 = fonts.size.h6;
    let h5 = fonts.size.h5;
    let md = fonts.size.md;
    let sm = fonts.size.sm;

    let edit_address = (secondary_default("Address", Icon::Edit), Nav::Address);

    parent.spawn(
        Node {
            width: EXPAND,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(16.0),
            ..default()
        }
    ).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        )).with_children(|parent| {circle(parent, colors, h6, h_font.clone(), "1")});
        parent.spawn((
            Node {
                width: EXPAND,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
        )).with_children(|parent| {
            parent.spawn(text("Confirm address", h_font.clone(), h5, colors.text_heading));
            parent.spawn(text("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", t_font.clone(), md, colors.text_primary));
            parent.spawn(text("Bitcoin sent to the wrong address can never be recovered.", t_font.clone(), sm, colors.text_secondary));
            parent.spawn((Node::default(), edit_address.1)).with_children(|parent|{
                ButtonComponent::spawn_button(parent, asset_server, fonts, edit_address.0);
            });
        }); 
    }); 
}

pub fn confirm_amount(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
    colors: &Res<Display>,
) {
    let h_font = fonts.style.heading.clone();
    let t_font = fonts.style.text.clone();

    let h6 = fonts.size.h6;
    let h5 = fonts.size.h5;
    let md = fonts.size.md;
    let sm = fonts.size.sm;

    let edit_amount = (secondary_default("Amount", Icon::Edit), Nav::Amount);
    let edit_speed = (secondary_default("Speed", Icon::Edit), Nav::Speed);

    parent.spawn(Node {
        width: EXPAND,
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Start,
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(16.0),
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Px(32.0),
                height: Val::Px(32.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
        )).with_children(|parent| {circle(parent, colors, h6, h_font.clone(), "2")});
        parent.spawn((
            Node {
                width: EXPAND,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(16.0),
                ..default()
            },
        )).with_children(|parent| {
            parent.spawn(text("Confirm amount", h_font.clone(), h5, colors.text_heading));
            parent.spawn(Node {
                    width: EXPAND,
                    flex_direction: FlexDirection::Column,
                    ..default()
            }).with_children(|parent|{
                tabular("Date", "10/05/2025", t_font.clone(), sm, colors, parent);
                tabular("Time", "8:45 PM", t_font.clone(), sm, colors, parent);
                tabular("Sent to Address", "1A1zP1eP5...fNa", t_font.clone(), sm, colors, parent);
                tabular("Amount Sent", "0.00001234 BTC", t_font.clone(), sm, colors, parent);
            });
            parent.spawn ((
                Node {
                    width: EXPAND,
                    justify_content: JustifyContent::Start,
                    align_items: AlignItems::Start,
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(16.0),
                    ..default()
                }
            )).with_children(|parent| {
                parent.spawn((Node::default(), edit_amount.1)).with_children(|parent|{
                    ButtonComponent::spawn_button(parent, asset_server, fonts, edit_amount.0);
                });
                parent.spawn((Node::default(), edit_speed.1)).with_children(|parent|{
                    ButtonComponent::spawn_button(parent, asset_server, fonts, edit_speed.0);
                });
            });
        }); 
    }); 
}

pub fn tabular(
    title: &str,
    content: &str,
    font: Handle<Font>,
    size: f32,
    colors: &Res<Display>,
    parent: &mut ChildBuilder,
){
    parent.spawn(Node {
        width: EXPAND,
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Start,
        flex_direction: FlexDirection::Row,
        padding: UiRect {
            top: Val::Px(4.0),
            bottom: Val::Px(4.0),
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn(text(title, font.clone(), size, colors.text_primary));
        parent.spawn(text(content, font, size, colors.text_primary));
    }); 
}

fn circle(
    parent: &mut ChildBuilder,
    colors: &Res<Display>,
    size: f32,
    font: Handle<Font>,
    txt: &str,
) {
    parent.spawn((
        Node {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        BackgroundColor(colors.bg_secondary),
        BorderRadius::MAX,
    )).with_children(|parent| {
        parent.spawn((
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            text(txt, font, size, colors.text_heading)
        ));
    }); 
}