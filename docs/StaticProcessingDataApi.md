# \StaticProcessingDataApi

All URIs are relative to *https://api.warframestat.us*

Method | HTTP request | Description
------------- | ------------- | -------------
[**arcanes_get**](StaticProcessingDataApi.md#arcanes_get) | **GET** /arcanes | Get Arcane Enhancement Data
[**conclave_get**](StaticProcessingDataApi.md#conclave_get) | **GET** /conclave | Get conclave challenge data
[**events_get**](StaticProcessingDataApi.md#events_get) | **GET** /events | Get Event-specific Data
[**factions_get**](StaticProcessingDataApi.md#factions_get) | **GET** /factions | Get Faction translation information.
[**fissure_modifiers_get**](StaticProcessingDataApi.md#fissure_modifiers_get) | **GET** /fissureModifiers | Get Fissure Modifier translation data.
[**items_get**](StaticProcessingDataApi.md#items_get) | **GET** /items | Get item data.
[**items_query_get**](StaticProcessingDataApi.md#items_query_get) | **GET** /items/{query} | Get item data.
[**languages_get**](StaticProcessingDataApi.md#languages_get) | **GET** /languages | Get Language strings for Warframe.
[**locales_get**](StaticProcessingDataApi.md#locales_get) | **GET** /locales | Full list of supported locales
[**mission_types_get**](StaticProcessingDataApi.md#mission_types_get) | **GET** /missionTypes | Get MissionType Translation Keys
[**mods_get**](StaticProcessingDataApi.md#mods_get) | **GET** /mods | Get Mod data.
[**mods_query_get**](StaticProcessingDataApi.md#mods_query_get) | **GET** /mods/{query} | Get item data.
[**mods_search_query_get**](StaticProcessingDataApi.md#mods_search_query_get) | **GET** /mods/search/{query} | Get Warframe Items data
[**operation_types_get**](StaticProcessingDataApi.md#operation_types_get) | **GET** /operationTypes | Get operation types data.
[**persistent_enemy_get**](StaticProcessingDataApi.md#persistent_enemy_get) | **GET** /persistentEnemy | Get Persistent Enemy translation data.
[**sol_nodes_get**](StaticProcessingDataApi.md#sol_nodes_get) | **GET** /solNodes | Get Sol Node information and translation data.
[**sortie_get**](StaticProcessingDataApi.md#sortie_get) | **GET** /sortie | Get Sortie translation information.
[**syndicates_get**](StaticProcessingDataApi.md#syndicates_get) | **GET** /syndicates | Get Syndicate translation data.
[**tutorials_get**](StaticProcessingDataApi.md#tutorials_get) | **GET** /tutorials | Get Tutorials Data
[**upgrade_types_get**](StaticProcessingDataApi.md#upgrade_types_get) | **GET** /upgradeTypes | Get upgrade types data for global upgrades.
[**warframes_get**](StaticProcessingDataApi.md#warframes_get) | **GET** /warframes | Get Warframe specs and data, such as polarities defenses, and profile.
[**warframes_query_get**](StaticProcessingDataApi.md#warframes_query_get) | **GET** /warframes/{query} | Get Warframe specs and data, such as polarity and defenses, and profile based on the query. Single result
[**weapons_get**](StaticProcessingDataApi.md#weapons_get) | **GET** /weapons | Get Weapon data and statistics.
[**weapons_query_get**](StaticProcessingDataApi.md#weapons_query_get) | **GET** /weapons/{query} | Get Weapon specs and data, such as polarity based on the query. Single result
[**weapons_search_query_get**](StaticProcessingDataApi.md#weapons_search_query_get) | **GET** /weapons/search/{query} | Get Weapon data and statistics based on the query.



## arcanes_get

> crate::models::Arcane arcanes_get()
Get Arcane Enhancement Data

Available Arcane Enhancements

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Arcane**](arcane.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conclave_get

> crate::models::Conclave conclave_get()
Get conclave challenge data

Data about conclave challenges

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Conclave**](conclave.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_get

> serde_json::Value events_get()
Get Event-specific Data

Data about events

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## factions_get

> crate::models::Factions factions_get()
Get Faction translation information.

Strings for translating faction identifiers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Factions**](factions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fissure_modifiers_get

> crate::models::FissureModifiers fissure_modifiers_get()
Get Fissure Modifier translation data.

Fissure translation identifiers

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::FissureModifiers**](fissureModifiers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## items_get

> Vec<crate::models::ItemsFields> items_get(only, remove)
Get item data.

Item information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**only** | Option<**String**> | Keys to keep on the object. Comma separated for multiple |  |
**remove** | Option<**String**> | Keys to remove on the object. Comma separated for multiple |  |

### Return type

[**Vec<crate::models::ItemsFields>**](itemsFields.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## items_query_get

> crate::models::Item items_query_get(query, only, remove, by)
Get item data.

Item information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |
**only** | Option<**String**> | Keys to keep on the object. Comma separated for multiple |  |
**remove** | Option<**String**> | Keys to remove on the object. Comma separated for multiple |  |
**by** | Option<**String**> | Key to search by on the object |  |

### Return type

[**crate::models::Item**](item.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## languages_get

> crate::models::Languages languages_get()
Get Language strings for Warframe.

Get language strings to assist translation. (Prefer the /languages/search/:query route)

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Languages**](languages.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## locales_get

> Vec<crate::models::Language> locales_get()
Full list of supported locales

Locales supported by the API

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Language>**](language.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mission_types_get

> crate::models::MissionTypes mission_types_get()
Get MissionType Translation Keys

Mission Type information to aid translating identifiers

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::MissionTypes**](missionTypes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mods_get

> Vec<crate::models::ModelMod> mods_get(only, remove)
Get Mod data.

Item information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**only** | Option<**String**> | Keys to keep on the object. Comma separated for multiple |  |
**remove** | Option<**String**> | Keys to remove on the object. Comma separated for multiple |  |

### Return type

[**Vec<crate::models::ModelMod>**](mod.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mods_query_get

> crate::models::ModelMod mods_query_get(query, only, remove, by)
Get item data.

Mod information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |
**only** | Option<**String**> | Keys to keep on the object. Comma separated for multiple |  |
**remove** | Option<**String**> | Keys to remove on the object. Comma separated for multiple |  |
**by** | Option<**String**> | Key to search by on the object |  |

### Return type

[**crate::models::ModelMod**](mod.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mods_search_query_get

> Vec<crate::models::ModelMod> mods_search_query_get(query, only, remove, by)
Get Warframe Items data

Item information, such as name, unique name, type, and image name.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |
**only** | Option<**String**> | Keys to keep on the object. Comma separated for multiple |  |
**remove** | Option<**String**> | Keys to remove on the object. Comma separated for multiple |  |
**by** | Option<**String**> | Key to search by on the object |  |

### Return type

[**Vec<crate::models::ModelMod>**](mod.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## operation_types_get

> crate::models::OperationTypes operation_types_get()
Get operation types data.

Operation Types information to aid translating identifiers for global upgrades

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::OperationTypes**](operationTypes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_enemy_get

> Vec<serde_json::Value> persistent_enemy_get()
Get Persistent Enemy translation data.

Persistent Enemy translation information for aiding translation of identifiers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sol_nodes_get

> crate::models::SolNode sol_nodes_get()
Get Sol Node information and translation data.

Sol Node translation information for aiding the translation of identifiers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SolNode**](solNode.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sortie_get

> crate::models::SortieData sortie_get()
Get Sortie translation information.

Sortie translation information for assisting translation of identifiers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::SortieData**](sortieData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## syndicates_get

> crate::models::Syndicates syndicates_get()
Get Syndicate translation data.

Information to assist translating syndicate identifiers.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Syndicates**](syndicates.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tutorials_get

> Vec<serde_json::Value> tutorials_get()
Get Tutorials Data

Tutorials data from DE

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_types_get

> crate::models::UpgradeTypes upgrade_types_get()
Get upgrade types data for global upgrades.

Upgrade types for what can be changed by global modifiers, such as double credit weekends.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UpgradeTypes**](upgradeTypes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warframes_get

> Vec<crate::models::Warframe> warframes_get(only, remove)
Get Warframe specs and data, such as polarities defenses, and profile.

Warframe stats and general information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**only** | Option<**String**> | Keys to keep on the object. Comma separated for multiple |  |
**remove** | Option<**String**> | Keys to remove on the object. Comma separated for multiple |  |

### Return type

[**Vec<crate::models::Warframe>**](warframe.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## warframes_query_get

> crate::models::Warframe warframes_query_get(query, by)
Get Warframe specs and data, such as polarity and defenses, and profile based on the query. Single result

Mod information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |
**by** | Option<**String**> | Key to search by on the object |  |

### Return type

[**crate::models::Warframe**](warframe.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## weapons_get

> Vec<crate::models::WeaponsFields> weapons_get(only, remove, by)
Get Weapon data and statistics.

Weapon statistics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**only** | Option<**String**> | Keys to keep on the object. Comma separated for multiple |  |
**remove** | Option<**String**> | Keys to remove on the object. Comma separated for multiple |  |
**by** | Option<**String**> | Key to search by on the object |  |

### Return type

[**Vec<crate::models::WeaponsFields>**](weaponsFields.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## weapons_query_get

> crate::models::Weapon weapons_query_get(query, only, remove, by)
Get Weapon specs and data, such as polarity based on the query. Single result

Mod information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |
**only** | Option<**String**> | Keys to keep on the object. Comma separated for multiple |  |
**remove** | Option<**String**> | Keys to remove on the object. Comma separated for multiple |  |
**by** | Option<**String**> | Key to search by on the object |  |

### Return type

[**crate::models::Weapon**](weapon.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## weapons_search_query_get

> Vec<crate::models::WeaponsFields> weapons_search_query_get(query, only, remove, by)
Get Weapon data and statistics based on the query.

Weapon statistics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |
**only** | Option<**String**> | Keys to keep on the object. Comma separated for multiple |  |
**remove** | Option<**String**> | Keys to remove on the object. Comma separated for multiple |  |
**by** | Option<**String**> | Key to search by on the object |  |

### Return type

[**Vec<crate::models::WeaponsFields>**](weaponsFields.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

