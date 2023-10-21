use crate::{
    assets::{add_texture, ImageAssets},
    gamestate::AppState,
    OverworldBound,
};
use bevy::{
    core::Name,
    ecs::query::Has,
    prelude::{
        Commands, Component, Entity, Event, EventReader, EventWriter, Handle, Image, OnEnter,
        Plugin, Query, Res, Startup, Update, Vec2,
    },
    sprite::{Anchor, Sprite},
};
use bevy_composable::app_impl::ComplexSpawnable;
use bevy_composable::{
    tree::{ComponentTree, EntityCommandSet},
    CT,
};

use bevy_twin_stick::{
    actors::{ActorBundle, Tracking},
    bevy_mod_transform2d::transform2d::Transform2d,
    bevy_rapier2d::prelude::CollisionEvent,
    player::Player,
    stats::Speed,
};
use bevy_twin_stick::{bevy_rapier2d::prelude::ActiveEvents, transform2d_mods::Sprite2dBundle};

#[derive(Component)]
pub struct EncounterTrigger;
#[derive(Component)]
pub struct Enemy;

#[derive(Event)]
pub struct EncounterCollision(Entity, Entity);
#[derive(Event)]
pub struct BeginPlayerEncounter(Entity);
#[derive(Event)]
pub struct BeginNPCEncounter(Entity, Entity);

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(AppState::InGame), debug_stage_setup);

        app.add_event::<BeginNPCEncounter>();
        app.add_event::<BeginPlayerEncounter>();
        app.add_systems(Update, trigger_encounter_on_touch);
    }
}

pub fn debug_stage_setup(mut commands: Commands, sprites: Res<ImageAssets>) {
    commands.spawn_complex(enemy_tree(&sprites.org1) + shift_pos(Vec2::new(0., 90.)));
}

pub fn trigger_encounter_on_touch(
    mut collision_events: EventReader<CollisionEvent>,
    player: Query<Entity, Has<Player>>,
    encounter_trigger: Query<Entity, Has<EncounterTrigger>>,
    mut begin_player_encounter: EventWriter<BeginPlayerEncounter>,
) {
    for collision_event in collision_events.iter() {
        println!("Collision with encounter trigger");
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            if let (Ok(_), Ok(_)) = (player.get(*e1), encounter_trigger.get(*e2)) {
                begin_player_encounter.send(BeginPlayerEncounter(*e2));
            } else if let (Ok(_), Ok(_)) = (player.get(*e2), encounter_trigger.get(*e1)) {
                begin_player_encounter.send(BeginPlayerEncounter(*e1));
            }
        }
    }
}

///////////////////////////////////////
/////////  Component Trees ////////////
///////////////////////////////////////

fn enemy_tree(sprite: &Handle<Image>) -> ComponentTree {
    overworld_actor("Debug Enemy", 400., sprite) + CT!(EncounterTrigger)
}

/// Holds all the default components for acting entities in the overworld.
pub fn overworld_actor(name: &'static str, speed: f32, tex: &Handle<Image>) -> ComponentTree {
    CT!(
        Name::new(name),
        ActorBundle::default(),
        Speed(speed),
        OverworldBound,
        ActiveEvents::COLLISION_EVENTS
    ) + add_texture(tex)
}

/// Returns a CT that can be used to easily shift the starting position of an entity by overwriting
/// the default transform provided by a previous CT.
pub fn shift_pos(pos: impl Into<Vec2>) -> ComponentTree {
    let new_pos = pos.into();
    CT!(Transform2d::from_translation(new_pos))
}

/// Returns a CT that can be used to easily shift the tracking state of an entity by overwriting
/// the default tracking provided by a previous CT.
pub fn shift_tracking(tracking: Option<Entity>) -> ComponentTree {
    CT!(Tracking(tracking))
}
