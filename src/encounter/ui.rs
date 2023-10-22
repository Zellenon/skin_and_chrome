use bevy::prelude::{Name, Query, Res};
use bevy_egui::{
    egui::{self, load::SizedTexture},
    EguiContexts,
};

use super::BattleData;

pub fn player_creatures_ui(
    encounter_data: Res<BattleData>,
    mut contexts: EguiContexts,
    names: Query<&Name>,
) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::TOP).with_cross_justify(true),
                |ui| {
                    for creature in encounter_data.bottom_party.members.iter() {
                        ui.vertical(|ui| {
                            ui.add_enabled_ui(false, |ui| {
                                ui.label(names.get(*creature).unwrap().to_string());
                                ui.button("Attack");
                                ui.button("Defend");
                                ui.button("Cover");
                                ui.image(SizedTexture::new(
                                    *encounter_data.battler_images.get(creature).unwrap(),
                                    [128., 128.],
                                ));
                            });
                        });
                        ui.separator();
                    }
                },
            );
        });
}

pub fn enemy_creatures_ui(
    encounter_data: Res<BattleData>,
    mut contexts: EguiContexts,
    names: Query<&Name>,
) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::TOP).with_cross_justify(true),
                |ui| {
                    for creature in encounter_data.top_party.members.iter() {
                        ui.vertical(|ui| {
                            ui.add_enabled_ui(false, |ui| {
                                ui.label(names.get(*creature).unwrap().to_string());
                                ui.button("Attack");
                                ui.button("Defend");
                                ui.button("Cover");
                                ui.image(SizedTexture::new(
                                    *encounter_data.battler_images.get(creature).unwrap(),
                                    [128., 128.],
                                ));
                            })
                        });
                        ui.separator();
                        // ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
                    }
                },
            );
        });
}
