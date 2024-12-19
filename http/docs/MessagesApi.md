# \MessagesApi

All URIs are relative to *http://localhost:8008*

Method | HTTP request | Description
------------- | ------------- | -------------
[**channels_id_messages_post**](MessagesApi.md#channels_id_messages_post) | **POST** /channels/{id}/messages | Sends message to the communication channel



## channels_id_messages_post

> channels_id_messages_post(id, sen_ml_record)
Sends message to the communication channel

Sends message to the communication channel. Messages can be sent as JSON formatted SenML or as blob. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | Unique channel identifier. | [required] |
**sen_ml_record** | [**Vec<models::SenMlRecord>**](SenMLRecord.md) | Message to be distributed. Since the platform expects messages to be properly formatted SenML in order to be post-processed, clients are obliged to specify Content-Type header for each published message. Note that all messages that aren't SenML will be accepted and published, but no post-processing will be applied.  | [required] |

### Return type

 (empty response body)

### Authorization

[basicAuth](../README.md#basicAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

