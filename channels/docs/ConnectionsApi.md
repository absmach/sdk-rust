# \ConnectionsApi

All URIs are relative to *http://localhost:9005*

Method | HTTP request | Description
------------- | ------------- | -------------
[**connect_clients_and_channels**](ConnectionsApi.md#connect_clients_and_channels) | **POST** /{domainID}/channels/connect | Connects client and channel.
[**connect_clients_to_channel**](ConnectionsApi.md#connect_clients_to_channel) | **POST** /{domainID}/channels/{chanID}/connect | Connects clients to a channel
[**disconnect_clients_and_channels**](ConnectionsApi.md#disconnect_clients_and_channels) | **POST** /{domainID}/channels/disconnect | Disconnects client and channel.
[**disconnect_clients_from_channel**](ConnectionsApi.md#disconnect_clients_from_channel) | **POST** /{domainID}/channels/{chanID}/disconnect | Disconnects clients from a channel



## connect_clients_and_channels

> connect_clients_and_channels(domain_id, connection_req_schema)
Connects client and channel.

Connect clients specified by IDs to channels specified by IDs. Channel and client are owned by user identified using the provided access token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**connection_req_schema** | [**ConnectionReqSchema**](ConnectionReqSchema.md) | JSON-formatted document describing the new connection. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## connect_clients_to_channel

> connect_clients_to_channel(domain_id, chan_id, channel_connection_req_schema)
Connects clients to a channel

Connects clients to a channel that is identified by the channel ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**chan_id** | **uuid::Uuid** | Unique channel identifier. | [required] |
**channel_connection_req_schema** | [**ChannelConnectionReqSchema**](ChannelConnectionReqSchema.md) | JSON-formatted document describing the new connection. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_clients_and_channels

> disconnect_clients_and_channels(domain_id, connection_req_schema)
Disconnects client and channel.

Disconnect clients specified by IDs from channels specified by IDs. Channel and client are owned by user identified using the provided access token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**connection_req_schema** | [**ConnectionReqSchema**](ConnectionReqSchema.md) | JSON-formatted document describing the new connection. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disconnect_clients_from_channel

> disconnect_clients_from_channel(domain_id, chan_id, channel_connection_req_schema)
Disconnects clients from a channel

Disconnects clients to a channel that is identified by the channel ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**chan_id** | **uuid::Uuid** | Unique channel identifier. | [required] |
**channel_connection_req_schema** | [**ChannelConnectionReqSchema**](ChannelConnectionReqSchema.md) | JSON-formatted document describing the new connection. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

