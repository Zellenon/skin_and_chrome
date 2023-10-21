use bevy::asset::AssetServer;
use bevy::prelude::{Handle, Image, Plugin, Resource};
use bevy::sprite::{Anchor, Sprite};
use bevy_asset_loader::prelude::*;
use bevy_composable::tree::ComponentTree;
use bevy_composable::tree::EntityCommandSet;
use bevy_composable::CT;
use bevy_twin_stick::transform2d_mods::Sprite2dBundle;

use crate::gamestate::AppState;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_loading_state(
            LoadingState::new(AppState::LoadingAssets).continue_to_state(AppState::MainMenu),
        )
        .add_collection_to_loading_state::<_, ImageAssets>(AppState::LoadingAssets);
    }
}

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "img/org1.png")]
    pub org1: Handle<Image>,
    #[asset(path = "img/org2_0001.png")]
    pub org2: Handle<Image>,
    #[asset(path = "img/robo1.png")]
    pub robo1: Handle<Image>,
    #[asset(path = "img/robo2blood.png")]
    pub robo2blood: Handle<Image>,
}

pub fn add_texture(tex: &Handle<Image>) -> ComponentTree {
    let tex = tex.clone();
    CT!(Sprite2dBundle {
        sprite: Sprite {
            anchor: Anchor::Center,
            ..Default::default()
        },
        texture: tex.clone(),
        ..Default::default()
    })
}
