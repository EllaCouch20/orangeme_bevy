use bevy::prelude::*;
use crate::utils::EXPAND;
use crate::Theme;

// ===== Profile Photo ===== //

pub fn profile_photo(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    theme: &Res<Theme>,
    size: f32,
    path: &str,
){
    parent.spawn(Node {
        width: Val::Px(size),
        height: Val::Px(size),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            Node {
                width: EXPAND,
                height: EXPAND,
                ..default()
            },
            BorderColor(theme.colors.outline_primary),
            BorderRadius::MAX,
            ImageNode::new(asset_server.load(path))
        ));
    });  
}
