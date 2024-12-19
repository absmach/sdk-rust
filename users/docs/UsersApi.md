# \UsersApi

All URIs are relative to *http://localhost:9002*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_user**](UsersApi.md#create_user) | **POST** /users | Registers user account
[**disable_user**](UsersApi.md#disable_user) | **POST** /users/{userID}/disable | Disables a user
[**domain_id_users_get**](UsersApi.md#domain_id_users_get) | **GET** /{domainID}/users | List users assigned to domain
[**enable_user**](UsersApi.md#enable_user) | **POST** /users/{userID}/enable | Enables a user
[**get_profile**](UsersApi.md#get_profile) | **GET** /users/profile | Gets info on currently logged in user.
[**get_user**](UsersApi.md#get_user) | **GET** /users/{userID} | Retrieves a user
[**issue_token**](UsersApi.md#issue_token) | **POST** /users/tokens/issue | Issue Token
[**list_users**](UsersApi.md#list_users) | **GET** /users | List users
[**list_users_in_channel**](UsersApi.md#list_users_in_channel) | **GET** /{domainID}/channels/{channelID}/users | List users in a channel
[**list_users_in_client**](UsersApi.md#list_users_in_client) | **GET** /{domainID}/clients/{clientID}/users | List users associated with a client
[**list_users_in_group**](UsersApi.md#list_users_in_group) | **GET** /{domainID}/groups/{groupID}/users | List users in a group
[**refresh_token**](UsersApi.md#refresh_token) | **POST** /users/tokens/refresh | Refresh Token
[**request_password_reset**](UsersApi.md#request_password_reset) | **POST** /password/reset-request | User password reset request
[**reset_password**](UsersApi.md#reset_password) | **PUT** /password/reset | User password reset endpoint
[**search_users**](UsersApi.md#search_users) | **GET** /users/search | Search users
[**update_email**](UsersApi.md#update_email) | **PATCH** /users/{userID}/email | Updates email of the user.
[**update_profile_picture**](UsersApi.md#update_profile_picture) | **PATCH** /users/{userID}/picture | Updates the user's profile picture.
[**update_role**](UsersApi.md#update_role) | **PATCH** /users/{userID}/role | Updates the user's role.
[**update_secret**](UsersApi.md#update_secret) | **PATCH** /users/secret | Updates secret of currently logged in user.
[**update_tags**](UsersApi.md#update_tags) | **PATCH** /users/{userID}/tags | Updates tags of the user.
[**update_user**](UsersApi.md#update_user) | **PATCH** /users/{userID} | Updates first, last name and metadata of the user.
[**update_username**](UsersApi.md#update_username) | **PATCH** /users/{userID}/username | Updates user's username.
[**users_user_id_delete**](UsersApi.md#users_user_id_delete) | **DELETE** /users/{userID} | Delete a user



## create_user

> models::User create_user(user_req_obj)
Registers user account

Registers new user account given email and password. New account will be uniquely identified by its email address. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_req_obj** | [**UserReqObj**](UserReqObj.md) | JSON-formatted document describing the new user to be registered | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[refreshAuth](../README.md#refreshAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disable_user

> models::User disable_user(user_id)
Disables a user

Disables a specific user that is identifier by the user ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## enable_user

> models::User enable_user(user_id)
Enables a user

Enables a specific user that is identifier by the user ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_profile

> models::User get_profile()
Gets info on currently logged in user.

Gets info on currently logged in user. Info is obtained using authorization token 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_user

> models::User get_user(user_id)
Retrieves a user

Retrieves a specific user that is identifier by the user ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## issue_token

> models::IssueToken200Response issue_token(issue_token)
Issue Token

Issue Access and Refresh Token used for authenticating into the system. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**issue_token** | [**IssueToken**](IssueToken.md) | Login credentials. | [required] |

### Return type

[**models::IssueToken200Response**](issueToken_200_response.md)

### Authorization

[refreshAuth](../README.md#refreshAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users

> models::UsersPage list_users(limit, offset, metadata, status, first_name, last_name, username, email, tags)
List users

Retrieves a list of users. Due to performance concerns, data is retrieved in subsets. The API must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**metadata** | Option<**String**> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json. |  |
**status** | Option<**String**> | User account status. |  |[default to enabled]
**first_name** | Option<**String**> | User's first name. |  |
**last_name** | Option<**String**> | User's last name. |  |
**username** | Option<**String**> | User's username. |  |
**email** | Option<**String**> | User's email address. |  |
**tags** | Option<[**Vec<String>**](String.md)> | User tags. |  |

### Return type

[**models::UsersPage**](UsersPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users_in_channel

> models::MembersPage list_users_in_channel(domain_id, channel_id, limit, offset, level, tree, metadata, name, parent_id)
List users in a channel

Retrieves a list of users in a channel. Due to performance concerns, data is retrieved in subsets. The API must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**channel_id** | **uuid::Uuid** | Unique channel identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**level** | Option<**i32**> | Level of hierarchy up to which to retrieve groups from given group id. |  |
**tree** | Option<**bool**> | Specify type of response, JSON array or tree. |  |[default to false]
**metadata** | Option<**String**> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json. |  |
**name** | Option<**String**> | Channel's name. |  |
**parent_id** | Option<**uuid::Uuid**> | Unique parent identifier for a group. |  |

### Return type

[**models::MembersPage**](MembersPage.md)

### Authorization

[refreshAuth](../README.md#refreshAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users_in_client

> models::MembersPage list_users_in_client(domain_id, client_id, limit, offset, level, tree, metadata, name, parent_id)
List users associated with a client

Retrieves a list of users associated with a client. Due to performance concerns, data is retrieved in subsets. The API must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Unique client identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**level** | Option<**i32**> | Level of hierarchy up to which to retrieve groups from given group id. |  |
**tree** | Option<**bool**> | Specify type of response, JSON array or tree. |  |[default to false]
**metadata** | Option<**String**> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json. |  |
**name** | Option<**String**> | Channel's name. |  |
**parent_id** | Option<**uuid::Uuid**> | Unique parent identifier for a group. |  |

### Return type

[**models::MembersPage**](MembersPage.md)

### Authorization

[refreshAuth](../README.md#refreshAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_users_in_group

> models::MembersPage list_users_in_group(domain_id, group_id, limit, offset, level, tree, metadata, name, parent_id)
List users in a group

Retrieves a list of users in a group. Due to performance concerns, data is retrieved in subsets. The API must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 

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

[**models::MembersPage**](MembersPage.md)

### Authorization

[refreshAuth](../README.md#refreshAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## refresh_token

> models::IssueToken200Response refresh_token()
Refresh Token

Refreshes Access and Refresh Token used for authenticating into the system. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IssueToken200Response**](issueToken_200_response.md)

### Authorization

[refreshAuth](../README.md#refreshAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## request_password_reset

> request_password_reset(referer, request_password_reset_request)
User password reset request

Generates a reset token and sends and email with link for resetting password. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**referer** | **String** | Host being sent by browser. | [required] |
**request_password_reset_request** | [**RequestPasswordResetRequest**](RequestPasswordResetRequest.md) | Initiate password request procedure. | [required] |

### Return type

 (empty response body)

### Authorization

[refreshAuth](../README.md#refreshAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reset_password

> reset_password(reset_password_request)
User password reset endpoint

When user gets reset token, after he submitted email to `/password/reset-request`, posting a   new password along to this endpoint will change password. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reset_password_request** | Option<[**ResetPasswordRequest**](ResetPasswordRequest.md)> | Password reset request data, new password and token that is appended on password reset link received in email. |  |

### Return type

 (empty response body)

### Authorization

[refreshAuth](../README.md#refreshAuth), [bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_users

> models::UsersPage search_users(user_id, limit, offset, username, first_name, last_name, email)
Search users

Search users by name and identity. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**username** | Option<**String**> | User's username. |  |
**first_name** | Option<**String**> | User's first name. |  |
**last_name** | Option<**String**> | User's last name. |  |
**email** | Option<**String**> | User's email address. |  |

### Return type

[**models::UsersPage**](UsersPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_email

> models::User update_email(user_id, email)
Updates email of the user.

Updates email of the user with provided ID. Email is updated using authorization token and the new received email. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |
**email** | [**Email**](Email.md) | Email change data. User can change its email. | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_profile_picture

> models::User update_profile_picture(user_id, user_profile_picture)
Updates the user's profile picture.

Updates the user's profile picture with provided ID. Profile picture is updated using authorization token and the new received picture. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |
**user_profile_picture** | [**UserProfilePicture**](UserProfilePicture.md) | JSON-formated document describing the profile picture of user to be update | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_role

> models::User update_role(user_id, user_role)
Updates the user's role.

Updates role for the user with provided ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |
**user_role** | [**UserRole**](UserRole.md) | JSON-formated document describing the role of the user to be updated | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_secret

> models::User update_secret(user_secret)
Updates secret of currently logged in user.

Updates secret of currently logged in user. Secret is updated using authorization token and the new received info. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_secret** | [**UserSecret**](UserSecret.md) | Secret change data. User can change its secret. | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_tags

> models::User update_tags(user_id, user_tags)
Updates tags of the user.

Updates tags of the user with provided ID. Tags is updated using authorization token and the new tags received in request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |
**user_tags** | [**UserTags**](UserTags.md) | JSON-formated document describing the tags of user to be update | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> models::User update_user(user_id, user_update)
Updates first, last name and metadata of the user.

Updates name and metadata of the user with provided ID. Name and metadata is updated using authorization token and the new received info. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |
**user_update** | [**UserUpdate**](UserUpdate.md) | JSON-formated document describing the metadata and name of user to be update | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_username

> models::User update_username(user_id, username)
Updates user's username.

Updates username of the user with provided ID. Username is updated using authorization token and the new received username. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |
**username** | [**Username**](Username.md) | JSON-formated document describing the username of the user to be updated | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## users_user_id_delete

> users_user_id_delete(user_id)
Delete a user

Delete a specific user that is identifier by the user ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

