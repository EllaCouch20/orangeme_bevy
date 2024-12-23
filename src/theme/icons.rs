#![allow(unused)]
use bevy::prelude::*;
use bevy_ui::prelude::*;
use crate::NavigateTo;


// ===== Icon Options ===== //

#[derive(Component, Copy, Clone)]
pub enum Icon {
    Add,
    Back,
    Bitcoin,
    Checkmark,
    Close,
    Copy,
    Delete,
    Door,
    Down,
    Edit,
    Error,
    Explore,
    Forward,
    Group,
    Home,
    Info,
    Left,
    Link,
    Message,
    Minus,
    Monitor,
    Paste,
    Profile,
    Profiles,
    QrCode,
    RadioFilled,
    Radio,
    Right,
    Scan,
    Search,
    Send,
    Up,
    Wallet,
    Warning,
}

// ===== Icon To ImageNode ===== //

impl Icon {
    pub fn new(self, asset_server: &Res<AssetServer>) -> ImageNode {
        let choice = match self {
            Icon::Add => "add",
            Icon::Back => "back",
            Icon::Bitcoin => "bitcoin",
            Icon::Checkmark => "checkmark",
            Icon::Close => "close",
            Icon::Copy => "copy",
            Icon::Delete => "delete",
            Icon::Door => "door",
            Icon::Down => "down",
            Icon::Edit => "edit",
            Icon::Error => "error",
            Icon::Explore => "explore",
            Icon::Forward => "forward",
            Icon::Group => "group",
            Icon::Home => "home",
            Icon::Info => "info",
            Icon::Left => "left",
            Icon::Link => "link",
            Icon::Message => "message",
            Icon::Minus => "minus",
            Icon::Monitor => "monitor",
            Icon::Paste => "paste",
            Icon::Profile => "profile",
            Icon::Profiles => "profiles",
            Icon::QrCode => "qr_code",
            Icon::RadioFilled => "radio_filled",
            Icon::Radio => "radio",
            Icon::Right => "right",
            Icon::Scan => "scan",
            Icon::Search => "search",
            Icon::Send => "send",
            Icon::Up => "up",
            Icon::Wallet => "wallet",
            Icon::Warning => "warning",
        };
        let img = format!("icons/{}.png", choice);
        ImageNode::new(asset_server.load(img.as_str()))
    }
}

// ===== Icon Button ===== //

pub fn icon_button(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    icon: Icon,
    navigate_to: NavigateTo
) {
    parent.spawn((
        Button,
        navigate_to,
        Icon::new(icon, asset_server),
        Node {
            height: Val::Px(32.0),
            width: Val::Px(32.0),
            ..default()
        },
    ));
}