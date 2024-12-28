use bevy::prelude::*;

use crate::Page;
use crate::Theme;
use crate::utils::EXPAND;
use crate::utils::text;

use crate::theme::color::ColorResources;

use crate::components::{
    interfaces::Header,
    interfaces::Bumper,
    interfaces::Interface,
    navigator::sidebar_navigator,
    button_presets::secondary_default,
};


#[derive(Component)]
pub struct OnConfirmScreen;

pub fn confirm_setup(
    mut commands: Commands, 
    theme: Res<Theme>,
) {

    let bumper = Bumper::new();
    let interface = Interface::new();
    let header = Header::new();

    commands.spawn((
        interface.node,
        OnConfirmScreen,
    )).with_children(|parent| {
        sidebar_navigator(parent, &theme, 0);

        parent.spawn(interface.page_node).with_children(|parent| {
            header.stack_header(parent, &theme, Some(theme.icons.left()), "Confirm send", Page::Amount);

            parent.spawn(interface.content).with_children(|parent| { 
                confirm_address(parent, &theme);
                confirm_amount(parent, &theme);
            });
            
            bumper.button_bumper(parent, &theme, vec![
                ("Confirm & Send", Page::Success, true)
            ]);
        });
    });
}

pub fn confirm_address(
    parent: &mut ChildBuilder,
    theme: &Res<Theme>,
) {
    let h_font = theme.fonts.style.heading.clone();
    let t_font = theme.fonts.style.text.clone();

    let h6 = theme.fonts.size.h6;
    let h5 = theme.fonts.size.h5;
    let md = theme.fonts.size.md;
    let sm = theme.fonts.size.sm;

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
        )).with_children(|parent| {circle(parent, theme, h6, h_font.clone(), "1")});
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
            parent.spawn(text("Confirm address", h_font.clone(), h5, theme.colors.text_heading));
            parent.spawn(text("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa", t_font.clone(), md, theme.colors.text_primary));
            parent.spawn(text("Bitcoin sent to the wrong address can never be recovered.", t_font.clone(), sm, theme.colors.text_secondary));
            secondary_default("Address", theme.icons.edit()).create_on(parent, Page::Address, &theme);
        }); 
    }); 
}

pub fn confirm_amount(
    parent: &mut ChildBuilder,
    theme: &Res<Theme>,
) {
    let h_font = theme.fonts.style.heading.clone();
    let t_font = theme.fonts.style.text.clone();

    let h6 = theme.fonts.size.h6;
    let h5 = theme.fonts.size.h5;
    let sm = theme.fonts.size.sm;

    parent.spawn(Node {
        width: EXPAND,
        justify_content: JustifyContent::Start,
        align_items: AlignItems::Start,
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(16.0),
        ..default()
    }).with_children(|parent| {
        parent.spawn(Node {
            width: Val::Px(32.0),
            height: Val::Px(32.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }).with_children(|parent| {circle(parent, theme, h6, h_font.clone(), "2")});

        parent.spawn(Node {
            width: EXPAND,
            justify_content: JustifyContent::Start,
            align_items: AlignItems::Start,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        }).with_children(|parent| {
            parent.spawn(text("Confirm amount", h_font.clone(), h5, theme.colors.text_heading));

            parent.spawn(Node {
                width: EXPAND,
                flex_direction: FlexDirection::Column,
                ..default()
            }).with_children(|parent|{
                tabular("Date", "10/05/2025", t_font.clone(), sm, theme.colors, parent);
                tabular("Time", "8:45 PM", t_font.clone(), sm, theme.colors, parent);
                tabular("Sent to Address", "1A1zP1eP5...fNa", t_font.clone(), sm, theme.colors, parent);
                tabular("Amount Sent", "0.00001234 BTC", t_font.clone(), sm, theme.colors, parent);
            });

            parent.spawn(Node {
                width: EXPAND,
                justify_content: JustifyContent::Start,
                align_items: AlignItems::Start,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(16.0),
                ..default()
            }).with_children(|parent| {
                secondary_default("Amount", theme.icons.edit()).create_on(parent, Page::Amount, &theme);
                secondary_default("Speed", theme.icons.edit()).create_on(parent, Page::Speed, &theme);
            });
        }); 
    }); 
}

pub fn tabular(
    title: &str,
    content: &str,
    font: Handle<Font>,
    size: f32,
    colors: ColorResources,
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
    theme: &Res<Theme>,
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
        BackgroundColor(theme.colors.bg_secondary),
        BorderRadius::MAX,
    )).with_children(|parent| {
        parent.spawn((
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            text(txt, font, size, theme.colors.text_heading)
        ));
    }); 
}