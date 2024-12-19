# \ProvisionApi

All URIs are relative to *http://localhost:9016*

Method | HTTP request | Description
------------- | ------------- | -------------
[**domain_id_mapping_get**](ProvisionApi.md#domain_id_mapping_get) | **GET** /{domainID}/mapping | Gets current mapping.
[**domain_id_mapping_post**](ProvisionApi.md#domain_id_mapping_post) | **POST** /{domainID}/mapping | Adds new device to proxy



## domain_id_mapping_get

> serde_json::Value domain_id_mapping_get(domain_id)
Gets current mapping.

Gets current mapping. This can be used in UI so that when bootstrap config is created from UI matches configuration created with provision service.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_id_mapping_post

> domain_id_mapping_post(domain_id, domain_id_mapping_post_request)
Adds new device to proxy

Adds new device to proxy

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**domain_id_mapping_post_request** | Option<[**DomainIdMappingPostRequest**](DomainIdMappingPostRequest.md)> | MAC address of device or other identifier |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

