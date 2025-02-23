use bevy::prelude::*;
use text_input::ui_text_input_focus;
pub mod text_input;
//////////
// BUTTON

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn ui_button_default(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>, Without<Checkbox>),
    >,
) {
    for (interaction, mut background_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON.into();
                border_color.0 = Color::linear_rgb(1.0, 0.0, 0.0);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

////////////
// CHECKBOX
#[derive(Component)]
#[require(Button)]
pub struct Checkbox {
    pub checked: bool,
}

fn ui_checkbox_default(
    mut query: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>, With<Checkbox>)>,
) {
    for (interaction, mut border_color) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {}
            Interaction::Hovered => {
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn ui_checkbox_checked(mut query: Query<(&mut BackgroundColor, &Checkbox), Changed<Checkbox>>) {
    for (mut background_color, checkbox) in query.iter_mut() {
        *background_color = if checkbox.checked {
            Color::linear_rgb(0., 1., 0.).into()
        } else {
            NORMAL_BUTTON.into()
        };
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                ui_button_default,
                ui_checkbox_default,
                ui_checkbox_checked,
                ui_text_input_focus,
            ),
        );
    }
}
