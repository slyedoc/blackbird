use bevy::prelude::*;

#[cfg(feature = "outline")]
use bevy_mod_outline::{OutlinePlugin, OutlineVolume, OutlineMode};

#[cfg(not(feature = "outline"))]
use bevy::pbr::wireframe::{Wireframe, WireframePlugin};

use crate::EditorState;

/// A marker for editor selected entities
#[derive(Component, Default, Clone)]
pub struct EditorSelected;

/// Selection system plugins
pub struct EditorSelectedPlugin;

impl Plugin for EditorSelectedPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(feature = "outline"))]
        {
            if !app.is_plugin_added::<WireframePlugin>() {
                app.add_plugins(WireframePlugin);
            }
        }
        #[cfg(feature = "outline")]
        {
            if !app.is_plugin_added::<OutlinePlugin>() {
                app.add_plugins(OutlinePlugin);
            }
        }
        app.add_systems(
            Update,
            selected_entity_wireframe_update.run_if(in_state(EditorState::Enabled)),
        );
        app.add_systems(OnEnter(EditorState::Disabled), clear_wireframes);
    }
}

#[cfg(not(feature = "outline"))]
fn selected_entity_wireframe_update(
    mut cmds: Commands,
    del_wireframe: Query<Entity, (With<Wireframe>, Without<EditorSelected>)>,
    need_wireframe: Query<Entity, (Without<Wireframe>, With<EditorSelected>)>,
) {
    for e in del_wireframe.iter() {
        cmds.entity(e).remove::<Wireframe>();
    }

    for e in need_wireframe.iter() {
        cmds.entity(e).insert(Wireframe);
    }
}

#[cfg(not(feature = "outline"))]
fn clear_wireframes(mut cmds: Commands, del_wireframe: Query<Entity, With<Wireframe>>) {
    for e in del_wireframe.iter() {
        cmds.entity(e).remove::<Wireframe>();
    }
}

#[cfg(feature = "outline")]
fn selected_entity_wireframe_update(
    mut cmds: Commands,
    del_wireframe: Query<Entity, (With<OutlineVolume>, Without<EditorSelected>)>,
    need_wireframe: Query<Entity, (Without<OutlineVolume>, With<EditorSelected>)>,
) {
    use bevy_mod_outline::OutlineMode;

    for e in del_wireframe.iter() {
        cmds.entity(e)
            .remove::<OutlineVolume>()
            .remove::<OutlineMode>();
    }

    for e in need_wireframe.iter() {
        cmds.entity(e).insert((
            OutlineVolume {
                visible: true,
                colour: Color::srgb(1.0, 1.0, 0.0),
                width: 2.0,
            },
            OutlineMode::ExtrudeReal,
        ));
    }
}

#[cfg(feature = "outline")]
fn clear_wireframes(mut cmds: Commands, del_wireframe: Query<Entity, (With<EditorSelected>, With<OutlineVolume>)>) {
    for e in del_wireframe.iter() {
        cmds.entity(e)
            .remove::<OutlineVolume>()
            .remove::<OutlineMode>();
    }
}

#[cfg(test)]
#[cfg(not(feature = "outline"))]
mod tests {
    use super::*;

    #[test]
    fn test_clear_wireframes() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, clear_wireframes);
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Wireframe);
            commands.spawn(Wireframe);
        });
        app.update();

        let mut query = app.world.query_filtered::<Entity, With<Wireframe>>();
        assert_eq!(0, query.iter(&app.world).count());
    }

    #[test]
    fn removes_wireframe_if_not_selected() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, selected_entity_wireframe_update);
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Wireframe);
            commands.spawn(Wireframe);
        });
        app.update();

        let mut query = app.world.query_filtered::<Entity, With<Wireframe>>();
        assert_eq!(0, query.iter(&app.world).count());
    }

    #[test]
    fn adds_wireframe_if_selected() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_systems(Update, selected_entity_wireframe_update);
        app.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(EditorSelected);
            commands.spawn(EditorSelected);
        });
        app.update();

        let mut query = app
            .world
            .query_filtered::<Entity, (With<Wireframe>, With<EditorSelected>)>();
        assert_eq!(2, query.iter(&app.world).count());
    }
}
