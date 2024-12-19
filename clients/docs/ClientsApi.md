# \ClientsApi

All URIs are relative to *http://localhost:9006*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_create_clients**](ClientsApi.md#bulk_create_clients) | **POST** /{domainID}/clients/bulk | Bulk provisions new clients
[**create_client**](ClientsApi.md#create_client) | **POST** /{domainID}/clients | Adds new client
[**disable_client**](ClientsApi.md#disable_client) | **POST** /{domainID}/clients/{clientID}/disable | Disables a client
[**domain_id_clients_client_id_delete**](ClientsApi.md#domain_id_clients_client_id_delete) | **DELETE** /{domainID}/clients/{clientID} | Delete client for a client with the given id.
[**enable_client**](ClientsApi.md#enable_client) | **POST** /{domainID}/clients/{clientID}/enable | Enables a client
[**get_client**](ClientsApi.md#get_client) | **GET** /{domainID}/clients/{clientID} | Retrieves client info
[**list_clients**](ClientsApi.md#list_clients) | **GET** /{domainID}/clients | Retrieves clients
[**list_user_clients**](ClientsApi.md#list_user_clients) | **GET** /{domainID}/users/{userID}/clients | List clients asssociated with a user.
[**remove_client_parent_group**](ClientsApi.md#remove_client_parent_group) | **DELETE** /{domainID}/clients/{clientID}/parent | Removes a parent group from a client.
[**set_client_parent_group**](ClientsApi.md#set_client_parent_group) | **POST** /{domainID}/clients/{clientID}/parent | Sets a parent group for a client
[**update_client**](ClientsApi.md#update_client) | **PATCH** /{domainID}/clients/{clientID} | Updates name and metadata of the client.
[**update_client_secret**](ClientsApi.md#update_client_secret) | **PATCH** /{domainID}/clients/{clientID}/secret | Updates Secret of the identified client.
[**update_client_tags**](ClientsApi.md#update_client_tags) | **PATCH** /{domainID}/clients/{clientID}/tags | Updates tags the client.



## bulk_create_clients

> models::ClientsPage bulk_create_clients(domain_id, client_req_obj)
Bulk provisions new clients

Adds a list of new clients to the list of clients owned by user identified using the provided access token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_req_obj** | [**Vec<models::ClientReqObj>**](ClientReqObj.md) | JSON-formatted document describing the new clients. | [required] |

### Return type

[**models::ClientsPage**](ClientsPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_client

> models::Client create_client(domain_id, client_req_obj)
Adds new client

Adds new client to the list of clients owned by user identified using the provided access token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_req_obj** | [**ClientReqObj**](ClientReqObj.md) | JSON-formatted document describing the new client to be registered | [required] |

### Return type

[**models::Client**](Client.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_client

> models::Client disable_client(domain_id, client_id)
Disables a client

Disables a specific client that is identified by the client ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |

### Return type

[**models::Client**](Client.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_id_clients_client_id_delete

> domain_id_clients_client_id_delete(domain_id, client_id)
Delete client for a client with the given id.

Delete client removes a client with the given id from repo and removes all the policies related to this client. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_client

> models::Client enable_client(domain_id, client_id)
Enables a client

Enables a specific client that is identified by the client ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |

### Return type

[**models::Client**](Client.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_client

> models::Client get_client(domain_id, client_id)
Retrieves client info

Retrieves a specific client that is identified by the client ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |

### Return type

[**models::Client**](Client.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_clients

> models::ClientsPage list_clients(domain_id, limit, offset, metadata, status, name, tags)
Retrieves clients

Retrieves a list of clients. Due to performance concerns, data is retrieved in subsets. The API clients must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**metadata** | Option<**String**> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json. |  |
**status** | Option<**String**> | Client account status. |  |[default to enabled]
**name** | Option<**String**> | Client's name. |  |
**tags** | Option<[**Vec<String>**](String.md)> | Client tags. |  |

### Return type

[**models::ClientsPage**](ClientsPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_user_clients

> models::ClientsPage list_user_clients(domain_id, user_id, limit, offset, metadata, status, name, tags)
List clients asssociated with a user.

Lists clients associated with a user identified by the user ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**metadata** | Option<**String**> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json. |  |
**status** | Option<**String**> | Client account status. |  |[default to enabled]
**name** | Option<**String**> | Client's name. |  |
**tags** | Option<[**Vec<String>**](String.md)> | Client tags. |  |

### Return type

[**models::ClientsPage**](ClientsPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_client_parent_group

> remove_client_parent_group(domain_id, client_id, parent_group_req_obj)
Removes a parent group from a client.

Removes a parent group from a specific client that is identified by the client ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
**parent_group_req_obj** | [**ParentGroupReqObj**](ParentGroupReqObj.md) | JSON-formated document describing the parent group to be set to or removed from a client. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_client_parent_group

> set_client_parent_group(domain_id, client_id, parent_group_req_obj)
Sets a parent group for a client

Sets a parent group for a specific client that is identified by the client ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
**parent_group_req_obj** | [**ParentGroupReqObj**](ParentGroupReqObj.md) | JSON-formated document describing the parent group to be set to or removed from a client. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_client

> models::Client update_client(domain_id, client_id, client_update)
Updates name and metadata of the client.

Update is performed by replacing the current resource data with values provided in a request payload. Note that the client's type and ID cannot be changed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
**client_update** | [**ClientUpdate**](ClientUpdate.md) | JSON-formated document describing the metadata and name of client to be update | [required] |

### Return type

[**models::Client**](Client.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_client_secret

> models::Client update_client_secret(domain_id, client_id, client_secret)
Updates Secret of the identified client.

Updates secret of the identified in client. Secret is updated using authorization token and the new received info. Update is performed by replacing current key with a new one. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
**client_secret** | [**ClientSecret**](ClientSecret.md) | Secret change data. Client can change its secret. | [required] |

### Return type

[**models::Client**](Client.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_client_tags

> models::Client update_client_tags(domain_id, client_id, client_tags)
Updates tags the client.

Updates tags of the client with provided ID. Tags is updated using authorization token and the new tags received in request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
**client_tags** | [**ClientTags**](ClientTags.md) | JSON-formated document describing the tags of client to be update | [required] |

### Return type

[**models::Client**](Client.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

