# \GroupsApi

All URIs are relative to *http://localhost:9004*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_children_groups**](GroupsApi.md#add_children_groups) | **POST** /{domainID}/groups/{groupID}/children | Add children groups.
[**create_group**](GroupsApi.md#create_group) | **POST** /{domainID}/groups | Creates new group
[**disable_group**](GroupsApi.md#disable_group) | **POST** /{domainID}/groups/{groupID}/disable | Disables a group
[**domain_id_groups_group_id_delete**](GroupsApi.md#domain_id_groups_group_id_delete) | **DELETE** /{domainID}/groups/{groupID} | Delete group for a group with the given id.
[**enable_group**](GroupsApi.md#enable_group) | **POST** /{domainID}/groups/{groupID}/enable | Enables a group
[**get_group**](GroupsApi.md#get_group) | **GET** /{domainID}/groups/{groupID} | Gets group info.
[**list_children_groups**](GroupsApi.md#list_children_groups) | **GET** /{domainID}/groups/{groupID}/children | List children of a certain group
[**list_group_hierarchy**](GroupsApi.md#list_group_hierarchy) | **GET** /{domainID}/groups/{groupID}/hierarchy | Lists groups hierarchy.
[**list_groups**](GroupsApi.md#list_groups) | **GET** /{domainID}/groups | Lists groups.
[**remove_all_children_groups**](GroupsApi.md#remove_all_children_groups) | **DELETE** /{domainID}/groups/{groupID}/children/all | Remove all children groups.
[**remove_children_groups**](GroupsApi.md#remove_children_groups) | **DELETE** /{domainID}/groups/{groupID}/children | Remove children groups.
[**remove_group_parent_group**](GroupsApi.md#remove_group_parent_group) | **DELETE** /{domainID}/groups/{groupID}/parent | Removes a parent group from a group.
[**set_group_parent_group**](GroupsApi.md#set_group_parent_group) | **POST** /{domainID}/groups/{groupID}/parent | Sets a parent group for a group.
[**update_group**](GroupsApi.md#update_group) | **PUT** /{domainID}/groups/{groupID} | Updates group data.



## add_children_groups

> add_children_groups(domain_id, group_id, children_group_req_obj)
Add children groups.

Adds children groups for a specific group that is identified by the group ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**children_group_req_obj** | [**ChildrenGroupReqObj**](ChildrenGroupReqObj.md) | JSON-formated document describing the children groups to be added to a group. | [required] |

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


## list_children_groups

> models::GroupsPage list_children_groups(domain_id, group_id, limit, offset, start_level, end_level, tree, metadata, name)
List children of a certain group

Lists groups up to a max level of hierarchy that can be fetched in one request ( max level = 5). Result can be filtered by metadata. Groups will be returned as JSON array or JSON tree. Due to performance concerns, result is returned in subsets. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**start_level** | Option<**i32**> | Level of hierarchy from which to start retrieving groups from given group id. |  |
**end_level** | Option<**i32**> | Level of hierarchy up to which to retrieve groups from given group id. |  |
**tree** | Option<**bool**> | Specify type of response, JSON array or tree. |  |[default to false]
**metadata** | Option<**String**> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json. |  |
**name** | Option<**String**> | Group's name. |  |

### Return type

[**models::GroupsPage**](GroupsPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_group_hierarchy

> models::GroupsHierarchyPage list_group_hierarchy(domain_id, group_id, level, tree, direction)
Lists groups hierarchy.

Lists groups heirarchy up to a max level of hierarchy that can be fetched in one request ( max level = 5). Result can be filtered by metadata. Groups will be returned as JSON array or JSON tree. Due to performance concerns, result is returned in subsets. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**level** | Option<**i32**> | Level of hierarchy up to which to retrieve groups from given group id. |  |
**tree** | Option<**bool**> | Specify type of response, JSON array or tree. |  |[default to false]
**direction** | Option<**i32**> | Direction of hierarchy traversal. |  |

### Return type

[**models::GroupsHierarchyPage**](GroupsHierarchyPage.md)

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


## remove_all_children_groups

> remove_all_children_groups(domain_id, group_id)
Remove all children groups.

Removes all children groups for a specific group that is identified by the group ID. 

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


## remove_children_groups

> remove_children_groups(domain_id, group_id, children_group_req_obj)
Remove children groups.

Removes children groups for a specific group that is identified by the group ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**children_group_req_obj** | [**ChildrenGroupReqObj**](ChildrenGroupReqObj.md) | JSON-formated document describing the children groups to be added to a group. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_group_parent_group

> remove_group_parent_group(domain_id, group_id, parent_group_req_obj)
Removes a parent group from a group.

Removes a parent group from a specific group that is identified by the group ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**parent_group_req_obj** | [**ParentGroupReqObj**](ParentGroupReqObj.md) | JSON-formated document describing the parent group to be set to or removed from a group. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_group_parent_group

> set_group_parent_group(domain_id, group_id, parent_group_req_obj)
Sets a parent group for a group.

Sets a parent group for a specific group that is identified by the group ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**parent_group_req_obj** | [**ParentGroupReqObj**](ParentGroupReqObj.md) | JSON-formated document describing the parent group to be set to or removed from a group. | [required] |

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

