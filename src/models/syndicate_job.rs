/*
 * WarframeStat.us API
 *
 * Simple API for data from the game Warframe. [Parser Docs](https://wfcd.github.io/warframe-worldstate-parser/) [Items Types](https://github.com/WFCD/warframe-items/blob/master/index.d.ts) 
 *
 * The version of the OpenAPI document: living
 * Contact: tobiah@protonmail.com
 * Generated by: https://openapi-generator.tech
 */

/// SyndicateJob : A Job for a syndicate. Often called a bounty.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct SyndicateJob {
    /// Timestamp for when the job becomes active
    #[serde(rename = "activation", skip_serializing_if = "Option::is_none")]
    pub activation: Option<String>,
    /// Timestamp for when the job becomes inactive
    #[serde(rename = "expiry", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<String>,
    /// Reward Pool for the job
    #[serde(rename = "rewardPool", skip_serializing_if = "Option::is_none")]
    pub reward_pool: Option<Vec<String>>,
    /// What type of Job (Bounty) it is
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    /// Array of enemy levels
    #[serde(rename = "enemyLevels", skip_serializing_if = "Option::is_none")]
    pub enemy_levels: Option<Vec<f32>>,
    /// Stages of standing rewards.
    #[serde(rename = "standingStages", skip_serializing_if = "Option::is_none")]
    pub standing_stages: Option<Vec<f32>>,
    /// Minimum Mastery Rank required to perform a job.
    #[serde(rename = "minMR", skip_serializing_if = "Option::is_none")]
    pub min_mr: Option<f32>,
}

impl SyndicateJob {
    /// A Job for a syndicate. Often called a bounty.
    pub fn new() -> SyndicateJob {
        SyndicateJob {
            activation: None,
            expiry: None,
            reward_pool: None,
            _type: None,
            enemy_levels: None,
            standing_stages: None,
            min_mr: None,
        }
    }
}


