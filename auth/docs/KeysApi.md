# \KeysApi

All URIs are relative to *http://localhost:9001*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_key**](KeysApi.md#get_key) | **GET** /keys/{keyID} | Gets API key details.
[**issue_key**](KeysApi.md#issue_key) | **POST** /keys | Issue API key
[**revoke_key**](KeysApi.md#revoke_key) | **DELETE** /keys/{keyID} | Revoke API key



## get_key

> models::Key get_key(key_id)
Gets API key details.

Gets API key details for the given key. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **uuid::Uuid** | API Key ID. | [required] |

### Return type

[**models::Key**](Key.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_key

> issue_key(issue_key_request)
Issue API key

Generates a new API key. Thew new API key will be uniquely identified by its ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_key_request** | [**IssueKeyRequest**](IssueKeyRequest.md) | JSON-formatted document describing key request. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_key

> revoke_key(key_id)
Revoke API key

Revoke API key identified by the given ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**key_id** | **uuid::Uuid** | API Key ID. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

