use bevy::{prelude::*, ui::FocusPolicy};
use bevy_simple_text_input::*;

const BORDER_COLOR_ACTIVE: Color = Color::srgb(0.9, 0.9, 0.9);
const BORDER_COLOR_INACTIVE: Color = Color::srgb(0.25, 0.25, 0.25);
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const TEXT_COLOR_PLACEHOLDER: Color = Color::srgb(0.5, 0.5, 0.5);
const BACKGROUND_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

pub fn ui_text_input_focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = BORDER_COLOR_ACTIVE.into();
                } else {
                    inactive.0 = true;
                    *border_color = BORDER_COLOR_INACTIVE.into();
                }
            }
        }
    }
}

pub fn create_text_input<C: Component>(
    commands: &mut Commands,
    marker: C,
    placeholder: String,
    default_value: Option<String>,
) -> Entity {
    let mut entity = commands.spawn((
        marker,
        BorderColor(BORDER_COLOR_INACTIVE),
        BackgroundColor(BACKGROUND_COLOR),
        FocusPolicy::Block,
        Node {
            width: Val::Px(200.0),
            height: Val::Px(50.0),
            border: UiRect::all(Val::Px(5.0)),
            padding: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        TextInput,
        TextInputTextFont(TextFont {
            font_size: 20.,
            ..default()
        }),
        TextInputTextColor(TextColor(TEXT_COLOR)),
        TextInputSettings {
            retain_on_submit: true,
            ..default()
        },
        TextInputPlaceholder {
            value: placeholder,
            text_color: Some(TextColor(TEXT_COLOR_PLACEHOLDER)),
            text_font: Some(TextFont {
                font_size: 20.,
                ..default()
            }),
        },
        TextInputInactive(true),
    ));

    if let Some(default_value) = default_value {
        entity.insert(TextInputValue(default_value));
    }

    entity.id()
}
