use bevy::{app::AppExit, color::palettes::css::CRIMSON, prelude::*};

use super::despawn_screen;
use bevy_ui::prelude::*;

use crate::primitives::button::{
    CustomButton, 
    ButtonWidth, 
    ButtonComponent, 
    ButtonSize, 
    InteractiveState, 
    ButtonStyle, 
    button_system,
    primary_default,
};

use crate::menu_plugin;

use crate::NavigateTo;
use crate::theme::icons::Icon;
use crate::theme::color::Display;
use crate::theme::fonts::{FontResources, FontSizes, Style, setup_fonts};
use crate::interface::header::{header, Header};
use crate::primitives::profile_photo::profile_photo;
use crate::interface::bumper::Bumper;

// ==== Components ==== //

use crate::components::{
    balance_display::balance_display,
    navigator::sidebar_navigator,
};


#[derive(Component)]
pub struct OnHomeScreen;

pub fn home_setup(mut commands: Commands, asset_server: Res<AssetServer>, fonts: Res<FontResources>) {

    let colors = Display::new();
    let bumper = Bumper::new();

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        OnHomeScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &fonts, &asset_server);
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            header(parent, &fonts, &asset_server, Header::Home, "Wallet");

            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    max_width: Val::Px(512.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
            )).with_children(|parent| {
                balance_display(parent, &fonts);
            });

            bumper.double_bumper(parent, &fonts, &asset_server);
        });
    });
}
