# \WorldstateApi

All URIs are relative to *https://api.warframestat.us*

Method | HTTP request | Description
------------- | ------------- | -------------
[**platform_alerts_get**](WorldstateApi.md#platform_alerts_get) | **GET** /{platform}/alerts | Alerts data
[**platform_arbitration_get**](WorldstateApi.md#platform_arbitration_get) | **GET** /{platform}/arbitration | [Unstable] Arbitration data
[**platform_cambion_cycle_get**](WorldstateApi.md#platform_cambion_cycle_get) | **GET** /{platform}/cambionCycle | Get Current Cambion Drift Status
[**platform_cetus_cycle_get**](WorldstateApi.md#platform_cetus_cycle_get) | **GET** /{platform}/cetusCycle | Get Current Cetus Status
[**platform_conclave_challenges_get**](WorldstateApi.md#platform_conclave_challenges_get) | **GET** /{platform}/conclaveChallenges | Get Conclave Challenge Data
[**platform_construction_progress_get**](WorldstateApi.md#platform_construction_progress_get) | **GET** /{platform}/constructionProgress | Get Construction Progress for Fomorians and Razorbacks
[**platform_daily_deals_get**](WorldstateApi.md#platform_daily_deals_get) | **GET** /{platform}/dailyDeals | Daily Deal information from Darvo
[**platform_dark_sectors_get**](WorldstateApi.md#platform_dark_sectors_get) | **GET** /{platform}/darkSectors | Dark Sector occupation and history
[**platform_earth_cycle_get**](WorldstateApi.md#platform_earth_cycle_get) | **GET** /{platform}/earthCycle | Get the current Earth rotation information
[**platform_events_get**](WorldstateApi.md#platform_events_get) | **GET** /{platform}/events | Listing of ongoing events
[**platform_fissures_get**](WorldstateApi.md#platform_fissures_get) | **GET** /{platform}/fissures | Data on current fissures
[**platform_flash_sales_get**](WorldstateApi.md#platform_flash_sales_get) | **GET** /{platform}/flashSales | Current Flash Sales from Darvo
[**platform_get**](WorldstateApi.md#platform_get) | **GET** /{platform} | Get Warframe Worldstate Data for the provided platform
[**platform_global_upgrades_get**](WorldstateApi.md#platform_global_upgrades_get) | **GET** /{platform}/globalUpgrades | Current Global Upgrades
[**platform_invasions_get**](WorldstateApi.md#platform_invasions_get) | **GET** /{platform}/invasions | Invasion Data
[**platform_kuva_get**](WorldstateApi.md#platform_kuva_get) | **GET** /{platform}/kuva | [Unstable] Current Kuva Mission listing
[**platform_news_get**](WorldstateApi.md#platform_news_get) | **GET** /{platform}/news | Current Listing of News items
[**platform_nightwave_get**](WorldstateApi.md#platform_nightwave_get) | **GET** /{platform}/nightwave | Get the current Nightwave state.
[**platform_persistent_enemies_get**](WorldstateApi.md#platform_persistent_enemies_get) | **GET** /{platform}/persistentEnemies | Get Persistent Enemy Data
[**platform_rivens_get**](WorldstateApi.md#platform_rivens_get) | **GET** /{platform}/rivens | Get Riven statistic data
[**platform_rivens_search_query_get**](WorldstateApi.md#platform_rivens_search_query_get) | **GET** /{platform}/rivens/search/{query} | Get Riven statistic data
[**platform_sentient_outposts_get**](WorldstateApi.md#platform_sentient_outposts_get) | **GET** /{platform}/sentientOutposts | Get the current Sentient Outpost, if any
[**platform_simaris_get**](WorldstateApi.md#platform_simaris_get) | **GET** /{platform}/simaris | Get the current Sanctuary Status
[**platform_sortie_get**](WorldstateApi.md#platform_sortie_get) | **GET** /{platform}/sortie | Current Sortie Data
[**platform_steel_path_get**](WorldstateApi.md#platform_steel_path_get) | **GET** /{platform}/steelPath | Current Steel Path Data
[**platform_syndicate_missions_get**](WorldstateApi.md#platform_syndicate_missions_get) | **GET** /{platform}/syndicateMissions | Listing of Syndicate mission nodes
[**platform_timestamp_get**](WorldstateApi.md#platform_timestamp_get) | **GET** /{platform}/timestamp | Get the timestamp that the current worldstate was generated at.
[**platform_vallis_cycle_get**](WorldstateApi.md#platform_vallis_cycle_get) | **GET** /{platform}/vallisCycle | Get the current state of the Orb Vallis
[**platform_void_trader_get**](WorldstateApi.md#platform_void_trader_get) | **GET** /{platform}/voidTrader | Get the current Void Trader Information



## platform_alerts_get

> crate::models::Alert platform_alerts_get(platform, accept_language, language)
Alerts data

Description and rewards for Alerts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**crate::models::Alert**](alert.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_arbitration_get

> crate::models::Arbitration platform_arbitration_get(platform, accept_language, language)
[Unstable] Arbitration data

Description of the currently active arbitration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**crate::models::Arbitration**](arbitration.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_cambion_cycle_get

> crate::models::CambionCycle platform_cambion_cycle_get(platform, accept_language, language)
Get Current Cambion Drift Status

Data on the Vome/Fass cycle for the Cambion Drift on Deimos

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**crate::models::CambionCycle**](cambionCycle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_cetus_cycle_get

> crate::models::CetusCycle platform_cetus_cycle_get(platform, accept_language, language)
Get Current Cetus Status

Data on the day/night cycle for Cetus on Earth

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**crate::models::CetusCycle**](cetusCycle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_conclave_challenges_get

> Vec<serde_json::Value> platform_conclave_challenges_get(platform, accept_language, language)
Get Conclave Challenge Data

Data on each day and week's conclave challenges

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_construction_progress_get

> crate::models::Construction platform_construction_progress_get(platform, accept_language, language)
Get Construction Progress for Fomorians and Razorbacks

Construction percentages for showing how far constructed the enemy fleets are.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**crate::models::Construction**](construction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_daily_deals_get

> Vec<serde_json::Value> platform_daily_deals_get(platform, accept_language, language)
Daily Deal information from Darvo

Darvo's Daily Deal details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_dark_sectors_get

> Vec<serde_json::Value> platform_dark_sectors_get(platform, accept_language, language)
Dark Sector occupation and history

Dark Sector (Rail Wars) data and history. Digital Extremes has emptied several of these.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_earth_cycle_get

> crate::models::EarthCycle platform_earth_cycle_get(platform, accept_language, language)
Get the current Earth rotation information

The current Earth day/night cycle progress.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**crate::models::EarthCycle**](earthCycle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_events_get

> Vec<crate::models::Event> platform_events_get(platform, accept_language, language)
Listing of ongoing events

Events, such as Fomorian Attacks are included here

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**Vec<crate::models::Event>**](event.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_fissures_get

> Vec<crate::models::Fissure> platform_fissures_get(platform, accept_language, language)
Data on current fissures

Information about current Void Fissure missions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**Vec<crate::models::Fissure>**](fissure.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_flash_sales_get

> Vec<serde_json::Value> platform_flash_sales_get(platform, accept_language, language)
Current Flash Sales from Darvo

Popular Deals, discounts, featured deals.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_get

> crate::models::Ws platform_get(platform, accept_language, language)
Get Warframe Worldstate Data for the provided platform

The full translated Warframe Worldstate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**crate::models::Ws**](ws.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_global_upgrades_get

> Vec<serde_json::Value> platform_global_upgrades_get(platform, accept_language, language)
Current Global Upgrades

Any current modifiers applied to all users, such as double drops.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_invasions_get

> Vec<crate::models::Invasion> platform_invasions_get(platform, accept_language, language)
Invasion Data

Data on invasion missions, such as estimated completion time, rewards, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**Vec<crate::models::Invasion>**](invasion.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_kuva_get

> Vec<crate::models::Kuva> platform_kuva_get(platform, accept_language, language)
[Unstable] Current Kuva Mission listing

Current Kuva Mission listing (provided by [semlar](https://10o.io/kuvalog)).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**Vec<crate::models::Kuva>**](kuva.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_news_get

> Vec<serde_json::Value> platform_news_get(platform)
Current Listing of News items

Translated News items from the worldstate

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_nightwave_get

> crate::models::Nightwave platform_nightwave_get(platform, accept_language, language)
Get the current Nightwave state.

The Current cycle and challenges of Nightwave, a battle-pass-esque rotation and challenge system

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**crate::models::Nightwave**](nightwave.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_persistent_enemies_get

> Vec<serde_json::Value> platform_persistent_enemies_get(platform, accept_language, language)
Get Persistent Enemy Data

Data about current acolytes attacking the Sol System

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**Vec<serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_rivens_get

> ::std::collections::HashMap<String, crate::models::Riven> platform_rivens_get(platform, accept_language, language)
Get Riven statistic data

Data about averages, deviations, medians, miniums, and maxes for all rivens for the provided platform

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**::std::collections::HashMap<String, crate::models::Riven>**](riven.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_rivens_search_query_get

> ::std::collections::HashMap<String, crate::models::Riven> platform_rivens_search_query_get(platform, query)
Get Riven statistic data

Data about averages, deviations, medians, miniums, and maxes for rivens whose name match the query for the provided platform

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**query** | **String** | Riven name to search for | [required] |

### Return type

[**::std::collections::HashMap<String, crate::models::Riven>**](riven.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_sentient_outposts_get

> crate::models::InlineResponse200 platform_sentient_outposts_get(platform, accept_language, language)
Get the current Sentient Outpost, if any

Status data for current Sentient Outpost, if any. Parsed source is combined data from DE\\'s worldstate and [semlar\\'s data](https://semlar.com/anomaly.json)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_simaris_get

> crate::models::Simaris platform_simaris_get(platform, accept_language, language)
Get the current Sanctuary Status

Status data for Simaris' Sanctuary

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**crate::models::Simaris**](simaris.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_sortie_get

> Vec<crate::models::Sortie> platform_sortie_get(platform, accept_language, language)
Current Sortie Data

Data about the missions for the current sortie

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**Vec<crate::models::Sortie>**](sortie.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_steel_path_get

> Vec<crate::models::SteelPath> platform_steel_path_get(platform, accept_language, language)
Current Steel Path Data

Data about the missions for the current sortie

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**Vec<crate::models::SteelPath>**](steelPath.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_syndicate_missions_get

> Vec<crate::models::SyndicateMission> platform_syndicate_missions_get(platform, accept_language, language)
Listing of Syndicate mission nodes

Cycling through different nodes each day, these are a general listing of the nodes that each syndicate will use for the day.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**Vec<crate::models::SyndicateMission>**](syndicateMission.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_timestamp_get

> String platform_timestamp_get(platform, accept_language, language)
Get the timestamp that the current worldstate was generated at.

The time that the worldstate was last generated

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_vallis_cycle_get

> crate::models::VallisCycle platform_vallis_cycle_get(platform, accept_language, language)
Get the current state of the Orb Vallis

The current cycle of the Orb Vallis warm/cold cycle

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**crate::models::VallisCycle**](vallisCycle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platform_void_trader_get

> crate::models::VoidTrader platform_void_trader_get(platform, accept_language, language)
Get the current Void Trader Information

Information on the current Void Trader offerings, or when he will arrive.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**platform** | [**crate::models::Platform**](.md) | Platform to provide data for | [required] |
**accept_language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |
**language** | Option<[**crate::models::Language**](.md)> | Language to retrieve |  |

### Return type

[**crate::models::VoidTrader**](voidTrader.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

