#![allow(unused)]
use bevy::prelude::*;
use bevy_ui::prelude::*;
use crate::NavigateTo;


// ===== Icon Options ===== //

#[derive(Component, Copy, Clone)]
pub enum Icon {
    Back, Bitcoin, Copy,
    Down, Edit, Error,
    Exit, File, Folder,
    Forward, Group, Home,
    Info, Left, Message,
    Paste, Profile, Scan,
    RadioFilled, Radio, 
    Right, Wallet,
}

// ===== Icon To ImageNode ===== //

impl Icon {
    pub fn new(self, asset_server: &Res<AssetServer>) -> ImageNode {
        let choice = match self {
            Icon::Copy => "copy", Icon::Down => "down",
            Icon::Exit => "exit", Icon::File => "file",
            Icon::Info => "info", Icon::Left => "left",
            Icon::Edit => "edit", Icon::Error => "error",
            Icon::Group => "group", Icon::Home => "home",
            Icon::Right => "right", Icon::Wallet => "wallet",
            Icon::Back => "back", Icon::Bitcoin => "bitcoin",
            Icon::Message => "message", Icon::Paste => "paste",
            Icon::Profile => "profile", Icon::Scan => "qr-code",
            Icon::Folder => "folder", Icon::Forward => "forward",
            Icon::RadioFilled => "radio-filled", Icon::Radio => "radio",
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