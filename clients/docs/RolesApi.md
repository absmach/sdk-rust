# \RolesApi

All URIs are relative to *http://localhost:9006*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_client_role_action**](RolesApi.md#add_client_role_action) | **POST** /{domainID}/clients/{clientID}/roles/{roleName}/actions | Adds a role action for a client role.
[**add_client_role_member**](RolesApi.md#add_client_role_member) | **POST** /{domainID}/clients/{clientID}/roles/{roleName}/members | Adds a member to a client role.
[**create_client_role**](RolesApi.md#create_client_role) | **POST** /{domainID}/clients/{clientID}/roles | Creates a role for a client
[**delete_all_client_role_actions**](RolesApi.md#delete_all_client_role_actions) | **POST** /{domainID}/clients/{clientID}/roles/{roleName}/actions/delete-all | Deletes all role actions for a client role.
[**delete_all_client_role_members**](RolesApi.md#delete_all_client_role_members) | **POST** /{domainID}/clients/{clientID}/roles/{roleName}/members/delete-all | Deletes all members from a client role.
[**delete_client_role**](RolesApi.md#delete_client_role) | **DELETE** /{domainID}/clients/{clientID}/roles/{roleName} | Deletes client role.
[**delete_client_role_action**](RolesApi.md#delete_client_role_action) | **POST** /{domainID}/clients/{clientID}/roles/{roleName}/actions/delete | Deletes role actions for a client role.
[**delete_client_role_members**](RolesApi.md#delete_client_role_members) | **POST** /{domainID}/clients/{clientID}/roles/{roleName}/members/delete | Deletes members from a client role.
[**get_client_role**](RolesApi.md#get_client_role) | **GET** /{domainID}/clients/{clientID}/roles/{roleName} | Retrieves client role.
[**list_available_actions**](RolesApi.md#list_available_actions) | **GET** /{domainID}/clients/roles/available-actions | Retrieves available actions.
[**list_client_role_actions**](RolesApi.md#list_client_role_actions) | **GET** /{domainID}/clients/{clientID}/roles/{roleName}/actions | Lists client role actions.
[**list_client_role_members**](RolesApi.md#list_client_role_members) | **GET** /{domainID}/clients/{clientID}/roles/{roleName}/members | Lists client role members.
[**list_client_roles**](RolesApi.md#list_client_roles) | **GET** /{domainID}/clients/{clientID}/roles | Retrieves clients roles.
[**update_client_role**](RolesApi.md#update_client_role) | **PUT** /{domainID}/clients/{clientID}/roles/{roleName} | Updates client role.



## add_client_role_action

> models::RoleActionsObj add_client_role_action(domain_id, client_id, role_name, role_actions_obj)
Adds a role action for a client role.

Adds a role action for a specific client role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
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


## add_client_role_member

> models::RoleMembersObj add_client_role_member(domain_id, client_id, role_name, role_members_obj)
Adds a member to a client role.

Adds a member to a specific client role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
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


## create_client_role

> models::NewRole create_client_role(domain_id, client_id, create_role_obj)
Creates a role for a client

Creates a role for a specific client that is identified by the client ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
**create_role_obj** | [**CreateRoleObj**](CreateRoleObj.md) | JSON- formatted object decsribing a new role to be created. | [required] |

### Return type

[**models::NewRole**](NewRole.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_client_role_actions

> delete_all_client_role_actions(domain_id, client_id, role_name)
Deletes all role actions for a client role.

Deletes all role actions for a specific client role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_all_client_role_members

> delete_all_client_role_members(domain_id, client_id, role_name)
Deletes all members from a client role.

Deletes all members from a specific client role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_client_role

> delete_client_role(domain_id, client_id, role_name)
Deletes client role.

Deletes a specific client role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_client_role_action

> delete_client_role_action(domain_id, client_id, role_name, role_actions_obj)
Deletes role actions for a client role.

Deletes a role action for a specific client role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
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


## delete_client_role_members

> delete_client_role_members(domain_id, client_id, role_name, role_members_obj)
Deletes members from a client role.

Deletes a member from a specific client role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
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


## get_client_role

> models::Role get_client_role(domain_id, client_id, role_name)
Retrieves client role.

Retrieves a specific client role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
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


## list_client_role_actions

> models::RoleActionsObj list_client_role_actions(domain_id, client_id, role_name)
Lists client role actions.

Retrieves a list of client role actions.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

[**models::RoleActionsObj**](RoleActionsObj.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_client_role_members

> models::RoleMembersObj list_client_role_members(domain_id, client_id, role_name)
Lists client role members.

Retrieves a list of client role members.  

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
**role_name** | **String** | Role's name. | [required] |

### Return type

[**models::RoleMembersObj**](RoleMembersObj.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_client_roles

> models::RolesPage list_client_roles(domain_id, client_id, limit, offset)
Retrieves clients roles.

Retrieves a list of client roles. Due to performance concerns, data is retrieved in subsets. The API clients must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
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


## update_client_role

> models::Role update_client_role(domain_id, client_id, role_name, update_role_obj)
Updates client role.

Updates a specific client role that is identified by the role name. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
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

