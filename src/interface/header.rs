use bevy::prelude::*;
use crate::primitives::profile_photo::profile_photo;

use crate::theme::{
    color::Display,
    fonts::FontResources,
    icons::Icon,
};

pub struct Header {
    title: String,
    font_size: f32,
    left: Option<Box<dyn FnOnce(&mut ChildBuilder)>>,
    right: Option<Box<dyn FnOnce(&mut ChildBuilder)>>,
}

impl Header {
    pub fn new(
        title: &str,
        font_size: f32,
        left: Option<Box<dyn FnOnce(&mut ChildBuilder)>>,
        right: Option<Box<dyn FnOnce(&mut ChildBuilder)>>,
    ) -> Self {
        Self {
            title: title.to_string(),
            font_size,
            left,
            right,
        }
    }

    pub fn header(
        &mut self,
        parent: &mut ChildBuilder,
        fonts: &Res<FontResources>,
        asset_server: &Res<AssetServer>,
    ) {
        let colors = Display::new();

        parent.spawn((Node {
            width: Val::Percent(100.0),
            height: Val::Px(88.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::SpaceBetween,
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(24.0)),
            ..default()
        })).with_children(|parent| { 
            let font = fonts.style.heading.clone();
            parent.spawn((
                if let Some(left) = &self.left { |parent| left(parent); },
                Node {
                    height: Val::Px(32.0),
                    width: Val::Px(32.0),
                    ..default()
                },
            )); 
            parent.spawn((
                Text::new("Wallet"),
                TextFont {
                    font,
                    font_size: self.font_size,
                    ..default()
                },
                TextColor(colors.text_heading),
            ));
            parent.spawn((
                if let Some(right) = &self.right { |parent| right(parent); },
                Node {
                    height: Val::Px(32.0),
                    width: Val::Px(32.0),
                    ..default()
                },
            )); 
        });
    }

    pub fn header_home(
        &mut self,
        parent: &mut ChildBuilder,
        fonts: &Res<FontResources>,
        asset_server: &Res<AssetServer>,
    ) {
        self.left = Some(Box::new(|parent: &mut ChildBuilder| {
            profile_photo(parent, &fonts, &asset_server, "profile_photo.png");
        }));
        self.right = None;
        self.font_size = fonts.size.h3; 
        self.header(parent, fonts, asset_server);
    }
}
