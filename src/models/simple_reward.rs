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
pub struct SimpleReward {
    /// Name of current reward
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Resource cost of item
    #[serde(rename = "cost", skip_serializing_if = "Option::is_none")]
    pub cost: Option<f32>,
}

impl SimpleReward {
    pub fn new() -> SimpleReward {
        SimpleReward {
            name: None,
            cost: None,
        }
    }
}

