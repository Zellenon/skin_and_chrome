use bevy::{
    prelude::{
        Component, OnEnter, OnExit, Plugin, Query, ResMut, Startup, States, Visibility, With,
    },
    reflect::Reflect,
};
use bevy_twin_stick::{
    bevy_rapier2d::prelude::RapierConfiguration, meta_states::PluginControlState,
};

// Things that should only be shown in x gamestate
#[derive(Component)]
pub struct OverworldBound;
#[derive(Component)]
pub struct EncounterBound;

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash, Reflect)]
pub enum GameMode {
    Neither,
    #[default]
    Overworld,
    Encounter,
}

impl PluginControlState for GameMode {
    fn active_state() -> Self {
        Self::Overworld
    }
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state::<GameMode>();

        app.add_systems(Startup, (leave_overworld, leave_encounter)); // Just to make sure things
                                                                      // start at the right values
        app.add_systems(OnEnter(GameMode::Overworld), enter_overworld);
        app.add_systems(OnExit(GameMode::Overworld), leave_overworld);
        app.add_systems(OnEnter(GameMode::Encounter), enter_encounter);
        app.add_systems(OnExit(GameMode::Encounter), leave_encounter);
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
