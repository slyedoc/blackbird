use core::f32;

use crate::states::AppState;
use bevy::{
    color::palettes::tailwind, ecs::relationship::RelatedSpawnerCommands, pbr::NotShadowCaster,
    prelude::*, render::view::NoFrustumCulling,
};
use bevy_inspector_egui::prelude::*;



pub struct SolarSystemPlugin;

impl Plugin for SolarSystemPlugin {
    fn build(&self, app: &mut App) {
        app        
        .add_systems(Update, (spawn_solor_system, orbit_system, rotation_system))
            .add_systems(PostUpdate, draw_orbit)
            .register_type::<SolarSystem>()
            .register_type::<CelestialOrbit>()
            .register_type::<CelestialRotation>();
    }
}


/// Component for orbit parameters (planet, moon, etc.)
#[derive(Component, Reflect, InspectorOptions)]
#[require(Transform, InheritedVisibility)]
#[reflect(Component, InspectorOptions)]
pub struct CelestialOrbit {
    pub orbit_radius: f32, // semi-major axis (a)
    #[inspector(min = 0.0, max = 1.0)]
    pub orbit_eccentricity: f32, // 0.0 for circular, up to <1.0 for elliptical
    pub orbit_speed: f32,  // angular speed (radians per second)
    pub orbit_angle: f32,  // current orbital angle (radians)
    pub orbit_inclination: f32, // tilt of the orbital plane (radians)
    pub orbit_node: f32,   // rotation of the orbital plane around Y (radians)
}

/// Component for self rotation (spin)
#[derive(Component, Reflect)]
pub struct CelestialRotation {
    pub rotation_speed: f32, // self rotation speed (radians per second)
    pub axial_tilt: f32,     // initial axial tilt (radians)
}

// uses to make lists
pub struct CelestialBody {
    pub name: String,
    pub radius: f32,             // radius of the celestial body in AU
    pub orbit_radius: f32,       // semi-major axis (a) in AU
    pub orbit_eccentricity: f32, // 0.0 for circular, up to <1.0 for elliptical
    pub orbit_speed: f32,        // angular speed (radians per second)
    pub orbit_angle: f32,        // current orbital angle (radians)
    pub orbit_inclination: f32,  // tilt of the orbital plane (radians)
    pub orbit_node: f32,         // rotation of the orbital plane around Y (radians)
    pub rotation_speed: f32,     // self rotation speed (radians per second)
    pub axial_tilt: f32,         // initial axial tilt (radians)
    pub color: Color,            // color of the celestial body
    pub children: Vec<CelestialBody>,
}

pub enum OrbitType {
    L5,
    Moon,
}

