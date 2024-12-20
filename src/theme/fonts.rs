#![allow(unused)]
use bevy::prelude::*;
use bevy_ui::prelude::*;

#[derive(Resource)]
pub struct FontResources {
    pub style: Style,
    pub size: FontSizes,
}

#[derive(Resource)]
pub struct Style {
    pub heading: Handle<Font>,
    pub text: Handle<Font>,
    pub label: Handle<Font>,
}

impl Default for Style {
    fn default() -> Style {
        Style {
            heading: Handle::default(),
            text: Handle::default(),
            label: Handle::default(),
        }
    }
}

#[derive(Resource)]
pub struct FontSizes {
    pub title: f32,
    pub h1: f32,
    pub h2: f32,
    pub h3: f32,
    pub h4: f32,
    pub h5: f32,
    pub h6: f32,
    pub xl: f32,
    pub lg: f32,
    pub md: f32,
    pub sm: f32,
    pub xs: f32,
}

impl Default for FontSizes {
    fn default() -> FontSizes {
        FontSizes {
            title: 72.0,
            h1: 48.0,
            h2: 32.0,
            h3: 24.0,
            h4: 20.0,
            h5: 16.0,
            h6: 14.0,
            xl: 24.0,
            lg: 20.0,
            md: 16.0,
            sm: 14.0,
            xs: 12.0,
        }
    }
}

pub fn setup_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let outfit_bold = asset_server.load("fonts/Outfit-Bold.ttf");
    let outfit_regular = asset_server.load("fonts/Outfit-Regular.ttf");

    commands.insert_resource(FontResources {
        style: Style {
            heading: outfit_bold.clone(),
            text: outfit_regular.clone(),
            label: outfit_bold.clone(),
        },
        size: FontSizes::default(),
    });
}