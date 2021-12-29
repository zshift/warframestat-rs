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
pub struct SteelPathAllOf {
    #[serde(rename = "activation", skip_serializing_if = "Option::is_none")]
    pub activation: Option<String>,
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[serde(rename = "currentReward", skip_serializing_if = "Option::is_none")]
    pub current_reward: Option<Box<crate::models::SimpleReward>>,
    #[serde(rename = "remaining", skip_serializing_if = "Option::is_none")]
    pub remaining: Option<String>,
    #[serde(rename = "rotation", skip_serializing_if = "Option::is_none")]
    pub rotation: Option<Vec<crate::models::SimpleReward>>,
    #[serde(rename = "evergreens", skip_serializing_if = "Option::is_none")]
    pub evergreens: Option<Vec<crate::models::SimpleReward>>,
    #[serde(rename = "incursions", skip_serializing_if = "Option::is_none")]
    pub incursions: Option<Box<crate::models::WorldstateObject>>,
}

impl SteelPathAllOf {
    pub fn new() -> SteelPathAllOf {
        SteelPathAllOf {
            activation: None,
            expiry: None,
            current_reward: None,
            remaining: None,
            rotation: None,
            evergreens: None,
            incursions: None,
        }
    }
}


