use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ShipDataTemplate(pub HashMap<String, ShipDataTemplateEntity>);

#[derive(Debug, Deserialize)]
pub struct ShipDataTemplateEntity {
    pub airassist_time: Vec<u32>,
    pub buff_list: Vec<u32>,
    pub buff_list_display: Vec<u32>,
    pub can_get_proficency: u32,
    pub energy: u32,
    pub equip_1: Vec<u32>,
    pub equip_2: Vec<u32>,
    pub equip_3: Vec<u32>,
    pub equip_4: Vec<u32>,
    pub equip_5: Vec<u32>,
    pub equip_id_1: u32,
    pub equip_id_2: u32,
    pub equip_id_3: u32,
    pub group_type: u32,
    pub hide_buff_list: Vec<u32>,
    pub id: u32,
    pub max_level: u32,
    pub oil_at_end: u32,
    pub oil_at_start: u32,
    pub specific_type: Vec<String>,
    pub star: u32,
    pub star_max: u32,
    pub strengthen_id: u32,
    pub r#type: u32,
}
