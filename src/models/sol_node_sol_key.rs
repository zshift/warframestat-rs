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
pub struct SolNodeSolKey {
    #[serde(rename = "enemy")]
    pub enemy: String,
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "value")]
    pub value: String,
}

impl SolNodeSolKey {
    pub fn new(enemy: String, _type: String, value: String) -> SolNodeSolKey {
        SolNodeSolKey {
            enemy,
            _type,
            value,
        }
    }
}


