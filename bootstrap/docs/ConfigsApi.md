# \ConfigsApi

All URIs are relative to *http://localhost:9013*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_config**](ConfigsApi.md#create_config) | **POST** /{domainID}/clients/configs | Adds new config
[**get_bootstrap_config**](ConfigsApi.md#get_bootstrap_config) | **GET** /clients/bootstrap/{externalId} | Retrieves configuration.
[**get_config**](ConfigsApi.md#get_config) | **GET** /{domainID}/clients/configs/{configId} | Retrieves config info (with channels).
[**get_configs**](ConfigsApi.md#get_configs) | **GET** /{domainID}/clients/configs | Retrieves managed configs
[**get_secure_bootstrap_config**](ConfigsApi.md#get_secure_bootstrap_config) | **GET** /clients/bootstrap/secure/{externalId} | Retrieves configuration.
[**remove_config**](ConfigsApi.md#remove_config) | **DELETE** /{domainID}/clients/configs/{configId} | Removes a Config
[**update_config**](ConfigsApi.md#update_config) | **PUT** /{domainID}/clients/configs/{configId} | Updates config info
[**update_config_certs**](ConfigsApi.md#update_config_certs) | **PATCH** /{domainID}/clients/configs/certs/{configId} | Updates certs
[**update_config_connections**](ConfigsApi.md#update_config_connections) | **PUT** /{domainID}/clients/configs/connections/{configId} | Updates channels the client is connected to
[**update_config_state**](ConfigsApi.md#update_config_state) | **PUT** /{domainID}/clients/state/{configId} | Updates Config state.



## create_config

> create_config(domain_id, create_config_request)
Adds new config

Adds new config to the list of config owned by user identified using the provided access token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**create_config_request** | [**CreateConfigRequest**](CreateConfigRequest.md) | JSON-formatted document describing the new config. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bootstrap_config

> models::BootstrapConfig get_bootstrap_config(external_id)
Retrieves configuration.

Retrieves a configuration with given external ID and external key. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | Unique Config identifier provided by external entity. | [required] |

### Return type

[**models::BootstrapConfig**](BootstrapConfig.md)

### Authorization

[bootstrapAuth](../README.md#bootstrapAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_config

> models::Config get_config(domain_id, config_id)
Retrieves config info (with channels).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**config_id** | **uuid::Uuid** | Unique Config identifier. It's the ID of the corresponding Client. | [required] |

### Return type

[**models::Config**](Config.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_configs

> models::ConfigList get_configs(domain_id, limit, offset, state, name)
Retrieves managed configs

Retrieves a list of managed configs. Due to performance concerns, data is retrieved in subsets. The API configs must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**state** | Option<[**State**](.md)> | A state of items |  |
**name** | Option<**String**> | Name of the config. Search by name is partial-match and case-insensitive. |  |

### Return type

[**models::ConfigList**](ConfigList.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_secure_bootstrap_config

> models::BootstrapConfig get_secure_bootstrap_config(external_id)
Retrieves configuration.

Retrieves a configuration with given external ID and encrypted external key. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_id** | **String** | Unique Config identifier provided by external entity. | [required] |

### Return type

[**models::BootstrapConfig**](BootstrapConfig.md)

### Authorization

[bootstrapEncAuth](../README.md#bootstrapEncAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_config

> remove_config(domain_id, config_id)
Removes a Config

Removes a Config. In case of successful removal the service will ensure that the removed config is disconnected from all of the SuperMQ channels. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**config_id** | **uuid::Uuid** | Unique Config identifier. It's the ID of the corresponding Client. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_config

> update_config(domain_id, config_id, update_config_request)
Updates config info

Update is performed by replacing the current resource data with values provided in a request payload. Note that the owner, ID, external ID, external key, SuperMQ Client ID and key cannot be changed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**config_id** | **uuid::Uuid** | Unique Config identifier. It's the ID of the corresponding Client. | [required] |
**update_config_request** | Option<[**UpdateConfigRequest**](UpdateConfigRequest.md)> | JSON-formatted document describing the updated client. |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_config_certs

> models::ConfigUpdateCerts update_config_certs(domain_id, config_id, update_config_certs_request)
Updates certs

Update is performed by replacing the current certificate data with values provided in a request payload. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**config_id** | **uuid::Uuid** | Unique Config identifier. It's the ID of the corresponding Client. | [required] |
**update_config_certs_request** | Option<[**UpdateConfigCertsRequest**](UpdateConfigCertsRequest.md)> | JSON-formatted document describing the updated client. |  |

### Return type

[**models::ConfigUpdateCerts**](ConfigUpdateCerts.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_config_connections

> update_config_connections(domain_id, config_id, update_config_connections_request)
Updates channels the client is connected to

Update connections performs update of the channel list corresponding Client is connected to. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**config_id** | **uuid::Uuid** | Unique Config identifier. It's the ID of the corresponding Client. | [required] |
**update_config_connections_request** | Option<[**UpdateConfigConnectionsRequest**](UpdateConfigConnectionsRequest.md)> | Array if IDs the client is be connected to. |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_config_state

> update_config_state(domain_id, config_id, update_config_state_request)
Updates Config state.

Updating state represents enabling/disabling Config, i.e. connecting and disconnecting corresponding SuperMQ Client to the list of Channels. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**config_id** | **uuid::Uuid** | Unique Config identifier. It's the ID of the corresponding Client. | [required] |
**update_config_state_request** | Option<[**UpdateConfigStateRequest**](UpdateConfigStateRequest.md)> | Update the state of the Config. |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

