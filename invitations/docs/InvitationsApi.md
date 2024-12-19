# \InvitationsApi

All URIs are relative to *http://localhost:9020*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accept_invitation**](InvitationsApi.md#accept_invitation) | **POST** /invitations/accept | Accept invitation
[**delete_invitation**](InvitationsApi.md#delete_invitation) | **DELETE** /invitations/{user_id}/{domain_id} | Deletes a specific invitation
[**get_invitation**](InvitationsApi.md#get_invitation) | **GET** /invitations/{user_id}/{domain_id} | Retrieves a specific invitation
[**list_invitations**](InvitationsApi.md#list_invitations) | **GET** /invitations | List invitations
[**reject_invitation**](InvitationsApi.md#reject_invitation) | **POST** /invitations/reject | Reject invitation
[**send_invitation**](InvitationsApi.md#send_invitation) | **POST** /invitations | Send invitation



## accept_invitation

> accept_invitation(accept_invitation_request)
Accept invitation

Current logged in user accepts invitation to join domain. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_invitation_request** | [**AcceptInvitationRequest**](AcceptInvitationRequest.md) | JSON-formatted document describing request for accepting invitation | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_invitation

> delete_invitation(user_id, domain_id)
Deletes a specific invitation

Deletes a specific invitation that is identifier by the user ID and domain ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |
**domain_id** | **uuid::Uuid** | Unique identifier for a domain. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_invitation

> models::Invitation get_invitation(user_id, domain_id)
Retrieves a specific invitation

Retrieves a specific invitation that is identifier by the user ID and domain ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |
**domain_id** | **uuid::Uuid** | Unique identifier for a domain. | [required] |

### Return type

[**models::Invitation**](Invitation.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_invitations

> models::InvitationPage list_invitations(user_id, limit, offset, invited_by, domain_id, relation, state)
List invitations

Retrieves a list of invitations. Due to performance concerns, data is retrieved in subsets. The API must ensure that the entire dataset is consumed either by making subsequent requests, or by increasing the subset size of the initial request. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | Unique user identifier. | [required] |
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**invited_by** | Option<**uuid::Uuid**> | Unique identifier for a user that invited the user. |  |
**domain_id** | Option<**uuid::Uuid**> | Unique identifier for a domain. |  |
**relation** | Option<**String**> | Relation between user and domain. |  |
**state** | Option<**String**> | Invitation state. |  |

### Return type

[**models::InvitationPage**](InvitationPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reject_invitation

> reject_invitation(accept_invitation_request)
Reject invitation

Current logged in user rejects invitation to join domain. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**accept_invitation_request** | [**AcceptInvitationRequest**](AcceptInvitationRequest.md) | JSON-formatted document describing request for accepting invitation | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_invitation

> send_invitation(send_invitation_req_obj)
Send invitation

Send invitation to user to join domain. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**send_invitation_req_obj** | [**SendInvitationReqObj**](SendInvitationReqObj.md) | JSON-formatted document describing request for sending invitation | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

