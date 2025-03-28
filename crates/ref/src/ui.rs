use bevy::{ecs::system::SystemState, prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::{
    bevy_egui::EguiContext,
    egui::{self},
};
use sly_common::UiConfig;
use strum::IntoEnumIterator;

use crate::{Generate, Prefab, Rename, Selected, Workflow};

#[derive(Component)]
#[require(Button)]
pub struct Btn;

pub fn on_add_button() {
    // .observe(update_material_on::<Pointer<Over>>(hover_matl.clone()))
    // .observe(update_material_on::<Pointer<Out>>(white_matl.clone()))
    // .observe(update_material_on::<Pointer<Pressed>>(pressed_matl.clone()))
    // .observe(update_material_on::<Pointer<Released>>(hover_matl.clone()))
}


pub fn setup_ui(mut commands: Commands, ui: Res<UiConfig>, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                
                right: Val::Px(10.),
                bottom: Val::Px(10.),
                padding: UiRect::all(Val::Px(2.0)),
                // horizontally center child text
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderRadius::all(Val::Px(5.)),
            BackgroundColor(ui.panel_background),
            BorderColor(ui.panel_border),
            Outline {
                width: Val::Px(2.),
                color: Color::WHITE,
                ..default()
            },
        ))
        .with_children(|parent| {
            // add button
            parent
                .spawn((
                    Button,
                    Node {
                        padding: UiRect::all(Val::Px(2.0)),
                        margin: UiRect::all(Val::Px(2.0)),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(ui.normal_button),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        ImageNode::new(asset_server.load("textures/icon/white/plus.png")),
                        Node {
                            // This will set the logo to be 200px wide, and auto adjust its height
                            width: Val::Px(30.0),
                            height: Val::Px(30.0),
                            ..default()
                        },
                        Outline {
                            width: Val::Px(2.),
                            color: ui.normal_button_border,
                            ..default()
                        },
                        // TextFont {
                        //     font_size: 33.0,
                        //     ..default()
                        // },
                        // TextColor(ui.normal_button_text),
                        // Text::new("Add"),
                    ));
                })
                .observe(
                    |_trigger: Trigger<Pointer<Click>>, mut commands: Commands| {
                        commands.spawn((
                            Name::new("Prefab"),
                            Prefab {
                                name: "Prefab".to_string(),
                                workflow: Workflow::StaticImage { image: None },
                            },
                            Transform::from_translation(Vec3::new(0.0, 2.5, 0.0)),
                        ));
                    },
                );
        });
}

