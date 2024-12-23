use bevy::prelude::*;
use crate::primitives::profile_photo::profile_photo;
use crate::utils::text;
use crate::Nav;
use crate::theme::{ color::Display, fonts::FontResources, icons::{Icon, icon_button} };

// ===== Header ===== //

pub struct Header {
    home_node: Node,
    stack_node: Node,
}

impl Default for Header {
    fn default() -> Self {
        Self::new()
    }
}

impl Header {
    pub fn new() -> Self {
        Self {
            home_node: Node {
                width: Val::Percent(100.0),
                height: Val::Px(88.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
            stack_node: Node {
                width: Val::Percent(100.0),
                height: Val::Px(88.0),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::SpaceBetween,
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(24.0)),
                ..default()
            },
        }
    }

    pub fn stack_header(
        self,
        parent: &mut ChildBuilder,
        fonts: &Res<FontResources>,
        asset_server: &Res<AssetServer>,
        colors: &Res<Display>,
        icon: Option<Icon>,
        title: &str,
        nav: Nav,
    ){
        parent.spawn(self.stack_node).with_children(|parent| { 
            header_icon(icon, parent, asset_server, nav);
            header_title(title, fonts.size.h4, colors, parent, fonts);
            placeholder(parent);
        });
    }

    pub fn home_header(
        self,
        parent: &mut ChildBuilder,
        fonts: &Res<FontResources>,
        asset_server: &Res<AssetServer>,
        colors: &Res<Display>,
        title: &str,
    ){
        parent.spawn(self.home_node).with_children(|parent| { 
            // profile_photo(parent, fonts, asset_server, "profile_photo.png");  // Mobile Only
            header_title(title, fonts.size.h3, colors, parent, fonts);
            // header_icon(Icon::Add, parent, asset_server, nav);
        });
    }
}

pub fn placeholder(parent: &mut ChildBuilder) {
    parent.spawn((
        Node {
            height: Val::Px(32.0),
            width: Val::Px(32.0),
            ..default()
        },
    ));
}

pub fn header_title(
    title: &str, 
    font_size: f32, 
    colors: &Res<Display>,
    parent: &mut ChildBuilder,
    fonts: &Res<FontResources>,
){
    let font = fonts.style.heading.clone();

    parent.spawn(
        text(title, font, font_size, colors.text_heading),
    );
}

pub fn header_icon(
    icon: Option<Icon>,
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    nav: Nav,
){
    let colors = Display::new();

    parent.spawn((Node::default(), nav))
    .with_children(|parent| {
        if let Some(icon) = icon { 
            icon_button(parent, asset_server, icon);
        } else {
            parent.spawn((
                Node {
                    height: Val::Px(32.0),
                    width: Val::Px(32.0),
                    ..default()
                },
            ));
        }
    });
}