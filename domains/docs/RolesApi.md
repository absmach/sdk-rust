# \RolesApi

All URIs are relative to *http://localhost:9003*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_domain_role_action**](RolesApi.md#add_domain_role_action) | **POST** /domains/{domainID}/roles/{roleName}/actions | Adds a role action for a domain role.
[**add_domain_role_member**](RolesApi.md#add_domain_role_member) | **POST** /domains/{domainID}/roles/{roleName}/members | Adds a member to a domain role.
[**create_domain_role**](RolesApi.md#create_domain_role) | **POST** /domains/{domainID}/roles | Creates a role for a domain
[**delete_all_domain_role_actions**](RolesApi.md#delete_all_domain_role_actions) | **POST** /domains/{domainID}/roles/{roleName}/actions/delete-all | Deletes all role actions for a domain role.
[**delete_all_domain_role_members**](RolesApi.md#delete_all_domain_role_members) | **POST** /domains/{domainID}/roles/{roleName}/members/delete-all | Deletes all members from a domain role.
[**delete_domain_role**](RolesApi.md#delete_domain_role) | **DELETE** /domains/{domainID}/roles/{roleName} | Deletes domain role.
[**delete_domain_role_action**](RolesApi.md#delete_domain_role_action) | **POST** /domains/{domainID}/roles/{roleName}/actions/delete | Deletes role actions for a domain role.
[**delete_domain_role_members**](RolesApi.md#delete_domain_role_members) | **POST** /domains/{domainID}/roles/{roleName}/members/delete | Deletes members from a domain role.
[**get_domain_role**](RolesApi.md#get_domain_role) | **GET** /domains/{domainID}/roles/{roleName} | Retrieves domain role.
[**list_available_actions**](RolesApi.md#list_available_actions) | **GET** /domains/roles/available-actions | Retrieves available actions.
[**list_domain_role_actions**](RolesApi.md#list_domain_role_actions) | **GET** /domains/{domainID}/roles/{roleName}/actions | Lists domain role actions.
[**list_domain_role_members**](RolesApi.md#list_domain_role_members) | **GET** /domains/{domainID}/roles/{roleName}/members | Lists domain role members.
[**list_domain_roles**](RolesApi.md#list_domain_roles) | **GET** /domains/{domainID}/roles | Retrieves domains roles.
[**update_domain_role**](RolesApi.md#update_domain_role) | **PUT** /domains/{domainID}/roles/{roleName} | Updates domain role.



## add_domain_role_action

> models::RoleActionsObj add_domain_role_action(domain_id, role_name, role_actions_obj)
Adds a role action for a domain role.

Adds a role action for a specific domain role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |
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


## add_domain_role_member

> models::RoleMembersObj add_domain_role_member(domain_id, role_name, role_members_obj)
Adds a member to a domain role.

Adds a member to a specific domain role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |
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


## create_domain_role

> models::NewRole create_domain_role(domain_id, create_role_obj)
Creates a role for a domain

Creates a role for a specific domain that is identified by the domain ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |
**create_role_obj** | [**CreateRoleObj**](CreateRoleObj.md) | JSON- formatted object decsribing a new role to be created. | [required] |

### Return type

[**models::NewRole**](NewRole.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_domain_role_actions

> delete_all_domain_role_actions(domain_id, role_name)
Deletes all role actions for a domain role.

Deletes all role actions for a specific domain role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_domain_role_members

> delete_all_domain_role_members(domain_id, role_name)
Deletes all members from a domain role.

Deletes all members from a specific domain role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_domain_role

> delete_domain_role(domain_id, role_name)
Deletes domain role.

Deletes a specific domain role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_domain_role_action

> delete_domain_role_action(domain_id, role_name, role_actions_obj)
Deletes role actions for a domain role.

Deletes a role action for a specific domain role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |
**role_name** | **String** | Role's name. | [required] |
**role_actions_obj** | [**RoleActionsObj**](RoleActionsObj.md) | JSON- formatted object decsribing an action to be added to a role. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_domain_role_members

> delete_domain_role_members(domain_id, role_name, role_members_obj)
Deletes members from a domain role.

Deletes a member from a specific domain role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |
**role_name** | **String** | Role's name. | [required] |
**role_members_obj** | [**RoleMembersObj**](RoleMembersObj.md) | JSON- formatted object decsribing a member to be added to a role. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_domain_role

> models::Role get_domain_role(domain_id, role_name)
Retrieves domain role.

Retrieves a specific domain role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |
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
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |

### Return type

[**models::AvailableActionsObj**](AvailableActionsObj.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_domain_role_actions

> models::RoleActionsObj list_domain_role_actions(domain_id, role_name)
Lists domain role actions.

Retrieves a list of domain role actions.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

[**models::RoleActionsObj**](RoleActionsObj.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_domain_role_members

> models::RoleMembersObj list_domain_role_members(domain_id, role_name)
Lists domain role members.

Retrieves a list of domain role members.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

[**models::RoleMembersObj**](RoleMembersObj.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_domain_roles

> models::RolesPage list_domain_roles(domain_id, limit, offset)
Retrieves domains roles.

Retrieves a list of domain roles. Due to performance concerns, data is retrieved in subsets. The API domains must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |
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


## update_domain_role

> models::Role update_domain_role(domain_id, role_name, update_role_obj)
Updates domain role.

Updates a specific domain role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identified. | [required] |
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