impl CelestialBody {
    pub fn build(
        &self,
        commands: &mut Commands,
        parent: Entity,
        config: &SolarSystem,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) -> impl Bundle {
        // mercury

        let e = commands.spawn((
            ChildOf { parent },
            Name::new(format!("{} - Pivot", self.name)),
            Transform::default(),
            CelestialOrbit {
                orbit_radius: self.orbit_radius * config.distance_scale,
                orbit_eccentricity: self.orbit_eccentricity,
                orbit_speed: self.orbit_speed,
                orbit_angle: self.orbit_angle,
                orbit_inclination: self.orbit_inclination,
                orbit_node: self.orbit_node,
            },
            children![(
                Name::new(self.name.clone()),
                Mesh3d(meshes.add(Mesh::from(Sphere {
                    radius: self.radius * config.planet_scale
                }))),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: self.color,
                    perceptual_roughness: 0.8,
                    reflectance: 1.0,
                    ..Default::default()
                })),
                CelestialRotation {
                    rotation_speed: self.rotation_speed,
                    axial_tilt: self.axial_tilt,
                },                
            )],
        )).id();

        for child in &self.children {
            child.build(commands, e, config, meshes, materials);
        }
    }
}
/// Returns a list of celestial bodies for the solar system using approximate realistic values.
///
/// ### Units and Notes
/// - **Distance and Radius:**  
///   Distances (orbit_radius) and radii are given in astronomical units (AU). Note that real planetary
///   radii are extremely small compared to their orbital distances, so you may need to scale these numbers for visualization.
/// - **Angular Speeds:**  
///   These are computed as 2π divided by the period in seconds (orbital or rotation period). Real periods are very long,
///   and so the resulting speeds are extremely small. In a simulation, you often scale time to observe noticeable movement.
///
/// ### Realistic Parameters (Approximate)
///
/// | Celestial Body | Semi‑Major Axis (AU) | Eccentricity | Rotation Period | Orbital Period  | Radius (AU)              | Inclination (deg) | Node (deg)   | Axial Tilt (deg) | Color               |
/// |----------------|----------------------|--------------|-----------------|-----------------|--------------------------|-------------------|--------------|------------------|---------------------|
/// | Mercury        | 0.387                | 0.2056       | 58.646 days     | 87.97 days      | ~2440 km ≈ 1.63e-5        | ~7.0              | ~48.33       | ~0.034           | tailwind::GRAY_500  |
/// | Venus          | 0.723                | 0.0068       | 243 days        | 224.7 days      | ~6052 km ≈ 4.04e-5        | ~3.39             | ~76.68       | ~177.36          | tailwind::YELLOW_500|
/// | Earth          | 1.000                | 0.0167       | 23.934 hrs      | 365.256 days    | ~6371 km ≈ 4.26e-5        | 0.0               | 0.0          | ~23.44           | tailwind::BLUE_500  |
/// | &nbsp;Moon     | 0.00257              | 0.0549       | 27.32166 days   | 27.32166 days   | ~1737 km ≈ 1.16e-5        | ~5.145            | ~125.08      | ~6.68            | tailwind::GRAY_500  |
/// | Mars           | 1.524                | 0.0934       | ~24.6229 hrs    | 687 days        | ~3389.5 km ≈ 2.27e-5      | ~1.85             | ~49.58       | ~25.19           | tailwind::RED_500   |
/// | Jupiter        | 5.204                | 0.0489       | ~9.925 hrs      | 4332.59 days    | ~69911 km ≈ 4.68e-4       | ~1.304            | ~100.464     | ~3.13            | tailwind::ORANGE_500|
/// | Saturn         | 9.583                | 0.0565       | ~10.7 hrs       | 10759 days      | ~58232 km ≈ 3.89e-4       | ~2.485            | ~113.665     | ~26.73           | tailwind::YELLOW_400|
/// | Uranus         | 19.191               | 0.0460       | ~17.24 hrs      | 30687 days      | ~25362 km ≈ 1.70e-4       | ~0.769            | ~74.006      | ~97.77           | tailwind::BLUE_300  |
/// | Neptune        | 30.07                | 0.0086       | ~16.11 hrs      | 60190 days      | ~24622 km ≈ 1.65e-4       | ~1.770            | ~131.784     | ~28.32           | tailwind::BLUE_800  |
pub fn planet_list() -> Vec<CelestialBody> {
    use std::f32::consts::PI;

    vec![
        // Mercury
        CelestialBody {
            name: "Mercury".to_string(),
            radius: 2440.0 / 149_600_000.0, // ≈ 1.63e-5 AU
            orbit_radius: 0.387,
            orbit_eccentricity: 0.2056,
            orbit_speed: 2.0 * PI / (87.97 * 86400.0),
            orbit_angle: 0.0,
            orbit_inclination: 7.0_f32.to_radians(),
            orbit_node: 48.33_f32.to_radians(),
            rotation_speed: 2.0 * PI / (58.646 * 86400.0),
            axial_tilt: 0.034_f32.to_radians(),
            color: Color::Srgba(tailwind::GRAY_500),
            children: vec![],
        },
        // Venus
        CelestialBody {
            name: "Venus".to_string(),
            radius: 6052.0 / 149_600_000.0, // ≈ 4.04e-5 AU
            orbit_radius: 0.723,
            orbit_eccentricity: 0.0068,
            orbit_speed: 2.0 * PI / (224.7 * 86400.0),
            orbit_angle: 0.0,
            orbit_inclination: 3.39_f32.to_radians(),
            orbit_node: 76.68_f32.to_radians(),
            // Retrograde rotation is represented by a negative angular speed.
            rotation_speed: -2.0 * PI / (243.0 * 86400.0),
            axial_tilt: 177.36_f32.to_radians(),
            color: Color::Srgba(tailwind::YELLOW_500),
            children: vec![],
        },
        // Earth
        CelestialBody {
            name: "Earth".to_string(),
            radius: 6371.0 / 149_600_000.0, // ≈ 4.26e-5 AU
            orbit_radius: 1.0,
            orbit_eccentricity: 0.0167,
            orbit_speed: 2.0 * PI / (365.256 * 86400.0),
            orbit_angle: 0.0,
            orbit_inclination: 0.0,
            orbit_node: 0.0,
            rotation_speed: 2.0 * PI / (23.934 * 3600.0),
            axial_tilt: 23.44_f32.to_radians(),
            color: Color::Srgba(tailwind::BLUE_500),
            children: vec![
                // Earth's Moon
                CelestialBody {
                    name: "Moon".to_string(),
                    radius: 1737.0 / 149_600_000.0, // ≈ 1.16e-5 AU
                    orbit_radius: 0.00257,
                    orbit_eccentricity: 0.0549,
                    orbit_speed: 2.0 * PI / (27.32166 * 86400.0),
                    orbit_angle: 0.0,
                    orbit_inclination: 5.145_f32.to_radians(),
                    orbit_node: 125.08_f32.to_radians(),
                    rotation_speed: 2.0 * PI / (27.32166 * 86400.0), // Synchronous rotation.
                    axial_tilt: 6.68_f32.to_radians(),
                    color: Color::Srgba(tailwind::GRAY_500),
                    children: vec![],
                },
            ],
        },
        // Mars
        CelestialBody {
            name: "Mars".to_string(),
            radius: 3389.5 / 149_600_000.0, // ≈ 2.27e-5 AU
            orbit_radius: 1.524,
            orbit_eccentricity: 0.0934,
            orbit_speed: 2.0 * PI / (687.0 * 86400.0),
            orbit_angle: 0.0,
            orbit_inclination: 1.85_f32.to_radians(),
            orbit_node: 49.58_f32.to_radians(),
            rotation_speed: 2.0 * PI / (24.6229 * 3600.0),
            axial_tilt: 25.19_f32.to_radians(),
            color: Color::Srgba(tailwind::RED_500),
            children: vec![],
        },
        // Jupiter
        CelestialBody {
            name: "Jupiter".to_string(),
            radius: 69911.0 / 149_600_000.0, // ≈ 4.68e-4 AU
            orbit_radius: 5.204,
            orbit_eccentricity: 0.0489,
            orbit_speed: 2.0 * PI / (4332.59 * 86400.0),
            orbit_angle: 0.0,
            orbit_inclination: 1.304_f32.to_radians(),
            orbit_node: 100.464_f32.to_radians(),
            rotation_speed: 2.0 * PI / (9.925 * 3600.0),
            axial_tilt: 3.13_f32.to_radians(),
            color: Color::Srgba(tailwind::ORANGE_500),
            children: vec![],
        },
        // Saturn
        CelestialBody {
            name: "Saturn".to_string(),
            radius: 58232.0 / 149_600_000.0, // ≈ 3.89e-4 AU
            orbit_radius: 9.583,
            orbit_eccentricity: 0.0565,
            orbit_speed: 2.0 * PI / (10759.0 * 86400.0),
            orbit_angle: 0.0,
            orbit_inclination: 2.485_f32.to_radians(),
            orbit_node: 113.665_f32.to_radians(),
            rotation_speed: 2.0 * PI / (10.7 * 3600.0),
            axial_tilt: 26.73_f32.to_radians(),
            color: Color::Srgba(tailwind::YELLOW_400),
            children: vec![],
        },
        // Uranus
        CelestialBody {
            name: "Uranus".to_string(),
            radius: 25362.0 / 149_600_000.0, // ≈ 1.70e-4 AU
            orbit_radius: 19.191,
            orbit_eccentricity: 0.0460,
            orbit_speed: 2.0 * PI / (30687.0 * 86400.0),
            orbit_angle: 0.0,
            orbit_inclination: 0.769_f32.to_radians(),
            orbit_node: 74.006_f32.to_radians(),
            rotation_speed: 2.0 * PI / (17.24 * 3600.0),
            axial_tilt: 97.77_f32.to_radians(),
            color: Color::Srgba(tailwind::BLUE_300),
            children: vec![],
        },
        // Neptune
        CelestialBody {
            name: "Neptune".to_string(),
            radius: 24622.0 / 149_600_000.0, // ≈ 1.65e-4 AU
            orbit_radius: 30.07,
            orbit_eccentricity: 0.0086,
            orbit_speed: 2.0 * PI / (60190.0 * 86400.0),
            orbit_angle: 0.0,
            orbit_inclination: 1.77_f32.to_radians(),
            orbit_node: 131.784_f32.to_radians(),
            rotation_speed: 2.0 * PI / (16.11 * 3600.0),
            axial_tilt: 28.32_f32.to_radians(),
            color: Color::Srgba(tailwind::BLUE_800),
            children: vec![],
        },
    ]
}

