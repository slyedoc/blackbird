use bevy::{
    ecs::system::SystemState,
    prelude::*,
    window::PrimaryWindow,
};
use bevy_inspector_egui::{
    bevy_egui::EguiContext,
    egui::{self, },
};
use strum::IntoEnumIterator;

use crate::{Generate, Prefab, Rename, Selected, Workflow};

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
                let id = egui::Id::new("prefab ui").with(p.name.clone());
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

                        // dropdown for workflow
                        // ui.label("Workflow");
                        // cx.ui_for_reflect(&mut p.workflow, ui);
                        // ui.end_row();

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


                        if workflow_changed {
                            changed = true;
                            // handles keeping useful data when changing workflow types
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
                                (
                                    Workflow::TextToImage { .. },
                                    Workflow::TextToImage { .. }
                                ) => unreachable!(),
                                (
                                    Workflow::TextToModel { .. },                                    
                                    Workflow::StaticImage { image },
                                ) => {
                                    p.workflow = Workflow::TextToModel {
                                        seed: 0,
                                        seed_random: false,
                                        prompt: "".to_string(),
                                        image: image.clone(),
                                        model: None,
                                    };
                                }
                                (
                                    Workflow::TextToModel { ..},
                                    Workflow::TextToImage { image, seed, seed_random, prompt },
                                ) => {
                                    p.workflow = Workflow::TextToModel {
                                        seed: *seed,
                                        seed_random: *seed_random,
                                        prompt: prompt.clone(),
                                        image: image.clone(),
                                        model: None,
                                    };
                                },
                                
                                (
                                    Workflow::TextToModel { .. },
                                    Workflow::TextToModel { .. },
                                ) => unreachable!(),
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
                            } => {
                                changed |= prompt_widget(ui, prompt);
                                image_widget(ui, image);
                                model_widget(ui, model);
                                changed |= seed_wigit(ui, seed, seed_random);
                            }
                        }

                        ui.label("");                        
                        if ui.add_enabled(enable_generate,
                                egui::Button::new("Generate").min_size(egui::Vec2::new(ui.available_width(), 30.0)))
                            .clicked()
                        {
                            cmd.trigger_targets(Generate, e);
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

    changed |= ui.text_edit_singleline(prompt).changed(); // optional, makes it fill width

    ui.end_row();
    changed
}

fn seed_wigit(ui: &mut egui::Ui, seed: &mut u32, random: &mut bool) -> bool {
    let mut changed = false;

    // egui::Slider::new(&mut seed, 0..u32::MAX)
    //     .text("Seed")
    //     .clamping(egui::SliderClamping::Always)
    //     .show_value(false),
    ui.label("Seed");
    changed |= ui
        .add(egui::Slider::new(seed, 0..=u32::MAX).text("Seed").clamping(
            egui::SliderClamping::Always,
        ))
        .changed();
    ui.end_row();

    ui.label("Seed");
    if ui.checkbox(random, "Randomize").changed() {
        changed = true;
    }
    ui.end_row();

    changed
}

