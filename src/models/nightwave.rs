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
pub struct Nightwave {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "activation", skip_serializing_if = "Option::is_none")]
    pub activation: Option<String>,
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[serde(rename = "params", skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    #[serde(rename = "rewardTypes", skip_serializing_if = "Option::is_none")]
    pub reward_types: Option<Vec<String>>,
    #[serde(rename = "season", skip_serializing_if = "Option::is_none")]
    pub season: Option<f32>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<f32>,
    #[serde(rename = "possibleChallenges", skip_serializing_if = "Option::is_none")]
    pub possible_challenges: Option<Vec<crate::models::NightwaveChallenge>>,
    #[serde(rename = "activeChallenges", skip_serializing_if = "Option::is_none")]
    pub active_challenges: Option<Vec<crate::models::NightwaveChallenge>>,
}

impl Nightwave {
    pub fn new() -> Nightwave {
        Nightwave {
            id: None,
            activation: None,
            expiry: None,
            params: None,
            reward_types: None,
            season: None,
            tag: None,
            phase: None,
            possible_challenges: None,
            active_challenges: None,
        }
    }
}

