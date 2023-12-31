use bevy::{
    ecs::query::Has,
    prelude::{
        in_state, Entity, EventReader, Handle, Image, IntoSystemConfigs, NextState, Parent, Plugin,
        Query, ResMut, Resource, Update,
    },
    utils::HashMap,
};
use bevy_egui::{egui, EguiContexts};

use crate::{
    gamestate::GameplayMode,
    overworld::{BeginNPCEncounter, BeginPlayerEncounter},
    rpg::{get_party_members, Creature, Party},
};

use self::ui::{enemy_creatures_ui, player_creatures_ui};

pub mod ui;

pub struct EncounterPlugin;

#[derive(Resource, Default)]
pub struct BattleData {
    top_party: Party,
    top_party_bot_ap: usize,
    bottom_party: Party,
    bottom_party_bot_ap: usize,
    battler_images: HashMap<Entity, egui::TextureId>,
}

impl Plugin for EncounterPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(BattleData::default());
        app.add_systems(Update, (trigger_player_encounter, trigger_npc_encounter));
        app.add_systems(
            Update,
            (player_creatures_ui, enemy_creatures_ui).run_if(in_state(GameplayMode::Encounter)),
        );
    }
}

pub fn trigger_player_encounter(
    mut state: ResMut<NextState<GameplayMode>>,
    mut triggers: EventReader<BeginPlayerEncounter>,
    mut battle_data: ResMut<BattleData>,
    creatures: Query<(Entity, &Parent), Has<Creature>>,
    textures: Query<&Handle<Image>, Has<Creature>>,
    mut ctx: EguiContexts,
) {
    for trigger in triggers.iter() {
        println!("Triggering player encounter");
        state.set(GameplayMode::Encounter);
        battle_data.bottom_party = get_party_members(trigger.1, &creatures);
        battle_data.top_party = get_party_members(trigger.0, &creatures);
        battle_data.battler_images = HashMap::new();
        let mut battler_ids = battle_data.top_party.members.clone();
        battler_ids.extend(battle_data.bottom_party.members.clone());
        battler_ids.iter().for_each(|x| {
            battle_data
                .battler_images
                .insert(x.clone(), ctx.add_image(textures.get(*x).unwrap().clone()));
        });

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
