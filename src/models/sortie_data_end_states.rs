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
pub struct SortieDataEndStates {
    #[serde(rename = "regions", skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<crate::models::SortieDataRegions>>,
    #[serde(rename = "bossName")]
    pub boss_name: String,
}

impl SortieDataEndStates {
    pub fn new(boss_name: String) -> SortieDataEndStates {
        SortieDataEndStates {
            regions: None,
            boss_name,
        }
    }
}

