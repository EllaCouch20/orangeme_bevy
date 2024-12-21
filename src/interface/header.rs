use bevy::prelude::*;
use crate::primitives::profile_photo::profile_photo;
use crate::NavigateTo;
use crate::theme::{ color::Display, fonts::FontResources, icons::{Icon, icon_button} };

// ===== Header ===== //

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

    // ===== Different Header Types ===== //

    match header_type {
        Header::Home => {
            parent.spawn(header_node).with_children(|parent| { 
                header_icon(None, parent, asset_server);
                // profile_photo(parent, fonts, asset_server, "profile_photo.png");  // Mobile Only
                header_title(title, fonts.size.h3, parent, fonts);
                header_icon(None, parent, asset_server);
            });
        },
        Header::Stack => {
            parent.spawn(header_node).with_children(|parent| { 
                header_icon(Some(Icon::Left), parent, asset_server);
                header_title(title, fonts.size.h4, parent, fonts);
                header_icon(None, parent, asset_server);
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
    let font = fonts.style.heading.clone();

    parent.spawn((
        Text::new(title),
        TextFont {
            font,
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
    if let Some(icon) = icon { 
        icon_button(parent, asset_server, icon, NavigateTo::Home);
    } else {
        parent.spawn((
            Node {
                height: Val::Px(32.0),
                width: Val::Px(32.0),
                ..default()
            },
        ));
    }
}