#[require(InheritedVisibility)]
#[derive(Component, Reflect)]
pub struct SolarSystem {
    pub color: Color,
    pub size: f32, // AU
    // distance scale for the orbit radius
    pub distance_scale: f32,
    pub star_scale: f32,
    pub planet_scale: f32,
}



impl Default for SolarSystem {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            size: 1.0, // AU
            distance_scale: 100.0,
            planet_scale: 10000.0,
            star_scale: 100.0,
        }
    }
}

pub fn spawn_solor_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &SolarSystem, Option<&Children>), Changed<SolarSystem>>,
) {
    for (e, config, children_maybe) in query.iter() {
        if let Some(children) = children_maybe {
            // remove current children
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }

        // spawn new children
        commands.spawn((
                ChildOf { parent: e },
                Name::new("Star"),
                Mesh3d(meshes.add(Sphere {                    
                    radius:  (695_700.0 / 149_600_000.0) * config.star_scale,
                })),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    emissive: LinearRgba::rgb(1000., 1000., 1000.),
                    ..default()
                })),
                PointLight {
                    color: Color::WHITE,
                    intensity: 1500000000.0,
                    range: 1000000000.0,
                    shadows_enabled: true,
                    ..Default::default()
                },                
                NotShadowCaster,
                NoFrustumCulling,
            ));
        
        for planet in planet_list() {
            planet.build(&mut commands, e, config, &mut meshes, &mut materials);
        }
    }
}

