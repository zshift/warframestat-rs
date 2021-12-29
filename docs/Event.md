# Event

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**activation** | Option<**String**> | Timestamp of when the event started | [optional]
**expiry** | Option<**String**> | Timestamp of when the event ends | [optional]
**maximum_score** | Option<**f32**> | Maximum score to complete the event | [optional]
**current_score** | Option<**f32**> | The current score for the event | [optional]
**small_interval** | Option<**f32**> | Interval for the first goal | [optional]
**large_interval** | Option<**f32**> | Interval for the second intermediate score | [optional]
**faction** | Option<[**crate::models::Faction**](faction.md)> |  | [optional]
**description** | Option<**String**> | The description or \"subtitle\" for the event. | [optional]
**tooltip** | Option<**String**> | Tooltip for the event | [optional]
**node** | Option<**String**> | Node that the event is taking place on | [optional]
**concurrent_nodes** | Option<**Vec<String>**> | Nodes that the event is happening concurrently on | [optional]
**victim_node** | Option<**String**> | Node that is being attacked & defended in the event. | [optional]
**score_loc_tag** | Option<**String**> | Localized tag for the event score | [optional]
**rewards** | Option<[**Vec<crate::models::Reward>**](reward.md)> |  | [optional]
**expired** | Option<**bool**> | Whether or not the event is expired | [optional]
**health** | Option<**f32**> | Amount of health remaining for the target | [optional]
**affiliated_with** | Option<[**crate::models::Syndicate**](syndicate.md)> |  | [optional]
**jobs** | Option<[**Vec<crate::models::SyndicateJob>**](syndicateJob.md)> |  | [optional]
**interim_steps** | Option<[**Vec<crate::models::EventInterimSteps>**](event_interimSteps.md)> | Interim steps, marking progress towards the final goal. | [optional]
**progress_steps** | Option<[**crate::models::EventProgressSteps**](event_progressSteps.md)> |  | [optional]
**progress_total** | Option<**f32**> | Total of progressSteps values. | [optional]
**show_total_at_end_of_mission** | Option<**bool**> | Whether or not to show the total score at the end of the mission | [optional]
**is_personal** | Option<**bool**> | Whether or not the event is personal | [optional]
**is_community** | Option<**bool**> | Whether or not the event is communal | [optional]
**region_drops** | Option<**Vec<String>**> | Drops in the area around the event node | [optional]
**archwing_drops** | Option<**Vec<String>**> | Archwing Drops in effect while this event is active | [optional]
**as_string** | Option<**String**> | Attempt to summarize event in a short string. (Do not use). | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Miscellaneous metadata in a string provided by Digital Extremes | [optional]
**completion_bonuses** | Option<**Vec<f32>**> | Completion bonus amounts per-stage | [optional]
**score_var** | Option<**String**> | Internal string used for unknown purpose | [optional]
**alt_expiry** | Option<**String**> | Alternate Expiry. Use unknown. | [optional]
**alt_activation** | Option<**String**> | Alternate Activation. Use unknown. | [optional]
**next_alt** | Option<[**crate::models::EventNextAlt**](event_nextAlt.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


