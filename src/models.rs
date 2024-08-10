use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Response {
    #[serde(rename = "type")]
    pub response_type: String,
    pub format: String,
    pub version: String,
    pub data: HashMap<String, Champion>
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Champion {
    pub version: String,
    pub id: String,
    pub key: String,
    pub name: String,
    pub title: String,
    pub blurb: String,
    pub info: ChampionInfo,
    pub image: Image,
    pub tags: Vec<Tag>,
    #[serde(rename = "partype")]
    pub resource_type: String,
    pub stats: Stats,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct ChampionInfo {
    pub attack: u8,
    pub defense: u8,
    pub magic: u8,
    pub difficulty: u8,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Image {
    pub full: String,
    pub sprite: String,
    pub group: String,
    pub x: u16,
    pub y: u16,
    #[serde(rename = "w")]
    pub width: u16,
    #[serde(rename = "h")]
    pub height: u16,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Tag {
    Mage,
    Support,
    Marksman,
    Assassin,
    Tank,
    Fighter,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
pub struct Stats {
    #[serde(rename = "movespeed")]
    pub move_speed: f32,
    #[serde(rename = "attackrange")]
    pub attack_range: f32,
    #[serde(rename = "attackdamage")]
    pub attack_damage: f32,
    #[serde(rename = "attackdamageperlevel")]
    pub attack_damage_per_level: f32,
    #[serde(rename = "attackspeed")]
    pub attack_speed: f32,
    #[serde(rename = "attackspeedperlevel")]
    pub attack_speed_per_level: f32,
    #[serde(rename = "crit")]
    pub crit_chance: f32,
    #[serde(rename = "critperlevel")]
    pub crit_chance_per_level: f32,
    pub hp: f32,
    #[serde(rename = "hpperlevel")]
    pub hp_per_level: f32,
    pub mp: f32,
    #[serde(rename = "mpperlevel")]
    pub mp_per_level: f32,
    #[serde(rename = "hpregen")]
    pub hp_regen: f32,
    #[serde(rename = "hpregenperlevel")]
    pub hp_regen_per_level: f32,
    #[serde(rename = "mpregen")]
    pub mp_regen: f32,
    #[serde(rename = "mpregenperlevel")]
    pub mp_regen_per_level: f32,
    pub armor: f32,
    #[serde(rename = "armorperlevel")]
    pub armor_per_level: f32,
    #[serde(rename = "spellblock")]
    pub magic_resist: f32,
    #[serde(rename = "spellblockperlevel")]
    pub magic_resist_per_level: f32,
}