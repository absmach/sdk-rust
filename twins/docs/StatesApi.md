# \StatesApi

All URIs are relative to *http://localhost:9018*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_states**](StatesApi.md#get_states) | **GET** /states/{twinID} | Retrieves states of twin with id twinID



## get_states

> models::StatesPage get_states(twin_id, limit, offset)
Retrieves states of twin with id twinID

Retrieves a list of states. Due to performance concerns, data is retrieved in subsets. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**twin_id** | **uuid::Uuid** | Unique twin identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]

### Return type

[**models::StatesPage**](StatesPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

