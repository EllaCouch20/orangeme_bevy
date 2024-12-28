use bevy::prelude::*;

use crate::Theme;
use crate::Page;

use crate::utils::text;
use crate::utils::EXPAND;
use crate::utils::MAX;

use crate::theme::icons::icon_button;
use crate::components::{
    button_presets::primary_disabled,
    button_presets::primary_default,
    button_presets::secondary_wide,
};


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
        theme: &Res<Theme>,
        icon: Option<ImageNode>,
        title: &str,
        nav: Page,
    ){
        parent.spawn(self.stack_node).with_children(|parent| { 
            header_icon(icon, parent, nav);
            header_title(title, theme.fonts.size.h4, parent, theme);
            placeholder(parent);
        });
    }

    pub fn home_header(
        self,
        parent: &mut ChildBuilder,
        theme: &Res<Theme>,
        title: &str,
    ){
        parent.spawn(self.home_node).with_children(|parent| { 
            // profile_photo(parent, fonts, asset_server, "profile_photo.png");  // Mobile Only
            header_title(title, theme.fonts.size.h3, parent, theme);
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
    parent: &mut ChildBuilder,
    theme: &Res<Theme>,
){
    let font = theme.fonts.style.heading.clone();

    parent.spawn(
        text(title, font, font_size, theme.colors.text_heading),
    );
}

pub fn header_icon(
    icon: Option<ImageNode>,
    parent: &mut ChildBuilder,
    nav: Page,
){

    parent.spawn((Node::default(), nav))
    .with_children(|parent| {
        if let Some(icon) = icon { 
            icon_button(parent, icon);
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

// ===== Bumper Instantiation ===== //

pub struct Bumper {
    bumper_content_node: Node,
    bumper_node: Node,
    bttn: Node,
}

impl Default for Bumper {
    fn default() -> Self {
        Self::new()
    }
}

impl Bumper {
    pub fn new() -> Self {
        Self {
            bumper_content_node: Node {
                width: EXPAND,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(8.0),
                padding: UiRect {
                    top: Val::Px(16.0),
                    bottom: Val::Px(16.0),
                    left: Val::Px(24.0),
                    right: Val::Px(24.0)
                },
                ..default()
            },
            bumper_node: Node {
                width: EXPAND,
                max_width: MAX,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Center,
                ..default()
            },
            bttn: Node {
                width: EXPAND,
                ..default()
            },
        }
    }

    // ===== Bumper Unlimited Buttons ===== //

    pub fn button_bumper(
        self,
        parent: &mut ChildBuilder,
        theme: &Res<Theme>,
        buttons: Vec<(&str, Page, bool)>,
    ) {
        parent.spawn(self.bumper_node).with_children(|parent| {
            parent.spawn(self.bumper_content_node).with_children(|parent| {
                for (name, navigate_to, enabled) in buttons {
                    parent.spawn(self.bttn.clone()).with_children(|parent|{
                        if enabled {
                            primary_default(name).create_on(parent, navigate_to, &theme);
                        } else {
                            primary_disabled(name).create_on(parent, navigate_to, &theme);
                        }
                    });
                }
            });
        });
    }

    pub fn secondary_button_bumper(
        self,
        parent: &mut ChildBuilder,
        theme: &Res<Theme>,
        data: (&str, Page),
    ) {
        parent.spawn(self.bumper_node).with_children(|parent| {
            parent.spawn(self.bumper_content_node).with_children(|parent| {
                secondary_wide(data.0).create_on(parent, data.1, &theme);
            });
        });
    }
}

pub struct Interface {
    pub node: Node,
    pub page_node: Node,
    pub content: Node,
    pub content_centered: Node,
}

impl Default for Interface {
    fn default() -> Self {
        Self::new()
    }
}

// ===== Commonplace Interface Nodes ===== //

impl Interface {
    pub fn new() -> Self {
        Self {
            node: Node {
                width: EXPAND,
                height: EXPAND,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            page_node: Node {
                width: EXPAND,
                height: EXPAND,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            content: Node {
                width: EXPAND,
                height: EXPAND,
                max_width: Val::Px(512.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(24.0),
                padding: UiRect {
                    left: Val::Px(24.0),
                    right: Val::Px(24.0),
                    top: Val::Px(16.0),
                    bottom: Val::Px(16.0),
                },
                ..default()
            },
            content_centered: Node {
                width: EXPAND,
                height: EXPAND,
                max_width: Val::Px(512.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(24.0),
                ..default()
            },
        }
    }
}
