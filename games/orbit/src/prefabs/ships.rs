use bevy::{ecs::{component::HookContext, world::DeferredWorld}, prelude::*};

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
#[require(
    Transform::default(),
)]
#[component(on_add = on_add_ship)]
pub enum Ship {
    PhalanxCorvette,
    PraetorGunship,
    RhodesBattlecruiser,
    TemplarFrigate,
}

fn on_add_ship(mut world: DeferredWorld<'_>, HookContext { entity, .. }: HookContext) {
    dbg!("Adding ship");
    let ship = &world.get::<Ship>(entity).unwrap();
    let file = match ship {
        Ship::PhalanxCorvette => "models/ships/PhalanxCorvette.glb",
        Ship::PraetorGunship => "models/shipsPraetorGunship.glb",
        Ship::RhodesBattlecruiser => "models/ships/RhodesBattlecruiser.glb",
        Ship::TemplarFrigate => "models/ships/TemplarFrigate.glb",            
    };
    let asset_server = world.resource::<AssetServer>();
    let scene =  asset_server.load(GltfAssetLabel::Scene(0).from_asset(file));    
    world
        .commands()
        .entity(entity)
        .insert( SceneRoot(scene));
}