/// System that updates orbital positions in 3D.
fn orbit_system(time: Res<Time>, mut query: Query<(&mut CelestialOrbit, &mut Transform)>) {
    for (mut orbit, mut transform) in query.iter_mut() {
        // Update the orbital angle.
        orbit.orbit_angle += orbit.orbit_speed * time.delta_secs();
        // Calculate semi-minor axis.
        let a = orbit.orbit_radius;
        let b = a * (1.0 - orbit.orbit_eccentricity);
        // Compute base position in the XZ plane.
        let base = Vec3::new(
            a * orbit.orbit_angle.cos(),
            0.0,
            b * orbit.orbit_angle.sin(),
        );
        // Rotate the base position: first tilt the orbital plane (around X)...
        let inclined = Quat::from_rotation_x(orbit.orbit_inclination) * base;
        // ...then rotate around Y to orient the node.
        let rotated = Quat::from_rotation_y(orbit.orbit_node) * inclined;
        transform.translation = rotated;
    }
}

/// System that rotates the visual representation (child) to simulate self rotation.
fn rotation_system(time: Res<Time>, mut query: Query<(&CelestialRotation, &mut Transform)>) {
    for (rotation, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(
            rotation.rotation_speed * time.delta_secs(),
        ));
    }
}

/// System that draws 3D orbit paths using gizmos.
fn draw_orbit(
    orbit_query: Query<(&CelestialOrbit, &ChildOf)>,
    trans_query: Query<(&GlobalTransform)>,
    mut gizmos: Gizmos,
) {
    for (orbit, child_of) in orbit_query.iter() {
        // Get the global transform of the parent entity.
        let global_trans = trans_query.get(child_of.parent).unwrap();
        let center = global_trans.translation();
        let a = orbit.orbit_radius;
        let b = a * (1.0 - orbit.orbit_eccentricity);
        let segments = 64;
        let mut points = Vec::with_capacity(segments + 1);
        for i in 0..=segments {
            let theta = (i as f32 / segments as f32) * std::f32::consts::TAU;
            let base = Vec3::new(a * theta.cos(), 0.0, b * theta.sin());
            let inclined = Quat::from_rotation_x(orbit.orbit_inclination) * base;
            let rotated = Quat::from_rotation_y(orbit.orbit_node) * inclined;
            points.push(center + rotated);
        }
        for i in 0..segments {
            gizmos.line(points[i], points[i + 1], Color::Srgba(tailwind::RED_400));
        }
    }
}
