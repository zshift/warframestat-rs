# \SearchableApi

All URIs are relative to *https://api.warframestat.us*

Method | HTTP request | Description
------------- | ------------- | -------------
[**arcanes_search_query_get**](SearchableApi.md#arcanes_search_query_get) | **GET** /arcanes/search/{query} | Get Arcane Enhancement Data based on the query
[**conclave_search_query_get**](SearchableApi.md#conclave_search_query_get) | **GET** /conclave/search/{query} | Get conclave challenge data based on the query
[**drops_search_query_get**](SearchableApi.md#drops_search_query_get) | **GET** /drops/search/{query} | Get Warframe Drops data
[**events_search_query_get**](SearchableApi.md#events_search_query_get) | **GET** /events/search/{query} | Get Event-specific Data based on the query
[**factions_search_query_get**](SearchableApi.md#factions_search_query_get) | **GET** /factions/search/{query} | Get Faction translation information based on the query.
[**fissure_modifiers_search_query_get**](SearchableApi.md#fissure_modifiers_search_query_get) | **GET** /fissureModifiers/search/{query} | Get Fissure Modifier translation data based on the query.
[**items_search_query_get**](SearchableApi.md#items_search_query_get) | **GET** /items/search/{query} | Get Warframe Items data
[**languages_search_query_get**](SearchableApi.md#languages_search_query_get) | **GET** /languages/search/{query} | Get Language strings for Warframe based on the query.
[**mission_types_search_query_get**](SearchableApi.md#mission_types_search_query_get) | **GET** /missionTypes/search/{query} | Get MissionType Translation Keys based on the query
[**mods_query_get**](SearchableApi.md#mods_query_get) | **GET** /mods/{query} | Get item data.
[**mods_search_query_get**](SearchableApi.md#mods_search_query_get) | **GET** /mods/search/{query} | Get Warframe Items data
[**operation_types_search_query_get**](SearchableApi.md#operation_types_search_query_get) | **GET** /operationTypes/search/{query} | Get operation types data based on the query.
[**persistent_enemy_search_query_get**](SearchableApi.md#persistent_enemy_search_query_get) | **GET** /persistentEnemy/search/{query} | Get Persistent Enemy translation data based on the query.
[**sol_nodes_search_query_get**](SearchableApi.md#sol_nodes_search_query_get) | **GET** /solNodes/search/{query} | Get Sol Node information and translation data based on the query.
[**sortie_search_query_get**](SearchableApi.md#sortie_search_query_get) | **GET** /sortie/search/{query} | Get Sortie translation information based on the query.
[**syndicates_search_query_get**](SearchableApi.md#syndicates_search_query_get) | **GET** /syndicates/search/{query} | Get Syndicate translation data based on the query.
[**tutorials_search_query_get**](SearchableApi.md#tutorials_search_query_get) | **GET** /tutorials/search/{query} | Get Tutorials Data based on the query
[**upgrade_types_search_query_get**](SearchableApi.md#upgrade_types_search_query_get) | **GET** /upgradeTypes/search/{query} | Get upgrade types data for global upgrades based on the query.
[**warframes_query_get**](SearchableApi.md#warframes_query_get) | **GET** /warframes/{query} | Get Warframe specs and data, such as polarity and defenses, and profile based on the query. Single result
[**warframes_search_query_get**](SearchableApi.md#warframes_search_query_get) | **GET** /warframes/search/{query} | Get Warframe specs and data, such as polarities defenses, and profile based on the query.
[**weapons_query_get**](SearchableApi.md#weapons_query_get) | **GET** /weapons/{query} | Get Weapon specs and data, such as polarity based on the query. Single result
[**weapons_search_query_get**](SearchableApi.md#weapons_search_query_get) | **GET** /weapons/search/{query} | Get Weapon data and statistics based on the query.



## arcanes_search_query_get

> crate::models::Arcane arcanes_search_query_get(query)
Get Arcane Enhancement Data based on the query

Available Arcane Enhancements

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**crate::models::Arcane**](arcane.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conclave_search_query_get

> crate::models::Conclave conclave_search_query_get(query)
Get conclave challenge data based on the query

Data about conclave challenges

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**crate::models::Conclave**](conclave.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drops_search_query_get

> Vec<serde_json::Value> drops_search_query_get(query)
Get Warframe Drops data

Percentages for Warframe drops in different areas of the game

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## events_search_query_get

> serde_json::Value events_search_query_get(query)
Get Event-specific Data based on the query

Data about events

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## factions_search_query_get

> crate::models::Factions factions_search_query_get(query)
Get Faction translation information based on the query.

Strings for translating faction identifiers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**crate::models::Factions**](factions.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## fissure_modifiers_search_query_get

> crate::models::FissureModifiers fissure_modifiers_search_query_get(query)
Get Fissure Modifier translation data based on the query.

Fissure translation identifiers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**crate::models::FissureModifiers**](fissureModifiers.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## items_search_query_get

> Vec<crate::models::ItemsFields> items_search_query_get(query, only, remove, by)
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

[**Vec<crate::models::ItemsFields>**](itemsFields.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## languages_search_query_get

> crate::models::Languages languages_search_query_get(query)
Get Language strings for Warframe based on the query.

Get language strings to assist translation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**crate::models::Languages**](languages.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## mission_types_search_query_get

> crate::models::MissionTypes mission_types_search_query_get(query)
Get MissionType Translation Keys based on the query

Mission Type information to aid translating identifiers

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**crate::models::MissionTypes**](missionTypes.md)

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


## operation_types_search_query_get

> crate::models::OperationTypes operation_types_search_query_get(query)
Get operation types data based on the query.

Operation Types information to aid translating identifiers for global upgrades

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**crate::models::OperationTypes**](operationTypes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## persistent_enemy_search_query_get

> Vec<serde_json::Value> persistent_enemy_search_query_get(query)
Get Persistent Enemy translation data based on the query.

Persistent Enemy translation information for aiding translation of identifiers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sol_nodes_search_query_get

> Vec<serde_json::Value> sol_nodes_search_query_get(query)
Get Sol Node information and translation data based on the query.

Sol Node translation information for aiding the translation of identifiers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sortie_search_query_get

> crate::models::SortieData sortie_search_query_get(query)
Get Sortie translation information based on the query.

Sortie translation information for assisting translation of identifiers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**crate::models::SortieData**](sortieData.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## syndicates_search_query_get

> crate::models::Syndicates syndicates_search_query_get(query)
Get Syndicate translation data based on the query.

Information to assist translating syndicate identifiers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**crate::models::Syndicates**](syndicates.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tutorials_search_query_get

> Vec<serde_json::Value> tutorials_search_query_get(query)
Get Tutorials Data based on the query

Tutorials data from DE

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upgrade_types_search_query_get

> crate::models::UpgradeTypes upgrade_types_search_query_get(query)
Get upgrade types data for global upgrades based on the query.

Upgrade types for what can be changed by global modifiers, such as double credit weekends.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |

### Return type

[**crate::models::UpgradeTypes**](upgradeTypes.md)

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


## warframes_search_query_get

> Vec<crate::models::Warframe> warframes_search_query_get(query, only, remove, by)
Get Warframe specs and data, such as polarities defenses, and profile based on the query.

Warframe stats and general information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**query** | **String** | Keyword to search for | [required] |
**only** | Option<**String**> | Keys to keep on the object. Comma separated for multiple |  |
**remove** | Option<**String**> | Keys to remove on the object. Comma separated for multiple |  |
**by** | Option<**String**> | Key to search by on the object |  |

### Return type

[**Vec<crate::models::Warframe>**](warframe.md)

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

