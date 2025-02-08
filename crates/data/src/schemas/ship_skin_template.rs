use std::collections::HashMap;
use serde::Deserialize;
use serde_json::Value;
use crate::schemas::{
    deserialize_as_vec,
    deserialize_as_hashmap,
    deserialize_bool_with_empty_as_false
};

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ShipSkinTemplate(pub HashMap<String, ShipSkinTemplateEntity>);

#[derive(Debug, Deserialize)]
pub struct ShipSkinTemplateEntity {
    pub bg: String,
    pub bg_sp: String,
    pub bgm: String,
    pub bound_bone: HashMap<String, Value>,
    pub desc: String,
    pub fx_container: Vec<Vec<f64>>,
    pub group_index: i32,
    pub gyro: i32,
    pub hand_id: i32,
    pub id: u32,
    pub illustrator: i32,
    pub illustrator2: i32,
    #[serde(deserialize_with = "deserialize_as_vec")]
    pub l2d_animations: Vec<String>,
    #[serde(deserialize_with = "deserialize_as_vec")]
    pub l2d_drag_rate: Vec<f32>,
    pub l2d_ignore_drag: i64,
    #[serde(deserialize_with = "deserialize_as_hashmap")]
    pub l2d_para_range: HashMap<String, Vec<f32>>,
    #[serde(deserialize_with = "deserialize_as_hashmap")]
    pub l2d_se: HashMap<String, Vec<Value>>,
    #[serde(deserialize_with = "deserialize_as_hashmap")]
    pub l2d_voice_calibrate: HashMap<String, Value>,
    pub lip_smoothing: i32,
    pub lip_sync_gain: i32,
    pub live2d_offset: Vec<f32>, // Value
    #[serde(deserialize_with = "deserialize_as_vec")]
    pub live2d_offset_profile: Vec<i32>,
    #[serde(rename = "main_UI_FX")]
    pub main_ui_fx: String,
    pub name: String,
    pub painting: String,
    pub prefab: String,
    pub rarity_bg: String,
    pub ship_group: u32,
    #[serde(deserialize_with = "deserialize_as_vec")]
    pub ship_l2d_id: Vec<i32>,
    pub shop_id: u32,
    pub shop_type_id: i32,
    pub show_skin: String,
    pub skin_type: i32,
    pub smoke: Value,
    pub special_effects: Value,
    #[serde(deserialize_with = "deserialize_bool_with_empty_as_false")]
    pub spine_action_offset: bool,
    #[serde(deserialize_with = "deserialize_as_vec")]
    pub spine_offset: Vec<Vec<f32>>,
    pub tag: Vec<i32>,
    #[serde(deserialize_with = "deserialize_as_vec")]
    pub time: Vec<Vec<i32>>,
    pub voice_actor: i32,
    pub voice_actor_2: i32,
}
