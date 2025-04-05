use bevy::{    
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use crate::ui::*;

#[derive(Component)]
#[require(
    Button,
    Node = Node {
        padding: UiRect::all(Val::Px(2.0)),
        margin: UiRect::all(Val::Px(4.0)),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },
    BackgroundColor(NORMAL_BUTTON),
    //BorderColor(PANEL_BORDER),
    Outline = Outline {
        width: Val::Px(2.),
        color: NORMAL_BUTTON_BORDER,
        ..default()
    },
    BorderRadius::all(Val::Px(5.)),
)]
#[component(on_add = on_add_quick_button)]
pub struct QuickButton;

pub fn on_add_quick_button(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    world
        .commands()
        .entity(entity)
        .observe(update_colors_on::<Pointer<Over>>(
            HOVERED_BUTTON,
            HOVERED_BUTTON_BORDER,
        ))
        .observe(update_colors_on::<Pointer<Out>>(
            NORMAL_BUTTON,
            NORMAL_BUTTON_BORDER,
        ))
        .observe(update_colors_on::<Pointer<Pressed>>(
            PRESSED_BUTTON,
            PRESSED_BUTTON_BORDER,
        ))
        .observe(update_colors_on::<Pointer<Released>>(
            HOVERED_BUTTON,
            HOVERED_BUTTON_BORDER,
        ));
    //  .insert(
    //      BackgroundColor(normal_button),
    //  )
}

#[derive(Component)]
#[require(
    Node = Node {
        width: Val::Px(30.0),
        height: Val::Px(30.0),
        ..default()
    },
)]
pub struct QuickButtonInner;