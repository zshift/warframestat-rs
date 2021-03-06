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
pub struct SortieDataBosses {
    #[serde(rename = "SORTIE_BOSS_KELA")]
    pub SORTIE_BOSS_KELA: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_AMBULAS")]
    pub SORTIE_BOSS_AMBULAS: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_TYL")]
    pub SORTIE_BOSS_TYL: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_ALAD")]
    pub SORTIE_BOSS_ALAD: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_RUK")]
    pub SORTIE_BOSS_RUK: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_HYENA")]
    pub SORTIE_BOSS_HYENA: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_KRIL")]
    pub SORTIE_BOSS_KRIL: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_CORRUPTED_VOR")]
    pub SORTIE_BOSS_CORRUPTED_VOR: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_INFALAD")]
    pub SORTIE_BOSS_INFALAD: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_PHORID")]
    pub SORTIE_BOSS_PHORID: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_JACKAL")]
    pub SORTIE_BOSS_JACKAL: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_RAPTOR")]
    pub SORTIE_BOSS_RAPTOR: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_VOR")]
    pub SORTIE_BOSS_VOR: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_HEK")]
    pub SORTIE_BOSS_HEK: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_NEF")]
    pub SORTIE_BOSS_NEF: Box<crate::models::SortieDataBossesSortieBossKela>,
    #[serde(rename = "SORTIE_BOSS_LEPHANTIS")]
    pub SORTIE_BOSS_LEPHANTIS: Box<crate::models::SortieDataBossesSortieBossKela>,
}

impl SortieDataBosses {
    pub fn new(SORTIE_BOSS_KELA: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_AMBULAS: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_TYL: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_ALAD: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_RUK: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_HYENA: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_KRIL: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_CORRUPTED_VOR: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_INFALAD: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_PHORID: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_JACKAL: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_RAPTOR: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_VOR: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_HEK: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_NEF: crate::models::SortieDataBossesSortieBossKela, SORTIE_BOSS_LEPHANTIS: crate::models::SortieDataBossesSortieBossKela) -> SortieDataBosses {
        SortieDataBosses {
            SORTIE_BOSS_KELA: Box::new(SORTIE_BOSS_KELA),
            SORTIE_BOSS_AMBULAS: Box::new(SORTIE_BOSS_AMBULAS),
            SORTIE_BOSS_TYL: Box::new(SORTIE_BOSS_TYL),
            SORTIE_BOSS_ALAD: Box::new(SORTIE_BOSS_ALAD),
            SORTIE_BOSS_RUK: Box::new(SORTIE_BOSS_RUK),
            SORTIE_BOSS_HYENA: Box::new(SORTIE_BOSS_HYENA),
            SORTIE_BOSS_KRIL: Box::new(SORTIE_BOSS_KRIL),
            SORTIE_BOSS_CORRUPTED_VOR: Box::new(SORTIE_BOSS_CORRUPTED_VOR),
            SORTIE_BOSS_INFALAD: Box::new(SORTIE_BOSS_INFALAD),
            SORTIE_BOSS_PHORID: Box::new(SORTIE_BOSS_PHORID),
            SORTIE_BOSS_JACKAL: Box::new(SORTIE_BOSS_JACKAL),
            SORTIE_BOSS_RAPTOR: Box::new(SORTIE_BOSS_RAPTOR),
            SORTIE_BOSS_VOR: Box::new(SORTIE_BOSS_VOR),
            SORTIE_BOSS_HEK: Box::new(SORTIE_BOSS_HEK),
            SORTIE_BOSS_NEF: Box::new(SORTIE_BOSS_NEF),
            SORTIE_BOSS_LEPHANTIS: Box::new(SORTIE_BOSS_LEPHANTIS),
        }
    }
}


