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
pub struct SortieAllOfVariants {
    #[serde(rename = "node")]
    pub node: String,
    #[serde(rename = "boss")]
    pub boss: String,
    #[serde(rename = "missionType")]
    pub mission_type: String,
    #[serde(rename = "planet")]
    pub planet: String,
    #[serde(rename = "modifier")]
    pub modifier: String,
    #[serde(rename = "modifierDescription")]
    pub modifier_description: String,
}

impl SortieAllOfVariants {
    pub fn new(node: String, boss: String, mission_type: String, planet: String, modifier: String, modifier_description: String) -> SortieAllOfVariants {
        SortieAllOfVariants {
            node,
            boss,
            mission_type,
            planet,
            modifier,
            modifier_description,
        }
    }
}


