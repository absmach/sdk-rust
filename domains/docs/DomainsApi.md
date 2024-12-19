# \DomainsApi

All URIs are relative to *http://localhost:9003*

Method | HTTP request | Description
------------- | ------------- | -------------
[**domains_domain_id_disable_post**](DomainsApi.md#domains_domain_id_disable_post) | **POST** /domains/{domainID}/disable | Disable a domain
[**domains_domain_id_enable_post**](DomainsApi.md#domains_domain_id_enable_post) | **POST** /domains/{domainID}/enable | Enables a domain
[**domains_domain_id_freeze_post**](DomainsApi.md#domains_domain_id_freeze_post) | **POST** /domains/{domainID}/freeze | Freeze a domain
[**domains_domain_id_get**](DomainsApi.md#domains_domain_id_get) | **GET** /domains/{domainID} | Retrieves domain information
[**domains_domain_id_patch**](DomainsApi.md#domains_domain_id_patch) | **PATCH** /domains/{domainID} | Updates name, metadata, tags and alias of the domain.
[**domains_get**](DomainsApi.md#domains_get) | **GET** /domains | Retrieves list of domains.
[**domains_post**](DomainsApi.md#domains_post) | **POST** /domains | Adds new domain



## domains_domain_id_disable_post

> domains_domain_id_disable_post(domain_id)
Disable a domain

Disable a specific domain that is identified by the domain ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_domain_id_enable_post

> domains_domain_id_enable_post(domain_id)
Enables a domain

Enables a specific domain that is identified by the domain ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_domain_id_freeze_post

> domains_domain_id_freeze_post(domain_id)
Freeze a domain

Freeze a specific domain that is identified by the domain ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_domain_id_get

> models::Domain domains_domain_id_get(domain_id)
Retrieves domain information

Retrieves a specific domain that is identified by the domain ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_domain_id_patch

> models::Domain domains_domain_id_patch(domain_id, domain_update)
Updates name, metadata, tags and alias of the domain.

Updates name, metadata, tags and alias of the domain. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |
**domain_update** | [**DomainUpdate**](DomainUpdate.md) | JSON-formated document describing the name, alias, tags, and metadata of the domain to be updated | [required] |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_get

> models::DomainsPage domains_get(limit, offset, metadata, status, name, permission)
Retrieves list of domains.

Retrieves list of domains that the user have access. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**metadata** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json. |  |
**status** | Option<**String**> | Domain status. |  |[default to enabled]
**name** | Option<**String**> | Domain's name. |  |
**permission** | Option<**String**> | permission. |  |

### Return type

[**models::DomainsPage**](DomainsPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domains_post

> models::Domain domains_post(domain_req_obj)
Adds new domain

Adds new domain. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_req_obj** | [**DomainReqObj**](DomainReqObj.md) | JSON-formatted document describing the new domain to be registered | [required] |

### Return type

[**models::Domain**](Domain.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

