# \GroupsApi

All URIs are relative to *http://localhost:9002*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assign_user**](GroupsApi.md#assign_user) | **POST** /{domainID}/groups/{groupID}/users/assign | Assigns a user to a group
[**create_group**](GroupsApi.md#create_group) | **POST** /{domainID}/groups | Creates new group
[**disable_group**](GroupsApi.md#disable_group) | **POST** /{domainID}/groups/{groupID}/disable | Disables a group
[**domain_id_groups_group_id_delete**](GroupsApi.md#domain_id_groups_group_id_delete) | **DELETE** /{domainID}/groups/{groupID} | Delete group for a group with the given id.
[**enable_group**](GroupsApi.md#enable_group) | **POST** /{domainID}/groups/{groupID}/enable | Enables a group
[**get_group**](GroupsApi.md#get_group) | **GET** /{domainID}/groups/{groupID} | Gets group info.
[**list_children**](GroupsApi.md#list_children) | **GET** /{domainID}/groups/{groupID}/children | List children of a certain group
[**list_groups**](GroupsApi.md#list_groups) | **GET** /{domainID}/groups | Lists groups.
[**list_groups_by_user**](GroupsApi.md#list_groups_by_user) | **GET** /{domainID}/users/{memberID}/groups | Get group associated with the member
[**list_groups_in_channel**](GroupsApi.md#list_groups_in_channel) | **GET** /{domainID}/channels/{memberID}/groups | Get group associated with the member
[**list_parents**](GroupsApi.md#list_parents) | **GET** /{domainID}/groups/{groupID}/parents | List parents of a certain group
[**unassign_user**](GroupsApi.md#unassign_user) | **POST** /{domainID}/groups/{groupID}/users/unassign | Unassigns a user to a group
[**update_group**](GroupsApi.md#update_group) | **PUT** /{domainID}/groups/{groupID} | Updates group data.



## assign_user

> assign_user(domain_id, group_id, assign_user_req_obj)
Assigns a user to a group

Assigns a specific user to a group that is identifier by the group ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**assign_user_req_obj** | [**AssignUserReqObj**](AssignUserReqObj.md) | JSON-formated document describing the policy related to assigning users to a group | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group

> models::Group create_group(domain_id, group_req_obj)
Creates new group

Creates new group that can be used for grouping entities. New account will be uniquely identified by its identity. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_req_obj** | [**GroupReqObj**](GroupReqObj.md) | JSON-formatted document describing the new group to be registered | [required] |

### Return type

[**models::Group**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_group

> models::Group disable_group(domain_id, group_id)
Disables a group

Disables a specific group that is identifier by the group ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |

### Return type

[**models::Group**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## domain_id_groups_group_id_delete

> domain_id_groups_group_id_delete(domain_id, group_id)
Delete group for a group with the given id.

Delete group removes a group with the given id from repo and removes all the policies related to this group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enable_group

> models::Group enable_group(domain_id, group_id)
Enables a group

Enables a specific group that is identifier by the group ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |

### Return type

[**models::Group**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group

> models::Group get_group(domain_id, group_id)
Gets group info.

Gets info on a group specified by id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |

### Return type

[**models::Group**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_children

> models::GroupsPage list_children(domain_id, group_id, limit, offset, level, tree, metadata, name, parent_id)
List children of a certain group

Lists groups up to a max level of hierarchy that can be fetched in one request ( max level = 5). Result can be filtered by metadata. Groups will be returned as JSON array or JSON tree. Due to performance concerns, result is returned in subsets. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**level** | Option<**i32**> | Level of hierarchy up to which to retrieve groups from given group id. |  |
**tree** | Option<**bool**> | Specify type of response, JSON array or tree. |  |[default to false]
**metadata** | Option<**String**> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json. |  |
**name** | Option<**String**> | Group's name. |  |
**parent_id** | Option<**uuid::Uuid**> | Unique parent identifier for a group. |  |

### Return type

[**models::GroupsPage**](GroupsPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_groups

> models::GroupsPage list_groups(domain_id, limit, offset, level, tree, metadata, name, parent_id)
Lists groups.

Lists groups up to a max level of hierarchy that can be fetched in one request ( max level = 5). Result can be filtered by metadata. Groups will be returned as JSON array or JSON tree. Due to performance concerns, result is returned in subsets. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**level** | Option<**i32**> | Level of hierarchy up to which to retrieve groups from given group id. |  |
**tree** | Option<**bool**> | Specify type of response, JSON array or tree. |  |[default to false]
**metadata** | Option<**String**> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json. |  |
**name** | Option<**String**> | Group's name. |  |
**parent_id** | Option<**uuid::Uuid**> | Unique parent identifier for a group. |  |

### Return type

[**models::GroupsPage**](GroupsPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_groups_by_user

> models::GroupsPage list_groups_by_user(domain_id, member_id, limit, offset, metadata, status, tags)
Get group associated with the member

Gets groups associated with the user member specified by id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**member_id** | **uuid::Uuid** | Unique member identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**metadata** | Option<**String**> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json. |  |
**status** | Option<**String**> | User account status. |  |[default to enabled]
**tags** | Option<[**Vec<String>**](String.md)> | User tags. |  |

### Return type

[**models::GroupsPage**](GroupsPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_groups_in_channel

> models::GroupsPage list_groups_in_channel(domain_id, member_id, limit, offset, metadata, status, tags)
Get group associated with the member

Gets groups associated with the channel member specified by id. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**member_id** | **uuid::Uuid** | Unique member identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**metadata** | Option<**String**> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json. |  |
**status** | Option<**String**> | User account status. |  |[default to enabled]
**tags** | Option<[**Vec<String>**](String.md)> | User tags. |  |

### Return type

[**models::GroupsPage**](GroupsPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_parents

> models::GroupsPage list_parents(domain_id, group_id, limit, offset, level, tree, metadata, name, parent_id)
List parents of a certain group

Lists groups up to a max level of hierarchy that can be fetched in one request ( max level = 5). Result can be filtered by metadata. Groups will be returned as JSON array or JSON tree. Due to performance concerns, result is returned in subsets. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**level** | Option<**i32**> | Level of hierarchy up to which to retrieve groups from given group id. |  |
**tree** | Option<**bool**> | Specify type of response, JSON array or tree. |  |[default to false]
**metadata** | Option<**String**> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json. |  |
**name** | Option<**String**> | Group's name. |  |
**parent_id** | Option<**uuid::Uuid**> | Unique parent identifier for a group. |  |

### Return type

[**models::GroupsPage**](GroupsPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## unassign_user

> unassign_user(domain_id, group_id, assign_user_req_obj)
Unassigns a user to a group

Unassigns a specific user to a group that is identifier by the group ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**assign_user_req_obj** | [**AssignUserReqObj**](AssignUserReqObj.md) | JSON-formated document describing the policy related to assigning users to a group | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group

> models::Group update_group(domain_id, group_id, group_update)
Updates group data.

Updates Name, Description or Metadata of a group. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**group_update** | [**GroupUpdate**](GroupUpdate.md) | JSON-formated document describing the metadata and name of group to be update | [required] |

### Return type

[**models::Group**](Group.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

