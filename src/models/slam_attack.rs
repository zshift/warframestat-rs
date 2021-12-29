/*
 * WarframeStat.us API
 *
 * Simple API for data from the game Warframe. [Parser Docs](https://wfcd.github.io/warframe-worldstate-parser/) [Items Types](https://github.com/WFCD/warframe-items/blob/master/index.d.ts) 
 *
 * The version of the OpenAPI document: living
 * Contact: tobiah@protonmail.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SlamAttack {
    #[serde(rename = "damage", skip_serializing_if = "Option::is_none")]
    pub damage: Option<f32>,
    #[serde(rename = "radial", skip_serializing_if = "Option::is_none")]
    pub radial: Option<Box<crate::models::SlamAttackRadial>>,
}

impl SlamAttack {
    pub fn new() -> SlamAttack {
        SlamAttack {
            damage: None,
            radial: None,
        }
    }
}

