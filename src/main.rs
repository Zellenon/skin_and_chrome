use std::marker::PhantomData;

use bevy::{
    app::{App, AppExit},
    prelude::{
        default, in_state, ClearColor, EventWriter, IntoSystemConfigs, NextState, OnEnter,
        PluginGroup, ResMut, Update,
    },
    render::color::Color,
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_stats::StatPlugin;
use bevy_twin_stick::{bevy_rapier2d::render::RapierDebugRenderPlugin, TwinStickToggleablePlugin};
use encounter::EncounterPlugin;
use gamestate::{AppState, GameStatePlugin, GameplayMode, OverworldBound};
use overworld::OverworldPlugin;

mod encounter;
mod gamestate;
mod overworld;
mod rpg;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.build().set(WindowPlugin {
        primary_window: Some(Window {
            title: "Skin and Chrome".to_string(),
            fit_canvas_to_parent: true,
            ..Default::default()
        }),
        ..default()
    }));

    app.add_plugins((
        TwinStickToggleablePlugin {
            use_default_camera: false, // TODO: Change this to true if you don't want to bother making your own camera system
            _p: PhantomData::<GameplayMode>,
        },
        StatPlugin,
        EguiPlugin,
    ));

    app.add_systems(OnEnter(AppState::Quitting), exit);
    app.add_systems(Update, main_menu_gui.run_if(in_state(AppState::MainMenu)));

    app.add_plugins((GameStatePlugin, OverworldPlugin, EncounterPlugin));

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
        app.add_plugins(RapierDebugRenderPlugin::default());
    }

    app.insert_resource(ClearColor(Color::rgb(
        0xA9 as f32 / 255.0,
        0xA9 as f32 / 255.0,
        0xAF as f32 / 255.0,
    )));

    app.run();

    Ok(())
}

pub fn main_menu_gui(mut root: EguiContexts, mut state: ResMut<NextState<AppState>>) {
    egui::CentralPanel::default().show(root.ctx_mut(), |ui| {
        ui.allocate_space(egui::Vec2::new(1.0, 200.0));

        egui::Frame::dark_canvas(ui.style()).show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label("Skin and Chrome");
                if ui.button("Play Game").clicked() {
                    state.set(AppState::InGame);
                }
                if ui.button("Options").clicked() {
                    todo!();
                }
                if ui.button("Quit").clicked() {
                    state.set(AppState::Quitting);
                }
            })
        });
    });
}

fn exit(mut app_exit_events: EventWriter<AppExit>) {
    app_exit_events.send(AppExit);
}
