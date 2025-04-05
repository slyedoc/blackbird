use bevy::{    
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use crate::ui::*;

#[derive(Component)]
#[require(
    Button,
    Node = Node {        
        margin: UiRect::all(Val::Px(10.0)),
        padding: UiRect::all(Val::Px(4.0)),      
        width: Val::Px(200.),                
        // Add a border so we can show which element is focused
        //border: UiRect::all(Val::Px(4.0)),
        // Center the button's text label
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        // Center the button within the grid cell
        align_self: AlignSelf::Center,
        justify_self: JustifySelf::Center,
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
#[component(on_add = on_add_button)]
pub struct MenuButton;

fn on_add_button(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
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
        width: Val::Percent(100.),
        // height: Val::Px(65.0),
        ..default()
    },
    TextColor = TextColor(Color::srgb(0.9, 0.9, 0.9)),
    TextShadow = TextShadow::default(),
    TextLayout = TextLayout{
        justify: JustifyText::Center,
        ..default()
    },
)]

#[component(on_add = on_add_button_inner)]
pub struct MenuButtonInner;


fn on_add_button_inner(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    let font = world.resource::<UiAssets>().font.clone();
    world
        .commands()
        .entity(entity)
        .insert((
            TextFont {
                font,
                font_size: 33.0,
                ..default()
            },
        ));
    //  .insert(
    //      BackgroundColor(normal_button),
    //  )
}