use bevy::reflect::TypePath;
use bevy_stats::{RPGResource, RPGStat};

#[derive(TypePath)]
pub struct Health;
#[derive(TypePath)]
pub struct AP;
#[derive(TypePath)]
pub struct MP;

#[derive(TypePath)]
pub struct Attack;
#[derive(TypePath)]
pub struct Defense;
#[derive(TypePath)]
pub struct Speed;

impl RPGStat for Health {}
impl RPGResource for Health {}

impl RPGStat for AP {}
impl RPGResource for AP {}

impl RPGStat for MP {}
impl RPGResource for MP {}

impl RPGStat for Attack {}
impl RPGStat for Defense {}
impl RPGStat for Speed {}
