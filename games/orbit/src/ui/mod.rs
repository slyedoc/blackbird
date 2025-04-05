mod quick;
pub use quick::*;
mod menu;
pub use menu::*;


use bevy::{
    color::palettes::tailwind,
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiAssets>();
    }
}



pub const NORMAL_BUTTON: Color = Color::Srgba(tailwind::SLATE_500);
pub const NORMAL_BUTTON_BORDER: Color = Color::Srgba(tailwind::SLATE_600);
//const NORMAL_BUTTON_TEXT: Color = Color::Srgba(tailwind::SLATE_100);
pub const HOVERED_BUTTON: Color = Color::Srgba(tailwind::SLATE_600);
pub const HOVERED_BUTTON_BORDER: Color = Color::Srgba(tailwind::SLATE_700);
pub const PRESSED_BUTTON: Color = Color::Srgba(tailwind::SLATE_700);
pub const PRESSED_BUTTON_BORDER: Color = Color::Srgba(tailwind::SLATE_800);
pub const PANEL_BACKGROUND: Color = Color::Srgba(tailwind::GRAY_900);
pub const PANEL_BORDER: Color = Color::Srgba(tailwind::GRAY_800);

#[derive(Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,    
}

impl FromWorld for UiAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        
        let font = asset_server.load("fonts/FiraMono-Medium.ttf");

        UiAssets { font }
    }
}

fn update_colors_on<E>(
    background: Color,
    outline: Color,
) -> impl Fn(Trigger<E>, Query<(&mut BackgroundColor, &mut Outline)>) {
    // An observer closure that captures `new_material`. We do this to avoid needing to write four
    // versions of this observer, each triggered by a different event and with a different hardcoded
    // material. Instead, the event type is a generic, and the material is passed in.
    move |trigger, mut query| {
        if let Ok((mut bg, mut out)) = query.get_mut(trigger.target()) {
            bg.0 = background;
            out.color = outline;
        }
    }
}


#[derive(Component)]
#[component(on_add = on_add_h1)]
pub struct H1;

fn on_add_h1(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    let font = world.resource::<UiAssets>().font.clone();
    world
        .commands()
        .entity(entity)
        .insert(
            TextFont {
                font,
                font_size: 40.0,
                ..default()
            },
        );
}