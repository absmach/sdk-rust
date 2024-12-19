# \NotifiersApi

All URIs are relative to *http://localhost:9014*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_subscription**](NotifiersApi.md#create_subscription) | **POST** /subscriptions | Create subscription
[**list_subscriptions**](NotifiersApi.md#list_subscriptions) | **GET** /subscriptions | List subscriptions
[**remove_subscription**](NotifiersApi.md#remove_subscription) | **DELETE** /subscriptions/{id} | Delete subscription with the provided id
[**view_subscription**](NotifiersApi.md#view_subscription) | **GET** /subscriptions/{id} | Get subscription with the provided id



## create_subscription

> create_subscription(create_subscription)
Create subscription

Creates a new subscription give a topic and contact.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_subscription** | [**CreateSubscription**](CreateSubscription.md) | JSON-formatted document describing the new subscription to be created | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_subscriptions

> models::Page list_subscriptions(topic, contact, offset, limit)
List subscriptions

List subscriptions given list parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**topic** | Option<**String**> | Topic name. |  |
**contact** | Option<**String**> | Subscription contact. |  |
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]

### Return type

[**models::Page**](Page.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_subscription

> remove_subscription(id)
Delete subscription with the provided id

Removes a subscription with the provided id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## view_subscription

> models::Subscription view_subscription(id)
Get subscription with the provided id

Retrieves a subscription with the provided id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Unique identifier. | [required] |

### Return type

[**models::Subscription**](Subscription.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

