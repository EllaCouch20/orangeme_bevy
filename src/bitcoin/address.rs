use bevy::prelude::*;

use crate::Page;
use crate::Theme;

use crate::components::{
    interfaces::Header,
    interfaces::Bumper,
    interfaces::Interface,
    text_input::text_input,
    navigator::sidebar_navigator,
    tip_button::tip_buttons,
    tip_button::Tip,
};

use bevy_simple_text_input::TextInputValue;
use crate::ButtonStyle;
use crate::InteractiveState;
use crate::components::amount_display::AmountDisplayUsd;
use crate::theme::color::ButtonColor;

#[derive(Component)]
pub struct OnAddressScreen;

pub fn address_setup(
    mut commands: Commands, 
    theme: Res<Theme>,
) {

    let bumper = Bumper::new();
    let interface = Interface::new();
    let header = Header::new();

    commands.spawn((
        interface.node,
        OnAddressScreen,
    ))
    .with_children(|parent| {
        sidebar_navigator(parent, &theme, 0);

        parent.spawn(interface.page_node).with_children(|parent| {
            header.stack_header(parent, &theme, Some(theme.icons.left()), "Bitcoin address", Page::Home);

            parent.spawn((interface.content, Interaction::None)).with_children(|parent| { 
                text_input(parent, &theme, "Bitcoin address...");
                tip_buttons(parent, &theme, vec![
                    ("Paste Clipboard", theme.icons.paste(), Tip::PasteClipboard), 
                    ("Scan QR Code", theme.icons.qr_code(), Tip::ScanQRCode),
                    ("Select Contact", theme.icons.profile(), Tip::SelectContact),
                ]);
            });

            bumper.button_bumper(parent, &theme, vec![
                ("Continue", Page::Amount, false)
            ]);
        });
    });
}

// ===== Button Color Handler ===== //

// pub fn address_tip_system(
//     mut interaction_query: Query<
//         (
//             Entity,
//             &Interaction,
//             &Parent,
//             &mut BackgroundColor,
//             &mut BorderColor,
//             Option<&ButtonStyle>,
//             &mut InteractiveState,
//         ),
//         (With<Button>, Changed<Interaction>),
//     >,
//     tip_query: Query<&Tip>,
//     address_screen_query: Query<(), With<OnAddressScreen>>,
// ) {
//     if address_screen_query.is_empty() {
//         return;
//     }

//     for (entity, interaction, parent, mut color, mut border_color, button_style, mut state) in &mut interaction_query {
//         println!("INTERACTION: {:?}", *interaction);
//         if *interaction == Interaction::Pressed {
//             let tip = match tip_query.get(parent.get()) {
//                 Ok(tip) => tip,
//                 Err(err) => {
//                     eprintln!("Failed to get tip for parent entity {:?}: {:?}", parent.get(), err);
//                     continue;
//                 }
//             };
    
//             println!("TIP: {:?}", tip);
    
//             // Handle different tip types
//             match tip {
//                 Tip::PasteClipboard => {
//                     if let Err(err) = handle_paste_clipboard() {
//                         eprintln!("Failed to handle PasteClipboard tip: {:?}", err);
//                     }
//                 }
//                 Tip::ScanQRCode => {
//                     println!("TIP is: ScanQRCode");
//                 }
//                 Tip::SelectContact => {
//                     println!("TIP is: SelectContact");
//                 }
//             }
//         }
//     }
// }

// fn handle_paste_clipboard() -> Result<(), Box<dyn std::error::Error>> {
//     let mut clipboard = Clipboard::new()?;
//     let contents = clipboard.get_text()?;
//     println!("Clipboard contains: {}", contents);
//     Ok(())
// }

pub fn address_input_system(
    mut param_set: ParamSet<(
        Query<(Entity, &TextInputValue), Changed<TextInputValue>>,
        Query<(
            Option<&ButtonStyle>,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut InteractiveState,
            &Children,
        ), With<Button>>,
    )>,
    mut text_query: Query<(&mut TextColor, &mut Text), Without<AmountDisplayUsd>>,
    address_screen_query: Query<(), With<OnAddressScreen>>,
    theme: Res<Theme>,
) {
    let is_address_screen = !address_screen_query.is_empty();

    let mut input_values: Vec<(Entity, String)> = Vec::new();
    for (entity, input_value) in param_set.p0().iter() {
        input_values.push((entity, input_value.0.clone()));
    }

    for (button_style, mut color, mut border_color, mut state, children) in param_set.p1().iter_mut() {
        let is_continue_button = children.iter().any(|&child| {
            if let Ok(text) = text_query.get(child) {
                text.1.0 == "Continue"
            } else {
                false
            }
        });

        if is_continue_button {
            *state = if is_address_screen && input_values.iter().any(|(_, value)| value.is_empty()) {
                InteractiveState::Disabled
            } else {
                InteractiveState::Default
            };

            if let Some(button_style) = button_style {
                let button_colors = ButtonColor::new(*button_style, *state);
                *color = button_colors.background.into();
                *border_color = button_colors.outline.into();

                for &child in children.iter() {
                    if let Ok((mut text_color, _)) = text_query.get_mut(child) {
                        *text_color = button_colors.label.into();
                    }
                }
            }
        }
    }
}
