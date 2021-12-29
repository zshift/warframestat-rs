# \UnstableApi

All URIs are relative to *https://api.warframestat.us*

Method | HTTP request | Description
------------- | ------------- | -------------
[**platform_arbitration_get**](UnstableApi.md#platform_arbitration_get) | **GET** /{platform}/arbitration | [Unstable] Arbitration data
[**platform_kuva_get**](UnstableApi.md#platform_kuva_get) | **GET** /{platform}/kuva | [Unstable] Current Kuva Mission listing



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

