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
pub struct CetusCycle {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "expiry")]
    pub expiry: String,
    #[serde(rename = "activation", skip_serializing_if = "Option::is_none")]
    pub activation: Option<String>,
    #[serde(rename = "isDay")]
    pub is_day: bool,
    /// Describes the current time. e.g. \"day\" or \"night\"
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "timeLeft")]
    pub time_left: String,
    #[serde(rename = "isCetus")]
    pub is_cetus: bool,
    /// A short description of the remaining time until the next day/night change.
    #[serde(rename = "shortString", skip_serializing_if = "Option::is_none")]
    pub short_string: Option<String>,
}

impl CetusCycle {
    pub fn new(id: String, expiry: String, is_day: bool, time_left: String, is_cetus: bool) -> CetusCycle {
        CetusCycle {
            id,
            expiry,
            activation: None,
            is_day,
            state: None,
            time_left,
            is_cetus,
            short_string: None,
        }
    }
}


