use bevy::prelude::*;
use bevy_mod_outline::OutlineVolume;

use crate::Prefab;



#[derive(Debug, Resource, Deref, DerefMut)]
pub struct CurrentSelected(pub Option<Entity>);

#[derive(Debug, Component, Reflect, Default)]
//#[component(on_remove = on_add_ray_caster)]
#[reflect(Component)]
pub struct Selected;

// fn on_add_ray_caster(mut world: DeferredWorld, ctx: HookContext) {
//     // let ray_caster = world.get::<RayCaster>(ctx.entity).unwrap();
//     // let max_hits = if ray_caster.max_hits == u32::MAX {
//     //     10
//     // } else {
//     //     ray_caster.max_hits as usize
//     // };

//     // // Initialize capacity for hits
//     // world.get_mut::<RayHits>(ctx.entity).unwrap().vector = Vec::with_capacity(max_hits);
// }

pub fn on_update_select(
    mut selected_query: Query<Entity, (Added<Selected>, With<Prefab>)>,    
    mut removed: RemovedComponents<Selected>,
    mut images: Query<&mut OutlineVolume>,  
) {
    for e  in selected_query.iter_mut() {
        if let Ok(mut outline) = images.get_mut(e) {
            outline.visible = true;
        }        
    }

    removed.read().for_each(|removed_entity| {
        if let Ok(mut outline) = images.get_mut(removed_entity) {
            outline.visible = false;
        }
    });
}


