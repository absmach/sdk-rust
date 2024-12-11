# \DomainsApi

All URIs are relative to *http://localhost:9002*

Method | HTTP request | Description
------------- | ------------- | -------------
[**domain_id_users_get**](DomainsApi.md#domain_id_users_get) | **GET** /{domainID}/users | List users assigned to domain



## domain_id_users_get

> models::UsersPage domain_id_users_get(domain_id, limit, offset, metadata, status)
List users assigned to domain

List users assigned to domain that is identified by the domain ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**metadata** | Option<**String**> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json. |  |
**status** | Option<**String**> | User account status. |  |[default to enabled]

### Return type

[**models::UsersPage**](UsersPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

