use bevy::{
    ecs::query::Has,
    prelude::{Entity, EventReader, NextState, Parent, Plugin, Query, ResMut, Resource, Update},
};
use bevy_twin_stick::player::Player;

use crate::{
    gamestate::GameplayMode,
    overworld::{BeginNPCEncounter, BeginPlayerEncounter},
    rpg::{get_party_members, Creature, Party},
};

pub struct EncounterPlugin;

#[derive(Resource, Default)]
pub struct BattleData {
    top_party: Party,
    bottom_party: Party,
}

impl Plugin for EncounterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(BattleData::default());
        app.add_systems(Update, (trigger_player_encounter, trigger_npc_encounter));
    }
}

pub fn trigger_player_encounter(
    mut state: ResMut<NextState<GameplayMode>>,
    mut triggers: EventReader<BeginPlayerEncounter>,
    mut battle_data: ResMut<BattleData>,
    creatures: Query<(Entity, &Parent), Has<Creature>>,
) {
    for trigger in triggers.iter() {
        println!("Triggering player encounter");
        state.set(GameplayMode::Encounter);
        battle_data.bottom_party = get_party_members(trigger.0, &creatures);
        battle_data.top_party = get_party_members(trigger.1, &creatures);

        println!(
            "Started fight between [{}] and [{}]",
            battle_data
                .top_party
                .members
                .iter()
                .map(|x| format!("{}", x.index()))
                .reduce(|acc, i| acc + "," + &i)
                .unwrap(),
            battle_data
                .bottom_party
                .members
                .iter()
                .map(|x| format!("{}", x.index()))
                .reduce(|acc, i| acc + "," + &i)
                .unwrap()
        )
    }
}

pub fn trigger_npc_encounter(mut triggers: EventReader<BeginNPCEncounter>) {
    for _ in triggers.iter() {
        println!(
            "Triggered a fight between two NPCs. This does nothing and can
        remain doing nothing unless we want to have NPCs fight each other.
        Only added because I had to do something with NPC <-> NPC collisions."
        );
    }
}
