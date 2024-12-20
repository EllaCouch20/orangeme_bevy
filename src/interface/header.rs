use bevy::prelude::*;
use crate::primitives::profile_photo::profile_photo;
use std::sync::Arc;

use crate::theme::{
    color::Display,
    fonts::FontResources,
    icons::Icon,
};

pub enum Header {
    Home,
    Stack
}

pub fn header(
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
    asset_server: &Res<AssetServer>,
    header_type: Header,
    title: &str,
) {
    let colors = Display::new();

    let header_node = Node {
        width: Val::Percent(100.0),
        height: Val::Px(88.0),
        align_items: AlignItems::Start,
        justify_content: JustifyContent::SpaceBetween,
        flex_direction: FlexDirection::Row,
        padding: UiRect::all(Val::Px(24.0)),
        ..default()
    };

    match header_type {
        Header::Home => {
            parent.spawn((header_node)).with_children(|parent| { 
                profile_photo(parent, &fonts, &asset_server, "profile_photo.png");
                header_title("Wallet", fonts.size.h3, parent, &fonts);
                header_icon(None, parent, &asset_server);
            });
        },
        Header::Stack => {
            parent.spawn((header_node)).with_children(|parent| { 
                header_icon(Some(Icon::Left), parent, &asset_server);
                header_title(title, fonts.size.h3, parent, &fonts);
                header_icon(None, parent, &asset_server);
            });
        }
    }
}


pub fn header_title(
    title: &str, 
    font_size: f32, 
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
){
    let colors = Display::new();

    parent.spawn((
        Text::new("Wallet"),
        TextFont {
            font: fonts.style.heading.clone(),
            font_size,
            ..default()
        },
        TextColor(colors.text_heading),
    ));
}

pub fn header_icon(
    icon: Option<Icon>, 
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
){
    let colors = Display::new();

    parent.spawn((
        if let Some(icon) = icon { Icon::new(icon, asset_server); },
        Node {
            height: Val::Px(32.0),
            width: Val::Px(32.0),
            ..default()
        },
    ));
}
