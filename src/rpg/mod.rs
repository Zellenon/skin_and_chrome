use bevy::ecs::query::Has;
use bevy::prelude::{Children, Entity, Parent, Plugin, Query, Visibility};
use bevy::prelude::{Component, Handle, Image, Name};
use bevy_composable::tree::ComponentTree;
use bevy_composable::tree::EntityCommandSet;
use bevy_composable::CT;
use bevy_stats::systems::StatRegisterable;
use bevy_twin_stick::transform2d_mods::Sprite2dBundle;

use self::stats::Health;

pub mod stats;

pub struct RPGPlugin;

impl Plugin for RPGPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.register_stat::<Health>();
    }
}
pub enum Species {
    Robot,
    Organic,
}

#[derive(Component)]
pub struct Creature;

#[derive(Default)]
pub struct Party {
    pub members: Vec<Entity>,
}

pub fn standard_creature<T: Into<String>>(name: T, sprite: &Handle<Image>) -> ComponentTree {
    let sprite = sprite.clone();
    let name = name.into();
    CT!(
        Name::new(name.clone()),
        Sprite2dBundle {
            texture: sprite.clone(),
            visibility: Visibility::Hidden,
            ..Default::default()
        }
    )
}

pub fn get_party_members(
    parent: Entity,
    creatures: &Query<(Entity, &Parent), Has<Creature>>,
) -> Party {
    Party {
        members: creatures
            .iter()
            .filter(|x| x.1.get() == parent)
            .map(|x| x.0)
            .collect(),
    }
}
