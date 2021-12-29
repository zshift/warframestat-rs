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
pub struct Arbitration {
    /// When the Arbitration becomes active
    #[serde(rename = "activation", skip_serializing_if = "Option::is_none")]
    pub activation: Option<String>,
    /// When the Arbitration becomes inactive
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// Plain name for the node
    #[serde(rename = "node", skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    #[serde(rename = "enemy", skip_serializing_if = "Option::is_none")]
    pub enemy: Option<crate::models::Faction>,
    /// Mission type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Whether or not this mission requires archwing
    #[serde(rename = "archwing", skip_serializing_if = "Option::is_none")]
    pub archwing: Option<bool>,
    /// Whether or not this mission requires sharkwing
    #[serde(rename = "sharkwing", skip_serializing_if = "Option::is_none")]
    pub sharkwing: Option<bool>,
}

impl Arbitration {
    pub fn new() -> Arbitration {
        Arbitration {
            activation: None,
            expiry: None,
            node: None,
            enemy: None,
            _type: None,
            archwing: None,
            sharkwing: None,
        }
    }
}


