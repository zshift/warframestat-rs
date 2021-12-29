# \RivensApi

All URIs are relative to *https://api.warframestat.us*

Method | HTTP request | Description
------------- | ------------- | -------------
[**platform_rivens_get**](RivensApi.md#platform_rivens_get) | **GET** /{platform}/rivens | Get Riven statistic data
[**platform_rivens_search_query_get**](RivensApi.md#platform_rivens_search_query_get) | **GET** /{platform}/rivens/search/{query} | Get Riven statistic data



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

