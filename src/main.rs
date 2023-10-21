use std::marker::PhantomData;

use bevy::{
    app::App,
    prelude::{default, ClearColor, Commands, PluginGroup, Startup},
    render::color::Color,
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_composable::{
    app_impl::ComplexSpawnable,
    tree::{ComponentTree, EntityCommandSet},
    CT,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_stats::StatPlugin;
use bevy_twin_stick::{
    ai::keyboard::KeyboardAI, bevy_rapier2d::render::RapierDebugRenderPlugin, player::Player,
    TwinStickToggleablePlugin,
};
use encounter::EncounterPlugin;
use gamestate::{GameMode, GameStatePlugin, OverworldBound};
use overworld::OverworldPlugin;

use crate::overworld::overworld_actor;

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
            _p: PhantomData::<GameMode>,
        },
        StatPlugin,
    ));

    app.add_plugins((GameStatePlugin, OverworldPlugin, EncounterPlugin));

    if cfg!(debug_assertions) {
        app.add_plugins(WorldInspectorPlugin::new());
        app.add_plugins(RapierDebugRenderPlugin::default());
    }

    app.add_systems(Startup, setup);

    app.insert_resource(ClearColor(Color::rgb(
        0xA9 as f32 / 255.0,
        0xA9 as f32 / 255.0,
        0xAF as f32 / 255.0,
    )));

    app.run();

    Ok(())
}

fn setup(mut commands: Commands) {
    commands.spawn_complex(player_tree());
}

fn player_tree() -> ComponentTree {
    overworld_actor("Player", 800.) + CT!(Player, KeyboardAI)
}
