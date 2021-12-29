# Invasion

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | unique identifier for this object/event/thing | 
**activation** | **String** | ISO-8601 formatted timestamp for when the event began | 
**expiry** | Option<**String**> | ISO-8601 formatted timestamp for when the event is expected to end | [optional]
**attacker** | Option<[**serde_json::Value**](serde_json::Value.md)> |  | [optional]
**attacker_reward** | Option<[**serde_json::Value**](serde_json::Value.md)> |  | [optional]
**attacking_faction** | **String** |  | 
**completed** | **bool** | Whether or not this invasion is \"over\" | 
**completion** | **f32** | percentage complete as a float value | 
**count** | **f32** | How many fights have happened. | 
**defender** | Option<[**serde_json::Value**](serde_json::Value.md)> |  | [optional]
**defender_reward** | Option<[**serde_json::Value**](serde_json::Value.md)> |  | [optional]
**defending_faction** | **String** |  | 
**desc** | **String** | description of invasion | 
**eta** | **String** | time string showing approximate time to the end of the invasion | 
**node** | **String** | localized Node name | 
**node_key** | Option<**String**> | i18n key for matching node (always english translation) | [optional]
**required_runs** | **f32** | How many runs of this mission are needed to qualify for the reward | 
**reward_types** | Option<[**Vec<crate::models::RewardType>**](rewardType.md)> |  | [optional]
**start_string** | Option<**String**> |  | [optional]
**vs_infestation** | **bool** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


