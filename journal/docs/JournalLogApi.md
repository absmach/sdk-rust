# \JournalLogApi

All URIs are relative to *http://localhost:9021*

Method | HTTP request | Description
------------- | ------------- | -------------
[**domain_id_journal_entity_type_id_get**](JournalLogApi.md#domain_id_journal_entity_type_id_get) | **GET** /{domainID}/journal/{entityType}/{id} | List entity journal log
[**journal_user_user_id_get**](JournalLogApi.md#journal_user_user_id_get) | **GET** /journal/user/{userID} | List user journal log



## domain_id_journal_entity_type_id_get

> models::JournalPage domain_id_journal_entity_type_id_get(domain_id, entity_type, id, offset, limit, operation, with_attributes, with_metadata, from, to, dir)
List entity journal log

Retrieves a list of journal. Due to performance concerns, data is retrieved in subsets. The API must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique identifier for a domain. | [required] |
**entity_type** | **String** | Type of entity, e.g. user, group, client, etc.entityType | [required] |
**id** | **uuid::Uuid** | Unique identifier for an entity, e.g. group, channel or thing.  Used together with entity_type. | [required] |
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**operation** | Option<**String**> | Journal operation. |  |
**with_attributes** | Option<**bool**> | Include journal attributes. |  |
**with_metadata** | Option<**bool**> | Include journal metadata. |  |
**from** | Option<**String**> | Start date in unix time. |  |
**to** | Option<**String**> | End date in unix time. |  |
**dir** | Option<**String**> | Sort direction. |  |

### Return type

[**models::JournalPage**](JournalPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## journal_user_user_id_get

> models::JournalPage journal_user_user_id_get(user_id, offset, limit, operation, with_attributes, with_metadata, from, to, dir)
List user journal log

Retrieves a list of journal. Due to performance concerns, data is retrieved in subsets. The API must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique identifier for a user. | [required] |
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**operation** | Option<**String**> | Journal operation. |  |
**with_attributes** | Option<**bool**> | Include journal attributes. |  |
**with_metadata** | Option<**bool**> | Include journal metadata. |  |
**from** | Option<**String**> | Start date in unix time. |  |
**to** | Option<**String**> | End date in unix time. |  |
**dir** | Option<**String**> | Sort direction. |  |

### Return type

[**models::JournalPage**](JournalPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

