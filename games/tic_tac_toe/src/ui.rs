use bevy::{color::palettes::tailwind::*, prelude::*};

pub const PANEL_BACKGROUND: Color = Color::Srgba(GRAY_900);
pub const BUTTON_BORDER: Srgba = RED_600;
pub const BUTTON_TEXT: Color = Color::Srgba(GRAY_100);
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                //**text = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED_500.into();
            }
            Interaction::Hovered => {
                //**text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                //**text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

#[derive(Component)]
#[require(
    BackgroundColor(|| BackgroundColor(PANEL_BACKGROUND))
)]
pub struct MenuPanel;

#[derive(Component)]
#[require(Button)]
#[require(Node(|| Node {
    padding: UiRect::all(Val::Px(10.0)),
    margin: UiRect::all(Val::Px(10.0)),
    width: Val::Percent(100.0),
    // horizontally center child text
    flex_direction: FlexDirection::Column,
    justify_content: JustifyContent::Center,
    // vertically center child text
    align_items: AlignItems::Center,
    ..default()
}))]
#[require(BackgroundColor( || BackgroundColor(NORMAL_BUTTON)))]
pub struct MenuButton;

#[derive(Component)]
#[require(TextFont(|| TextFont {
    font_size: 33.0,
    ..Default::default()
}))]
#[require(TextColor(|| TextColor(BUTTON_TEXT)))]
pub struct MenuButtonText;

/// Returns an observer that updates the entity's material to the one specified.
pub fn update_material_on<E>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(Trigger<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |trigger, mut query| {
        if let Ok(mut material) = query.get_mut(trigger.entity()) {
            material.0 = new_material.clone();
        }
    }
}
