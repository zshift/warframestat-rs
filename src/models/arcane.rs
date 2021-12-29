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
pub struct Arcane {
    #[serde(rename = "regex")]
    pub regex: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "effect")]
    pub effect: String,
    #[serde(rename = "rarity")]
    pub rarity: String,
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "thumbnail")]
    pub thumbnail: String,
    #[serde(rename = "info")]
    pub info: String,
}

impl Arcane {
    pub fn new(regex: String, name: String, effect: String, rarity: String, location: String, thumbnail: String, info: String) -> Arcane {
        Arcane {
            regex,
            name,
            effect,
            rarity,
            location,
            thumbnail,
            info,
        }
    }
}

