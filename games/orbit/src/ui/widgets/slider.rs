use crate::ui::*;
use bevy::{
    ecs::{component::HookContext, world::DeferredWorld},
    prelude::*,
};
use bevy_inspector_egui::InspectorOptions;

/// Slider track widget
#[derive(Component, InspectorOptions)]
#[require(
    Node = Node {
        width: Val::Px(300.0),
        height: Val::Px(20.0),
        margin: UiRect::all(Val::Px(10.0)),
        padding: UiRect::all(Val::Px(4.0)),
        ..default()
    },
    BackgroundColor(SLIDER_TRACK_COLOR),
)]
#[component(on_add = on_add_slider)]
/// Component to store the normalized slider value (0.0 to 1.0)
pub struct Slider {
    #[inspector(min = 0.0, max = 1.0)]
    pub value: f32,
}

#[derive(Event, Reflect, Clone, Debug)]
pub struct SliderChanged {
    pub value: f32,
}

/// Called when a MenuSlider is added to the world.
fn on_add_slider(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    // Setup observers for pointer-over and pointer-out events to change track color.
    world
        .commands()
        .entity(entity)
        .observe(update_colors_on::<Pointer<Over>>(
            HOVERED_SLIDER_TRACK,
            HOVERED_SLIDER_TRACK,
        ))
        .observe(update_colors_on::<Pointer<Out>>(
            SLIDER_TRACK_COLOR,
            SLIDER_TRACK_COLOR,
        ));

    // Create the slider thumb (draggable part) and attach it as a child.
    world.commands().spawn((SliderThumb, ChildOf(entity)));
}

/// Slider thumb widget
#[derive(Component)]
#[require(
    Node = Node {
        width: Val::Px(30.0),
        height: Val::Px(30.0),
        ..default()
    },
    BackgroundColor(SLIDER_THUMB_COLOR),
)]
#[component(on_add = on_add_slider_thumb)]
pub struct SliderThumb;

/// Called when a MenuSliderThumb is added to the world.
fn on_add_slider_thumb(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    // Setup observers to update the thumb’s appearance based on pointer events.
    world
        .commands()
        .entity(entity)
        .observe(update_colors_on::<Pointer<Over>>(
            HOVERED_SLIDER_THUMB,
            HOVERED_SLIDER_THUMB,
        ))
        .observe(update_colors_on::<Pointer<Out>>(
            SLIDER_THUMB_COLOR,
            SLIDER_THUMB_COLOR,
        ))
        .observe(update_colors_on::<Pointer<Pressed>>(
            SLIDER_THUMB_COLOR,
            SLIDER_THUMB_COLOR,
        ))
        .observe(update_colors_on::<Pointer<Released>>(
            HOVERED_SLIDER_THUMB,
            HOVERED_SLIDER_THUMB,
        ))
        .observe(update_slider_value_on_drag);
}

fn update_slider_value_on_drag(
    drag: Trigger<Pointer<Drag>>,
    mut thumb_styles: Query<(&mut Node, &ChildOf), Without<Slider>>,
    mut slider_query: Query<(&Node, &mut Slider)>,
    mut commands: Commands,
) {
    // The slider thumb entity that is being dragged.
    let thumb_entity = drag.target();

    // Try to retrieve the style component for the thumb.
    let (mut thumb_node, child_of) = thumb_styles.get_mut(thumb_entity).unwrap();
    // Retrieve the parent (slider track) of the thumb.
    let slider_entity = child_of.0;
    // Get the slider track's Node (layout) and SliderValue components.
    let (node, mut slider_value) = slider_query.get_mut(slider_entity).unwrap();
    // Extract the track width. We assume it is defined as Pixels.
    let track_width = if let Val::Px(w) = node.width {
        w
    } else {
        300.0
    };

    // Get the thumb's current horizontal offset.
    let current_left = match thumb_node.left {
        Val::Px(val) => val,
        _ => 0.0,
    };

    // Adjust the thumb's left offset using the drag delta.
    // Clamp the new offset so the thumb stays within the slider track.
    let new_left = (current_left + drag.delta.x).clamp(0.0, track_width);
    thumb_node.left = Val::Px(new_left);

    // Update the normalized slider value (0.0 - 1.0) based on thumb position.
    let new_value = new_left / track_width;

    if new_value != slider_value.value {
        // Emit a SliderChange event if the value has changed.

        slider_value.value = new_value;
        commands.trigger_targets(SliderChanged { value: new_value }, slider_entity);
    }
}
