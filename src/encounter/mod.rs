use bevy::prelude::{EventReader, NextState, Plugin, ResMut, Resource, Update};

use crate::{
    gamestate::GameMode,
    overworld::{BeginNPCEncounter, BeginPlayerEncounter},
};

pub struct EncounterPlugin;

#[derive(Resource)]
pub struct BattleData {}

impl Plugin for EncounterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(BattleData {});
        app.add_systems(Update, (trigger_npc_encounter, trigger_player_encounter));
    }
}

pub fn trigger_player_encounter(
    mut state: ResMut<NextState<GameMode>>,
    mut triggers: EventReader<BeginPlayerEncounter>,
    mut battle_data: ResMut<BattleData>,
) {
    for _ in triggers.iter() {
        println!("Triggering player encounter");
        state.set(GameMode::Encounter);
    }
}

pub fn trigger_npc_encounter(mut triggers: EventReader<BeginNPCEncounter>) {
    for _ in triggers.iter() {
        println!(
            "Triggered a fight between two NPCs. This does nothing and can
        remain doing nothing unless we want to have NPCs fight each other."
        );
    }
}
