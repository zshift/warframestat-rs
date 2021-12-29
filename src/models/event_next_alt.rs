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
pub struct EventNextAlt {
    /// Next alternate expiry. Use unknown.
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// Next alternate activation. Use unknown.
    #[serde(rename = "activation", skip_serializing_if = "Option::is_none")]
    pub activation: Option<String>,
}

impl EventNextAlt {
    pub fn new() -> EventNextAlt {
        EventNextAlt {
            expiry: None,
            activation: None,
        }
    }
}


