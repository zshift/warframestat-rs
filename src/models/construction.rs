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
pub struct Construction {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "fomorianProgress")]
    pub fomorian_progress: String,
    #[serde(rename = "razorbackProgress")]
    pub razorback_progress: String,
    #[serde(rename = "unknownProgress")]
    pub unknown_progress: String,
}

impl Construction {
    pub fn new(id: String, fomorian_progress: String, razorback_progress: String, unknown_progress: String) -> Construction {
        Construction {
            id,
            fomorian_progress,
            razorback_progress,
            unknown_progress,
        }
    }
}


