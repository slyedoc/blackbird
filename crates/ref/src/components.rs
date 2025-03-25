use bevy::{prelude::*, state::commands};
use bevy_mod_outline::{OutlineVolume, OutlineMode};
use leafwing_input_manager::prelude::ActionState;

use crate::{Action, Selected};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(OutlineVolume(|| OutlineVolume {
    visible: false,
    colour: Color::srgb(1.0, 1.0, 1.0),
    width: 5.0,
}))]
#[require(OutlineMode(|| OutlineMode::FloodFlat))]
pub struct RefImage {
    pub path: String,
}


pub fn on_add_ref_image(
    mut commands: Commands,
    added_query: Query<Entity, Added<RefImage>>,
) {
    for entity in added_query.iter() {
        info!("Added RefImage: {:?}", entity);
        commands.entity(entity)
            .observe(on_drag)
            .observe(on_select);
    }
}


fn on_drag(drag: Trigger<Pointer<Drag>>, mut transforms: Query<&mut Transform>) {
    info!("Drag event: {:?}", drag);
    if let Ok(mut transform) = transforms.get_mut(drag.target) {
        transform.translation.x += drag.delta.x * 0.02;
        transform.translation.y -= drag.delta.y * 0.02;
    }
}

// remove the outline other image
fn on_select(
    target: Trigger<Pointer<Click>>,     
    mut commands: Commands,
    actions: Res<ActionState<Action>>,
    mut query: Query<(Entity, Option<&Selected>), With<RefImage>>) {
    for (e, selected) in query.iter_mut() {
        if e == target.target {
            if selected.is_none() {
                commands.entity(e).insert(Selected);
                info!("Selected: {:?}", e);
            }
            break;  
        }
         
        if !actions.pressed(&Action::SelectAll) {
            if selected.is_some()  {
                commands.entity(e).remove::<Selected>();
            }
        }
        
    }
}


