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
pub struct VoidTrader {
    /// unique identifier for this object/event/thing
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// ISO-8601 formatted timestamp for when the event began
    #[serde(rename = "activation", skip_serializing_if = "Option::is_none")]
    pub activation: Option<String>,
    /// ISO-8601 formatted timestamp for when the event is expected to end
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    #[serde(rename = "character")]
    pub character: String,
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "inventory")]
    pub inventory: Vec<crate::models::VoidTraderAllOfInventory>,
    #[serde(rename = "psId")]
    pub ps_id: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "startString")]
    pub start_string: String,
    #[serde(rename = "endString")]
    pub end_string: String,
}

impl VoidTrader {
    pub fn new(character: String, location: String, inventory: Vec<crate::models::VoidTraderAllOfInventory>, ps_id: String, active: bool, start_string: String, end_string: String) -> VoidTrader {
        VoidTrader {
            id: None,
            activation: None,
            expiry: None,
            character,
            location,
            inventory,
            ps_id,
            active,
            start_string,
            end_string,
        }
    }
}


