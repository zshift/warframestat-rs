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
pub struct Conclave {
    #[serde(rename = "modes")]
    pub modes: Box<crate::models::ConclaveModes>,
    #[serde(rename = "categories")]
    pub categories: Box<crate::models::ConclaveCategories>,
}

impl Conclave {
    pub fn new(modes: crate::models::ConclaveModes, categories: crate::models::ConclaveCategories) -> Conclave {
        Conclave {
            modes: Box::new(modes),
            categories: Box::new(categories),
        }
    }
}