pub fn ui_select(world: &mut World) {
    let mut egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world)
        .clone();

    let mut system_state: SystemState<(Commands, Query<(Entity, &mut Prefab), With<Selected>>)> =
        SystemState::new(world);

    let (mut cmd, mut query) = system_state.get_mut(world);

    egui::Window::new("Select").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {
            // let changed = ui.text_edit_singleline(&mut prefab.name);
            // if changed.changed() {
            //     world
            //         .query_filtered::<&mut Prefab, With<Selected>>()
            //         .single(world)
            //         .name = prefab.name.clone();
            // }
            //bevy_inspector_egui::bevy_inspector::ui_for_entities_filtered(world, ui, false, &Filter::<With<Prefab>>::all());
            for (e, mut p) in query.iter_mut() {
                let id = egui::Id::new("prefab ui").with(e);
                let mut changed = false;
                egui::Grid::new(id)
                    .num_columns(2)
                    .spacing([16.0, 4.0]) // [horizontal, vertical] spacing
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Name");
                        let mut new_name = p.name.clone();
                        changed |= ui.add(egui::TextEdit::singleline(&mut new_name)).changed();
                        if changed {
                            cmd.trigger_targets(Rename(new_name), e);
                        }
                        ui.end_row();

                        ui.label("Workflow");
                        let mut workflow_copy = p.workflow.clone();
                        let mut workflow_changed = false;
                        egui::ComboBox::from_id_salt(id)
                            .selected_text(format!("{}", workflow_copy))
                            .show_ui(ui, |ui| {
                                for variant in Workflow::iter() {
                                    workflow_changed |= ui
                                        .selectable_value(
                                            &mut workflow_copy,
                                            variant.clone(),
                                            format!("{}", &variant),
                                        )
                                        .changed();
                                }
                            });

                        // handles keeping useful data when changing workflow types
                        if workflow_changed {
                            changed = true;
                            match (workflow_copy, &p.workflow) {
                                (
                                    Workflow::StaticImage { .. },
                                    Workflow::TextToImage { image, .. },
                                ) => {
                                    p.workflow = Workflow::StaticImage {
                                        image: image.clone(),
                                    };
                                }
                                (Workflow::StaticImage { .. }, Workflow::StaticImage { .. }) => {
                                    unreachable!()
                                }
                                (
                                    Workflow::StaticImage { .. },
                                    Workflow::TextToModel { image, .. },
                                ) => {
                                    p.workflow = Workflow::StaticImage {
                                        image: image.clone(),
                                    };
                                }
                                (Workflow::TextToImage { .. }, Workflow::StaticImage { image }) => {
                                    p.workflow = Workflow::TextToImage {
                                        seed: 0,
                                        seed_random: false,
                                        prompt: "".to_string(),
                                        image: image.clone(),
                                    };
                                }
                                (
                                    Workflow::TextToImage { .. },
                                    Workflow::TextToModel {
                                        image,
                                        seed,
                                        seed_random,
                                        prompt,
                                        ..
                                    },
                                ) => {
                                    p.workflow = Workflow::TextToImage {
                                        image: image.clone(),
                                        seed: *seed,
                                        seed_random: *seed_random,
                                        prompt: prompt.clone(),
                                    };
                                }
                                (Workflow::TextToImage { .. }, Workflow::TextToImage { .. }) => {
                                    unreachable!()
                                }
                                (Workflow::TextToModel { .. }, Workflow::StaticImage { image }) => {
                                    p.workflow = Workflow::TextToModel {
                                        seed: 0,
                                        seed_random: false,
                                        prompt: "".to_string(),
                                        image: image.clone(),
                                        num_faces: 50000,
                                        model: None,
                                    };
                                }
                                (
                                    Workflow::TextToModel { .. },
                                    Workflow::TextToImage {
                                        image,
                                        seed,
                                        seed_random,
                                        prompt,
                                    },
                                ) => {
                                    p.workflow = Workflow::TextToModel {
                                        seed: *seed,
                                        seed_random: *seed_random,
                                        num_faces: 50000,
                                        prompt: prompt.clone(),
                                        image: image.clone(),
                                        model: None,
                                    };
                                }

                                (Workflow::TextToModel { .. }, Workflow::TextToModel { .. }) => {
                                    unreachable!()
                                }
                            }
                        }
                        ui.end_row();

                        let mut enable_generate = true;
                        match &mut p.workflow {
                            Workflow::StaticImage { image } => {
                                image_widget(ui, image);
                                enable_generate = false;
                            }
                            Workflow::TextToImage {
                                image,
                                prompt,
                                seed,
                                seed_random,
                            } => {
                                changed |= prompt_widget(ui, prompt);
                                image_widget(ui, image);
                                seed_wigit(ui, seed, seed_random);
                            }
                            Workflow::TextToModel {
                                prompt,
                                image,
                                model,
                                seed,
                                seed_random,
                                num_faces,
                            } => {
                                changed |= prompt_widget(ui, prompt);
                                image_widget(ui, image);
                                model_widget(ui, model);
                                changed |= seed_wigit(ui, seed, seed_random);

                                ui.label("Faces");
                                changed |= ui
                                    .add(
                                        egui::Slider::new(num_faces, 0..=u32::MAX)
                                            .text("Faces")
                                            .step_by(1000.)
                                            .clamping(egui::SliderClamping::Always),
                                    )
                                    .changed();
                                ui.end_row();

                                ui.label("");
                                if ui
                                    .add_enabled(
                                        enable_generate,
                                        egui::Button::new("Generate Image")
                                            .min_size(egui::Vec2::new(ui.available_width(), 30.0)),
                                    )
                                    .clicked()
                                {
                                    cmd.trigger_targets(Generate(Some(0)), e);
                                }
                                ui.end_row();

                                ui.label("");
                                if ui
                                    .add_enabled(
                                        enable_generate,
                                        egui::Button::new("Generate Model")
                                            .min_size(egui::Vec2::new(ui.available_width(), 30.0)),
                                    )
                                    .clicked()
                                {
                                    cmd.trigger_targets(Generate(Some(1)), e);
                                }
                                ui.end_row();
                            }
                        }

                        ui.label("");
                        if ui
                            .add_enabled(
                                enable_generate,
                                egui::Button::new("Generate Full")
                                    .min_size(egui::Vec2::new(ui.available_width(), 30.0)),
                            )
                            .clicked()
                        {
                            cmd.trigger_targets(Generate(None), e);
                        }
                        ui.end_row();
                    });
            }
            //ui_for_entities_filtered(world, ui, &Filter::<(With<Prefab>, With<Selected>)>::all());

            //bevy_inspector_egui::bevy_inspector::ui_for_value(prefab.deref_mut(), ui, world);

            ui.allocate_space(ui.available_size());
        });
    });

    system_state.apply(world);
}

fn image_widget(ui: &mut egui::Ui, p: &mut Option<String>) {
    ui.label("Image");
    if let Some(text) = p {
        ui.add_enabled(
            false,
            egui::TextEdit::singleline(text).desired_width(f32::INFINITY), // optional, makes it fill width
        );
    } else {
        ui.label("None");
    }
    ui.end_row();
}

fn model_widget(ui: &mut egui::Ui, p: &mut Option<String>) {
    ui.label("Model");
    if let Some(text) = p {
        ui.add_enabled(
            false,
            egui::TextEdit::singleline(text).desired_width(f32::INFINITY), // optional, makes it fill width
        );
    } else {
        ui.label("None");
    }
    ui.end_row();
}

fn prompt_widget(ui: &mut egui::Ui, prompt: &mut String) -> bool {
    let mut changed = false;
    ui.label("Prompt");
    changed |= ui.text_edit_multiline(prompt).changed();
    ui.end_row();
    changed
}

fn seed_wigit(ui: &mut egui::Ui, seed: &mut u32, random: &mut bool) -> bool {
    let mut changed = false;

    ui.label("Seed");
    changed |= ui
        .add(
            egui::Slider::new(seed, 0..=u32::MAX)
                .text("Seed")
                .clamping(egui::SliderClamping::Always),
        )
        .changed();
    ui.end_row();

    ui.label("Seed");
    if ui.checkbox(random, "Randomize").changed() {
        changed = true;
    }
    ui.end_row();

    changed
}
