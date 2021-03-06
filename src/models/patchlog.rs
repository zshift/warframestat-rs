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
pub struct Patchlog {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "additions", skip_serializing_if = "Option::is_none")]
    pub additions: Option<String>,
    #[serde(rename = "changes", skip_serializing_if = "Option::is_none")]
    pub changes: Option<String>,
    #[serde(rename = "fixes", skip_serializing_if = "Option::is_none")]
    pub fixes: Option<String>,
}

impl Patchlog {
    pub fn new() -> Patchlog {
        Patchlog {
            name: None,
            date: None,
            url: None,
            additions: None,
            changes: None,
            fixes: None,
        }
    }
}


