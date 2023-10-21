use bevy::{
    prelude::{
        Commands, Component, NextState, OnEnter, OnExit, Plugin, Query, ResMut, Startup, States,
        Visibility, With,
    },
    reflect::Reflect,
};
use bevy_composable::tree::EntityCommandSet;
use bevy_composable::{app_impl::ComplexSpawnable, tree::ComponentTree, CT};
use bevy_twin_stick::ai::keyboard::KeyboardAI;
use bevy_twin_stick::player::Player;
use bevy_twin_stick::{
    bevy_rapier2d::prelude::RapierConfiguration, meta_states::PluginControlState,
};

use crate::overworld::overworld_actor;

/// Things that should only be shown in x gamestate
#[derive(Component)]
pub struct OverworldBound;
#[derive(Component)]
pub struct EncounterBound;

/// The game has two main modes representing the states of an RPG game, Overworld and Encounter.
/// If we implement something like a crafting system or interacting puzzle later, they should
/// probably be added here.
#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum GameplayMode {
    #[default]
    Neither,
    Overworld,
    Encounter,
}

/// Do we need other states besides these two?
#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum AppState {
    #[default]
    LoadingAssets,
    MainMenu,
    InGame,
    Quitting,
}

impl PluginControlState for GameplayMode {
    fn active_state() -> Self {
        Self::Overworld
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameplayMode>();
        app.add_state::<AppState>();

        app.add_systems(Startup, (leave_overworld, leave_encounter)); // Just to make sure things
                                                                      // start at the right values
        app.add_systems(OnEnter(AppState::InGame), start_new_game);
        app.add_systems(OnEnter(GameplayMode::Overworld), enter_overworld);
        app.add_systems(OnExit(GameplayMode::Overworld), leave_overworld);
        app.add_systems(OnEnter(GameplayMode::Encounter), enter_encounter);
        app.add_systems(OnExit(GameplayMode::Encounter), leave_encounter);
    }
}

fn enter_overworld(
    mut things_to_show: Query<&mut Visibility, With<OverworldBound>>,
    mut physics: ResMut<RapierConfiguration>,
) {
    for mut visibility in things_to_show.iter_mut() {
        *visibility = Visibility::Visible;
    }
    (*physics).physics_pipeline_active = true;
}

fn leave_overworld(
    mut things_to_hide: Query<&mut Visibility, With<OverworldBound>>,
    mut physics: ResMut<RapierConfiguration>,
) {
    for mut visibility in things_to_hide.iter_mut() {
        *visibility = Visibility::Hidden;
    }
    (*physics).physics_pipeline_active = false;
}

fn enter_encounter(mut things_to_show: Query<&mut Visibility, With<EncounterBound>>) {
    for mut visibility in things_to_show.iter_mut() {
        *visibility = Visibility::Visible;
    }
}

fn leave_encounter(mut things_to_hide: Query<&mut Visibility, With<EncounterBound>>) {
    for mut visibility in things_to_hide.iter_mut() {
        *visibility = Visibility::Hidden;
    }
}

fn start_new_game(mut commands: Commands, mut game_mode: ResMut<NextState<GameplayMode>>) {
    commands.spawn_complex(player_tree());
    game_mode.set(GameplayMode::Overworld);
}

fn player_tree() -> ComponentTree {
    overworld_actor("Player", 800.) + CT!(Player, KeyboardAI)
}
