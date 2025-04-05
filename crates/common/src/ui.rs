use bevy::{color::palettes::tailwind, prelude::*};

pub struct UiPlugin;

#[derive(Resource, Debug)]
pub struct UiConfig {
    pub normal_button: Color,
    pub normal_button_border: Color,
    pub normal_button_text: Color,
    pub hovered_button: Color,
    pub hovered_button_border: Color,
    pub pressed_button: Color,
    pub pressed_button_border: Color,
    pub panel_background: Color,
    pub panel_border: Color,
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            normal_button: tailwind::SLATE_500.into(),
            normal_button_border: tailwind::SLATE_600.into(),
            normal_button_text: tailwind::SLATE_100.into(),
            hovered_button: tailwind::SLATE_600.into(),
            hovered_button_border: tailwind::SLATE_700.into(),
            pressed_button: tailwind::SLATE_700.into(),
            pressed_button_border: tailwind::SLATE_800.into(),
            panel_background: tailwind::GRAY_900.into(),
            panel_border: tailwind::GRAY_800.into(),
        }
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UiConfig::default());
        //.add_systems(Update, button_system);
    }
}

#[derive(Component)]

pub struct Panel;

// fn button_system(
//     mut interaction_query: Query<
//         (
//             &Interaction,
//             &mut BackgroundColor,
//             &mut BorderColor,
//             &Children,
//         ),
//         (Changed<Interaction>, With<Button>),
//     >,
//     mut text_query: Query<&mut Text>,
//     config: Res<UiConfig>,
// ) {
//     for (interaction, mut color, mut border_color, children) in &mut interaction_query {
//         let mut _text = text_query.get_mut(children[0]).unwrap();
//         match *interaction {
//             Interaction::Pressed => {
//                 //**text = "Press".to_string();
//                 *color = config.pressed_button.into();
//                 border_color.0 = config.hovered_button_border.into();
//             }
//             Interaction::Hovered => {
//                 //**text = "Hover".to_string();
//                 *color = config.hovered_button.into();
//                 border_color.0 = config.hovered_button_border.into();
//             }
//             Interaction::None => {
//                 //**text = "Button".to_string();
//                 *color = config.normal_button.into();
//                 border_color.0 = config.normal_button_border.into();
//             }
//         }
//     }
// }
