# \ReadersApi

All URIs are relative to *http://localhost:9003*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_messages**](ReadersApi.md#get_messages) | **GET** /channels/{chanId}/messages | Retrieves messages sent to single channel



## get_messages

> models::MessagesPage get_messages(chan_id, limit, offset, publisher, name, v, vb, vs, vd, from, to, aggregation, interval)
Retrieves messages sent to single channel

Retrieves a list of messages sent to specific channel. Due to performance concerns, data is retrieved in subsets. The API readers must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**chan_id** | **uuid::Uuid** | Unique channel identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**publisher** | Option<**uuid::Uuid**> | Unique client identifier. |  |
**name** | Option<**String**> | SenML message name. |  |
**v** | Option<**String**> | SenML message value. |  |
**vb** | Option<**bool**> | SenML message bool value. |  |
**vs** | Option<**String**> | SenML message string value. |  |
**vd** | Option<**String**> | SenML message data value. |  |
**from** | Option<**f64**> | SenML message time in nanoseconds (integer part represents seconds). |  |
**to** | Option<**f64**> | SenML message time in nanoseconds (integer part represents seconds). |  |
**aggregation** | Option<**String**> | Aggregation function. |  |
**interval** | Option<**String**> | Aggregation interval. |  |

### Return type

[**models::MessagesPage**](MessagesPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth), [clientAuth](../README.md#clientAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

