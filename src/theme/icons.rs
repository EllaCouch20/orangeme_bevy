#![allow(unused)]
use bevy::prelude::*;
use bevy_ui::prelude::*;

#[derive(Component, Copy, Clone)]
pub enum Icon {
    Exit,
    Left,
    Right,
    Wallet,
    Message,
}

impl Icon {
    pub fn new(self, asset_server: &Res<AssetServer>) -> ImageNode {
        let choice = match self {
            Icon::Exit => "exit",
            Icon::Left => "left",
            Icon::Right => "right",
            Icon::Wallet => "wallet",
            Icon::Message => "message",
        };
        let img = format!("icons/{}.png", choice);
        ImageNode::new(asset_server.load(img.as_str()))
    }
}
