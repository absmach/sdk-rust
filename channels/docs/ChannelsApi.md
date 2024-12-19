# \ChannelsApi

All URIs are relative to *http://localhost:9005*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_channel**](ChannelsApi.md#create_channel) | **POST** /{domainID}/channels | Creates new channel
[**create_channels**](ChannelsApi.md#create_channels) | **POST** /{domainID}/channels/bulk | Creates new channels
[**disable_channel**](ChannelsApi.md#disable_channel) | **POST** /{domainID}/channels/{chanID}/disable | Disables a channel
[**domain_id_channels_chan_id_delete**](ChannelsApi.md#domain_id_channels_chan_id_delete) | **DELETE** /{domainID}/channels/{chanID} | Delete channel for given channel id.
[**enable_channel**](ChannelsApi.md#enable_channel) | **POST** /{domainID}/channels/{chanID}/enable | Enables a channel
[**get_channel**](ChannelsApi.md#get_channel) | **GET** /{domainID}/channels/{chanID} | Retrieves channel info.
[**list_channels**](ChannelsApi.md#list_channels) | **GET** /{domainID}/channels | Lists channels.
[**remove_channel_parent_group**](ChannelsApi.md#remove_channel_parent_group) | **DELETE** /{domainID}/channels/{chanID}/parent | Removes a parent group from a channel.
[**set_channel_parent_group**](ChannelsApi.md#set_channel_parent_group) | **POST** /{domainID}/channels/{chanID}/parent | Sets a parent group for a channel
[**update_channel**](ChannelsApi.md#update_channel) | **PATCH** /{domainID}/channels/{chanID} | Updates channel data.
[**update_channel_tags**](ChannelsApi.md#update_channel_tags) | **PATCH** /{domainID}/channels/{chanID}/tags | Updates channel tags.



## create_channel

> models::Channel create_channel(domain_id, channel_req_obj)
Creates new channel

Creates new channel in domain. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**channel_req_obj** | [**ChannelReqObj**](ChannelReqObj.md) | JSON-formatted document describing the new channel to be registered | [required] |

### Return type

[**models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_channels

> Vec<models::Channel> create_channels(domain_id, channel_req_obj)
Creates new channels

Creates new channels in domain. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**channel_req_obj** | [**Vec<models::ChannelReqObj>**](ChannelReqObj.md) | JSON-formatted document describing the new channels to be registered | [required] |

### Return type

[**Vec<models::Channel>**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_channel

> models::Channel disable_channel(domain_id, chan_id)
Disables a channel

Disables a specific channel that is identified by the channel ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**chan_id** | **uuid::Uuid** | Unique channel identifier. | [required] |

### Return type

[**models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_id_channels_chan_id_delete

> domain_id_channels_chan_id_delete(domain_id, chan_id)
Delete channel for given channel id.

Delete channel remove given channel id from repo and removes all the policies related to channel. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**chan_id** | **uuid::Uuid** | Unique channel identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_channel

> models::Channel enable_channel(domain_id, chan_id)
Enables a channel

Enables a specific channel that is identified by the channel ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**chan_id** | **uuid::Uuid** | Unique channel identifier. | [required] |

### Return type

[**models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_channel

> models::Channel get_channel(domain_id, chan_id)
Retrieves channel info.

Gets info on a channel specified by id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**chan_id** | **uuid::Uuid** | Unique channel identifier. | [required] |

### Return type

[**models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_channels

> models::ChannelsPage list_channels(domain_id, limit, offset, metadata, name)
Lists channels.

Retrieves a list of channels. Due to performance concerns, data is retrieved in subsets. The API clients must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**metadata** | Option<**String**> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json. |  |
**name** | Option<**String**> | Channel's name. |  |

### Return type

[**models::ChannelsPage**](ChannelsPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_channel_parent_group

> remove_channel_parent_group(domain_id, chan_id, parent_group_req_obj)
Removes a parent group from a channel.

Removes a parent group from a specific channel that is identified by the channel ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**chan_id** | **uuid::Uuid** | Unique channel identifier. | [required] |
**parent_group_req_obj** | [**ParentGroupReqObj**](ParentGroupReqObj.md) | JSON-formated document describing the parent group to be set to or removed from a channel. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_channel_parent_group

> set_channel_parent_group(domain_id, chan_id, parent_group_req_obj)
Sets a parent group for a channel

Sets a parent group for a specific channel that is identified by the channel ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**chan_id** | **uuid::Uuid** | Unique channel identifier. | [required] |
**parent_group_req_obj** | [**ParentGroupReqObj**](ParentGroupReqObj.md) | JSON-formated document describing the parent group to be set to or removed from a channel. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_channel

> models::Channel update_channel(domain_id, chan_id, channel_update)
Updates channel data.

Update is performed by replacing the current resource data with values provided in a request payload. Note that the channel's ID will not be affected. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**chan_id** | **uuid::Uuid** | Unique channel identifier. | [required] |
**channel_update** | [**ChannelUpdate**](ChannelUpdate.md) | JSON-formated document describing the metadata and name of channel to be updated. | [required] |

### Return type

[**models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_channel_tags

> models::Channel update_channel_tags(domain_id, chan_id, channel_update)
Updates channel tags.

Update is performed by replacing the current resource data with values provided in a request payload. Note that the channel's ID will not be affected. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**chan_id** | **uuid::Uuid** | Unique channel identifier. | [required] |
**channel_update** | [**ChannelUpdate**](ChannelUpdate.md) | JSON-formated document describing the tags of channel to be updated. | [required] |

### Return type

[**models::Channel**](Channel.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

