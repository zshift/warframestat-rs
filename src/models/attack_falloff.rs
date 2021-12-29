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
pub struct AttackFalloff {
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<f32>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<f32>,
    #[serde(rename = "reduction", skip_serializing_if = "Option::is_none")]
    pub reduction: Option<f32>,
}

impl AttackFalloff {
    pub fn new() -> AttackFalloff {
        AttackFalloff {
            start: None,
            end: None,
            reduction: None,
        }
    }
}

