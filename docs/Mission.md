# Mission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**reward** | [**crate::models::Reward**](reward.md) |  | 
**node** | **String** |  | 
**faction** | **String** |  | 
**max_enemy_level** | **f32** |  | 
**min_enemy_level** | **f32** |  | 
**max_wave_num** | **f32** |  | 
**_type** | **String** |  | 
**nightmare** | **bool** |  | 
**archwing_required** | **bool** | Whether or not an Archwing is required for participating in the mision. | 
**is_sharkwing** | Option<**bool**> | Whether or not the mission takes place in a submerssible mission. | [optional]
**enemy_spec** | Option<**String**> | Enemy specification for the mission | [optional]
**level_override** | Option<**String**> | Override for the map on this mission | [optional]
**advanced_spawners** | Option<**Vec<String>**> | Array of strings denoting extra spawners for a mission | [optional]
**required_items** | Option<**Vec<String>**> | Items required to enter the mission | [optional]
**consume_required_items** | Option<**bool**> | Whether or not the required items are consumed | [optional]
**leaders_always_allowed** | Option<**bool**> | Whether or not leaders are always allowed | [optional]
**level_auras** | Option<**Vec<String>**> | Affectors for this mission | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


