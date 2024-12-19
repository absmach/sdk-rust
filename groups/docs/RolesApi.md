# \RolesApi

All URIs are relative to *http://localhost:9004*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_group_role_action**](RolesApi.md#add_group_role_action) | **POST** /{domainID}/groups/{groupID}/roles/{roleName}/actions | Adds a role action for a group role.
[**add_group_role_member**](RolesApi.md#add_group_role_member) | **POST** /{domainID}/groups/{groupID}/roles/{roleName}/members | Adds a member to a group role.
[**create_group_role**](RolesApi.md#create_group_role) | **POST** /{domainID}/groups/{groupID}/roles | Creates a role for a group
[**delete_all_group_role_actions**](RolesApi.md#delete_all_group_role_actions) | **POST** /{domainID}/groups/{groupID}/roles/{roleName}/actions/delete-all | Deletes all role actions for a group role.
[**delete_all_group_role_members**](RolesApi.md#delete_all_group_role_members) | **POST** /{domainID}/groups/{groupID}/roles/{roleName}/members/delete-all | Deletes all members from a group role.
[**delete_group_role**](RolesApi.md#delete_group_role) | **DELETE** /{domainID}/groups/{groupID}/roles/{roleName} | Deletes group role.
[**delete_group_role_action**](RolesApi.md#delete_group_role_action) | **POST** /{domainID}/groups/{groupID}/roles/{roleName}/actions/delete | Deletes role actions for a group role.
[**delete_group_role_members**](RolesApi.md#delete_group_role_members) | **POST** /{domainID}/groups/{groupID}/roles/{roleName}/members/delete | Deletes members from a group role.
[**get_group_role**](RolesApi.md#get_group_role) | **GET** /{domainID}/groups/{groupID}/roles/{roleName} | Retrieves group role.
[**list_available_actions**](RolesApi.md#list_available_actions) | **GET** /{domainID}/groups/roles/available-actions | Retrieves available actions.
[**list_group_role_actions**](RolesApi.md#list_group_role_actions) | **GET** /{domainID}/groups/{groupID}/roles/{roleName}/actions | Lists group role actions.
[**list_group_role_members**](RolesApi.md#list_group_role_members) | **GET** /{domainID}/groups/{groupID}/roles/{roleName}/members | Lists group role members.
[**list_group_roles**](RolesApi.md#list_group_roles) | **GET** /{domainID}/groups/{groupID}/roles | Retrieves groups roles.
[**update_group_role**](RolesApi.md#update_group_role) | **PUT** /{domainID}/groups/{groupID}/roles/{roleName} | Updates group role.



## add_group_role_action

> models::RoleActionsObj add_group_role_action(domain_id, group_id, role_name, role_actions_obj)
Adds a role action for a group role.

Adds a role action for a specific group role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |
**role_actions_obj** | [**RoleActionsObj**](RoleActionsObj.md) | JSON- formatted object decsribing an action to be added to a role. | [required] |

### Return type

[**models::RoleActionsObj**](RoleActionsObj.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_group_role_member

> models::RoleMembersObj add_group_role_member(domain_id, group_id, role_name, role_members_obj)
Adds a member to a group role.

Adds a member to a specific group role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |
**role_members_obj** | [**RoleMembersObj**](RoleMembersObj.md) | JSON- formatted object decsribing a member to be added to a role. | [required] |

### Return type

[**models::RoleMembersObj**](RoleMembersObj.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_group_role

> models::NewRole create_group_role(domain_id, group_id, create_role_obj)
Creates a role for a group

Creates a role for a specific group that is identified by the group ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**create_role_obj** | [**CreateRoleObj**](CreateRoleObj.md) | JSON- formatted object decsribing a new role to be created. | [required] |

### Return type

[**models::NewRole**](NewRole.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_group_role_actions

> delete_all_group_role_actions(domain_id, group_id, role_name)
Deletes all role actions for a group role.

Deletes all role actions for a specific group role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_group_role_members

> delete_all_group_role_members(domain_id, group_id, role_name)
Deletes all members from a group role.

Deletes all members from a specific group role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_role

> delete_group_role(domain_id, group_id, role_name)
Deletes group role.

Deletes a specific group role that is identifier by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_role_action

> delete_group_role_action(domain_id, group_id, role_name, role_actions_obj)
Deletes role actions for a group role.

Deletes a role action for a specific group role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |
**role_actions_obj** | [**RoleActionsObj**](RoleActionsObj.md) | JSON- formatted object decsribing an action to be added to a role. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_role_members

> delete_group_role_members(domain_id, group_id, role_name, role_members_obj)
Deletes members from a group role.

Deletes a member from a specific group role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |
**role_members_obj** | [**RoleMembersObj**](RoleMembersObj.md) | JSON- formatted object decsribing a member to be added to a role. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_role

> models::Role get_group_role(domain_id, group_id, role_name)
Retrieves group role.

Retrieves a specific group role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_available_actions

> models::AvailableActionsObj list_available_actions(domain_id)
Retrieves available actions.

Retrieves a list of available actions.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |

### Return type

[**models::AvailableActionsObj**](AvailableActionsObj.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_group_role_actions

> models::RoleActionsObj list_group_role_actions(domain_id, group_id, role_name)
Lists group role actions.

Retrieves a list of group role actions.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

[**models::RoleActionsObj**](RoleActionsObj.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_group_role_members

> models::RoleMembersObj list_group_role_members(domain_id, group_id, role_name)
Lists group role members.

Retrieves a list of group role members.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

[**models::RoleMembersObj**](RoleMembersObj.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_group_roles

> models::RolesPage list_group_roles(domain_id, group_id, limit, offset)
Retrieves groups roles.

Retrieves a list of group roles. Due to performance concerns, data is retrieved in subsets. The API groups must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]

### Return type

[**models::RolesPage**](RolesPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_role

> models::Role update_group_role(domain_id, group_id, role_name, update_role_obj)
Updates group role.

Updates a specific group role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**group_id** | **uuid::Uuid** | Unique group identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |
**update_role_obj** | [**UpdateRoleObj**](UpdateRoleObj.md) | JSON- formatted object decsribing a role to be updated. | [required] |

### Return type

[**models::Role**](Role.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

