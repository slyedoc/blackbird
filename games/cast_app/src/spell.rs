use bevy::{prelude::*, utils::HashMap};
use bevy_hanabi::prelude::*;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpellEffects>();
    }
}

#[derive(Resource)]
pub struct SpellEffects {
    pub hashmap: HashMap<Spell, Handle<EffectAsset>>,
}

impl FromWorld for SpellEffects {
    fn from_world(world: &mut World) -> Self {
        let mut effects = world.resource_mut::<Assets<EffectAsset>>();

        let mut hashmap = HashMap::default();
        hashmap.insert(Spell::FrostBolt, effects.add(Spell::FrostBolt.effect()));
        hashmap.insert(
            Spell::ArcaneExplosion,
            effects.add(Spell::ArcaneExplosion.effect()),
        );
        hashmap.insert(Spell::Blizzard, effects.add(Spell::Blizzard.effect()));
        Self { hashmap }
    }
}

#[derive(Component, Clone, PartialEq, Eq, Hash, Debug)]
pub enum Spell {
    FrostBolt,
    ArcaneExplosion,
    Blizzard,
}

impl Spell {
    pub fn effect(&self) -> EffectAsset {
        let mut color_gradient1 = Gradient::new();
        color_gradient1.add_key(0.0, Vec4::splat(1.0));
        color_gradient1.add_key(0.1, Vec4::new(1.0, 1.0, 0.0, 1.0));
        color_gradient1.add_key(0.4, Vec4::new(1.0, 0.0, 0.0, 1.0));
        color_gradient1.add_key(1.0, Vec4::splat(0.0));

        let mut size_gradient1 = Gradient::new();
        size_gradient1.add_key(0.0, Vec3::splat(0.1));
        size_gradient1.add_key(0.5, Vec3::splat(0.5));
        size_gradient1.add_key(0.8, Vec3::splat(0.08));
        size_gradient1.add_key(1.0, Vec3::splat(0.0));

        let writer1 = ExprWriter::new();

        let age1 = writer1.lit(0.).expr();
        let init_age1 = SetAttributeModifier::new(Attribute::AGE, age1);

        let lifetime1 = writer1.lit(5.).expr();
        let init_lifetime1 = SetAttributeModifier::new(Attribute::LIFETIME, lifetime1);

        // Add constant downward acceleration to simulate gravity
        let accel1 = writer1.lit(Vec3::Y * -3.).expr();
        let update_accel1 = AccelModifier::new(accel1);

        let init_pos1 = SetPositionCone3dModifier {
            base_radius: writer1.lit(0.).expr(),
            top_radius: writer1.lit(2.).expr(),
            height: writer1.lit(2.).expr(),
            dimension: ShapeDimension::Volume,
        };

        let init_vel1 = SetVelocitySphereModifier {
            center: writer1.lit(Vec3::ZERO).expr(),
            speed: writer1.lit(1.).expr(),
        };

        EffectAsset::new(32768, Spawner::rate(500.0.into()), writer1.finish())
            .with_name("emit:rate")
            .init(init_pos1)
            // Make spawned particles move away from the emitter origin
            .init(init_vel1)
            .init(init_age1)
            .init(init_lifetime1)
            .update(update_accel1)
            .render(ColorOverLifetimeModifier {
                gradient: color_gradient1,
            })
            .render(SizeOverLifetimeModifier {
                gradient: size_gradient1,
                screen_space_size: false,
            })
            .render(OrientModifier::new(OrientMode::FaceCameraPosition))
    }
}
