use crate::states::AppState;
use bevy::{color::palettes::tailwind, ecs::relationship::RelatedSpawnerCommands, pbr::NotShadowCaster, prelude::*};

pub struct SolPlugin;

impl Plugin for SolPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_sol, rotate, orbit))
        .add_systems(PostUpdate, draw_orbit)
            .register_type::<Sol>();
    }
}

#[require(InheritedVisibility)]
#[derive(Component, Reflect)]
pub struct Sol {
    pub distance_scale: f32,
    pub size_scale: f32,
}

impl Default for Sol {
    fn default() -> Self {
        Self {
            distance_scale: 1.0,
            size_scale: 1.0,
        }
    }
}

pub struct Body {
    pub name: String,
    pub size: f32,
    pub distance: f32,
    pub rotation_period: f32,
    pub orbit_period: f32,
    pub tilt: f32,
    pub color: Color,
    pub children: Vec<Body>,
}
impl Body {
    pub fn spawn_recursive(
        &self,
        parent: &mut RelatedSpawnerCommands<ChildOf>,
        config: &Sol,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        parent.spawn((
            Name::new(format!("Planet - {}", self.name)),
            Mesh3d(meshes.add(Sphere {
                radius: self.size * config.size_scale,
            })),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: self.color,
                ..default()
            })),
            Transform::from_translation(Vec3::new(
                self.distance * config.distance_scale,
                0.0,
                0.0,
            )),
            Rotate {
                speed: self.rotation_period,
            },
            Orbit {
                speed: self.orbit_period,
            },
        ))
        .with_children(|child_builder| {
            for b in &self.children {
                b.spawn_recursive(child_builder, config, meshes, materials);
            }
        });
    }
}

impl Sol {
    pub fn bodies() -> Vec<Body> {
        vec![
            Body {
                name: "Mercury".to_string(),
                size: 0.05,
                distance: 1.39,
                rotation_period: 1.0,
                orbit_period: 2.0,
                tilt: 0.0,
                color: Color::WHITE,
                children: vec![],
            },
            Body {
                name: "Venus".to_string(),
                size: 0.1,
                distance: 1.72,
                rotation_period: 0.2,
                orbit_period: 1.5,
                tilt: 0.0,
                color: Color::WHITE,
                children: vec![],
            },
            Body {
                name: "Earth".to_string(),
                size: 0.1,
                distance: 2.0,
                rotation_period: 1.0,
                orbit_period: 1.0,
                tilt: 0.0,
                color: Color::WHITE,
                children: vec![
                    Body {
                        name: "Moon".to_string(),
                        size: 0.1,
                        distance: 0.3,
                        rotation_period: 1.0,
                        orbit_period: 1.0,
                        tilt: 0.0,
                        color: Color::Srgba(tailwind::RED_300),
                        children: vec![],
                    },
                ],
            },
            Body {
                name: "Mars".to_string(),
                size: 0.05,
                distance: 3.52,
                rotation_period: 1.88,
                orbit_period: 0.8,
                tilt: 0.0,
                color: Color::WHITE,
                children: vec![],
            },
            Body {
                name: "Jupiter".to_string(),
                size: 0.5,
                distance: 6.2,
                rotation_period: 11.86,
                orbit_period: 0.4,
                tilt: 0.0,
                color: Color::WHITE,
                children: vec![],
            },
            Body {
                name: "Saturn".to_string(),
                size: 0.4,
                distance: 9.58,
                rotation_period: 29.46,
                orbit_period: 0.2,
                tilt: 0.0,
                color: Color::WHITE,
                children: vec![],
            },
            Body {
                name: "Uranus".to_string(),
                size: 0.3,
                distance: 19.22,
                rotation_period: 84.0,
                orbit_period: 0.1,
                tilt: 0.0,
                color: Color::WHITE,
                children: vec![],
            },
            Body {
                name: "Neptune".to_string(),
                size: 0.3,
                distance: 30.05,
                rotation_period: 164.0,
                orbit_period: 0.05,
                tilt: 0.0,
                color: Color::WHITE,
                children: vec![],
            },
        ]
    }
}


pub fn spawn_sol(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &Sol, Option<&Children>), Changed<Sol>>,
) {
    for (e, config, children_maybe) in query.iter() {
        if let Some(children) = children_maybe {
            // remove current children
            for child in children.iter() {
                commands.entity(child).despawn();
            }
        }

        // spawn new children
        commands.entity(e).with_children(|parent| {
            // star
            parent.spawn((
                Name::new("Star - Sol"),
                Mesh3d(meshes.add(Sphere {
                    radius: 1.0 * config.size_scale,
                })),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    unlit: true,
                    ..default()
                })),
                Transform::from_xyz(0.0, 0.0, 0.0),
                NotShadowCaster,
            ));

            // planets
            // Name, Size, Distance, Rotation - Peroid, Obit - Peroid, Colo
            //  assuming Sun is 1.0 radius and 1.0 distance

            for b in Sol::bodies().iter() {
                b.spawn_recursive(parent, config, &mut meshes, &mut materials);                
            }
        });
    }
}

#[derive(Component, Reflect)]
pub struct Rotate {
    pub speed: f32,
}

fn rotate(time: Res<Time>, mut query: Query<(&Rotate, &mut Transform)>) {
    for (rotate, mut transform) in query.iter_mut() {
        transform.rotate(Quat::from_rotation_y(rotate.speed * time.delta_secs()));
    }
}

#[derive(Component, Reflect)]
pub struct Orbit {
    pub speed: f32,
}

fn orbit(
    time: Res<Time>,
    mut query: Query<(Entity, &Orbit, &ChildOf)>,
    mut transform_query: Query<&mut Transform>,
) {
    for (e, orbit, child_of) in query.iter_mut() {
        let [mut transform, parent_transform] =
            transform_query.get_many_mut([e, child_of.parent]).unwrap();

        // rotate around parent
        transform.rotate_around(
            parent_transform.translation,
            Quat::from_rotation_y(orbit.speed * time.delta_secs()),
        );
    }
}

fn draw_orbit(
    time: Res<Time>,
    mut gizmos: Gizmos,
    mut query: Query<(Entity, &Orbit, &ChildOf)>,
    mut transform_query: Query<&mut Transform>,
) {
    for (e, orbit, child_of) in query.iter_mut() {
        let [mut transform, parent_transform] =
            transform_query.get_many_mut([e, child_of.parent]).unwrap();

        gizmos.line(
            transform.translation,
            parent_transform.translation,
            Color::Srgba(tailwind::YELLOW_500),
        );
    }
}
