#![allow(unused)]
use bevy::prelude::*;
use crate::InteractiveState;

pub struct IconResources {
    pub up: ImageNode,
    pub add: ImageNode,
    pub back: ImageNode,
    pub exit: ImageNode,
    pub left: ImageNode,
    pub save: ImageNode,
    pub file: ImageNode,
    pub copy: ImageNode,
    pub door: ImageNode,
    pub down: ImageNode,
    pub edit: ImageNode,
    pub send: ImageNode,
    pub info: ImageNode,
    pub home: ImageNode,
    pub link: ImageNode,
    pub scan: ImageNode,
    pub paste: ImageNode,
    pub right: ImageNode,
    pub group: ImageNode,
    pub minus: ImageNode,
    pub radio: ImageNode,
    pub close: ImageNode,
    pub error: ImageNode,
    pub search: ImageNode,
    pub delete: ImageNode,
    pub folder: ImageNode,
    pub wallet: ImageNode,
    pub profile: ImageNode,
    pub forward: ImageNode,
    pub explore: ImageNode,
    pub monitor: ImageNode,
    pub message: ImageNode,
    pub qr_code: ImageNode,
    pub warning: ImageNode,
    pub bitcoin: ImageNode,
    pub profiles: ImageNode,
    pub wordmark: ImageNode,
    pub checkmark: ImageNode,
    pub radio_filled: ImageNode,
}

impl IconResources {
    pub fn resolve(assets: &Res<AssetServer>, name: &str) -> ImageNode {
        ImageNode::new(assets.load(&format!("icons/{name}.png")))
    }

    pub fn new(assets: &Res<AssetServer>) -> Self {
        IconResources {
            up: Self::resolve(assets, "up"),
            add: Self::resolve(assets, "add"),
            back: Self::resolve(assets, "back"),
            exit: Self::resolve(assets, "exit"),
            left: Self::resolve(assets, "left"),
            save: Self::resolve(assets, "save"),
            file: Self::resolve(assets, "file"),
            copy: Self::resolve(assets, "copy"),
            door: Self::resolve(assets, "door"),
            down: Self::resolve(assets, "down"),
            edit: Self::resolve(assets, "edit"),
            send: Self::resolve(assets, "send"),
            info: Self::resolve(assets, "info"),
            home: Self::resolve(assets, "home"),
            link: Self::resolve(assets, "link"),
            scan: Self::resolve(assets, "scan"),
            paste: Self::resolve(assets, "paste"),
            right: Self::resolve(assets, "right"),
            group: Self::resolve(assets, "group"),
            minus: Self::resolve(assets, "minus"),
            radio: Self::resolve(assets, "radio"),
            close: Self::resolve(assets, "close"),
            error: Self::resolve(assets, "error"),
            search: Self::resolve(assets, "search"),
            delete: Self::resolve(assets, "delete"),
            folder: Self::resolve(assets, "folder"),
            wallet: Self::resolve(assets, "wallet"),
            bitcoin: Self::resolve(assets, "bitcoin"),
            profile: Self::resolve(assets, "profile"),
            forward: Self::resolve(assets, "forward"),
            explore: Self::resolve(assets, "explore"),
            monitor: Self::resolve(assets, "monitor"),
            message: Self::resolve(assets, "message"),
            qr_code: Self::resolve(assets, "qr_code"),
            warning: Self::resolve(assets, "warning"),
            profiles: Self::resolve(assets, "profiles"),
            wordmark: Self::resolve(assets, "wordmark"),
            checkmark: Self::resolve(assets, "checkmark"),
            radio_filled: Self::resolve(assets, "radio_filled"),
        }
    }

    pub fn up(&self) -> ImageNode { self.up.clone() }
    pub fn add(&self) -> ImageNode { self.add.clone() }
    pub fn back(&self) -> ImageNode { self.back.clone() }
    pub fn exit(&self) -> ImageNode { self.exit.clone() }
    pub fn left(&self) -> ImageNode { self.left.clone() }
    pub fn save(&self) -> ImageNode { self.save.clone() }
    pub fn file(&self) -> ImageNode { self.file.clone() }
    pub fn copy(&self) -> ImageNode { self.copy.clone() }
    pub fn door(&self) -> ImageNode { self.door.clone() }
    pub fn down(&self) -> ImageNode { self.down.clone() }
    pub fn edit(&self) -> ImageNode { self.edit.clone() }
    pub fn send(&self) -> ImageNode { self.send.clone() }
    pub fn info(&self) -> ImageNode { self.info.clone() }
    pub fn home(&self) -> ImageNode { self.home.clone() }
    pub fn link(&self) -> ImageNode { self.link.clone() }
    pub fn scan(&self) -> ImageNode { self.scan.clone() }
    pub fn paste(&self) -> ImageNode { self.paste.clone() }
    pub fn right(&self) -> ImageNode { self.right.clone() }
    pub fn group(&self) -> ImageNode { self.group.clone() }
    pub fn minus(&self) -> ImageNode { self.minus.clone() }
    pub fn radio(&self) -> ImageNode { self.radio.clone() }
    pub fn close(&self) -> ImageNode { self.close.clone() }
    pub fn error(&self) -> ImageNode { self.error.clone() }
    pub fn search(&self) -> ImageNode { self.search.clone() }
    pub fn delete(&self) -> ImageNode { self.delete.clone() }
    pub fn folder(&self) -> ImageNode { self.folder.clone() }
    pub fn wallet(&self) -> ImageNode { self.wallet.clone() }
    pub fn profile(&self) -> ImageNode { self.profile.clone() }
    pub fn forward(&self) -> ImageNode { self.forward.clone() }
    pub fn explore(&self) -> ImageNode { self.explore.clone() }
    pub fn monitor(&self) -> ImageNode { self.monitor.clone() }
    pub fn message(&self) -> ImageNode { self.message.clone() }
    pub fn qr_code(&self) -> ImageNode { self.qr_code.clone() }
    pub fn warning(&self) -> ImageNode { self.warning.clone() }
    pub fn bitcoin(&self) -> ImageNode { self.bitcoin.clone() }
    pub fn profiles(&self) -> ImageNode { self.profiles.clone() }
    pub fn wordmark(&self) -> ImageNode { self.wordmark.clone() }
    pub fn checkmark(&self) -> ImageNode { self.checkmark.clone() }
    pub fn radio_filled(&self) -> ImageNode { self.radio_filled.clone() }
}


// ===== Icon Button ===== //

pub fn icon_button(
    parent: &mut ChildBuilder,
    icon: ImageNode,
) {
    parent.spawn((
        Button,
        InteractiveState::Default,
        icon,
        Node {
            height: Val::Px(32.0),
            width: Val::Px(32.0),
            ..default()
        },
    ));
}