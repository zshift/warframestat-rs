/*
 * WarframeStat.us API
 *
 * Simple API for data from the game Warframe. [Parser Docs](https://wfcd.github.io/warframe-worldstate-parser/) [Items Types](https://github.com/WFCD/warframe-items/blob/master/index.d.ts) 
 *
 * The version of the OpenAPI document: living
 * Contact: tobiah@protonmail.com
 * Generated by: https://openapi-generator.tech
 */

/// RivenStatistic : A colleciton of rivens about a specific weapon's riven sales



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RivenStatistic {
    #[serde(rename = "itemType", skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
    #[serde(rename = "compatability", skip_serializing_if = "Option::is_none")]
    pub compatability: Option<String>,
    #[serde(rename = "rerolled", skip_serializing_if = "Option::is_none")]
    pub rerolled: Option<bool>,
    #[serde(rename = "avg", skip_serializing_if = "Option::is_none")]
    pub avg: Option<f32>,
    #[serde(rename = "stddev", skip_serializing_if = "Option::is_none")]
    pub stddev: Option<f32>,
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<f32>,
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<f32>,
    #[serde(rename = "pop", skip_serializing_if = "Option::is_none")]
    pub pop: Option<f32>,
    #[serde(rename = "median", skip_serializing_if = "Option::is_none")]
    pub median: Option<f32>,
}

impl RivenStatistic {
    /// A colleciton of rivens about a specific weapon's riven sales
    pub fn new() -> RivenStatistic {
        RivenStatistic {
            item_type: None,
            compatability: None,
            rerolled: None,
            avg: None,
            stddev: None,
            min: None,
            max: None,
            pop: None,
            median: None,
        }
    }
}


