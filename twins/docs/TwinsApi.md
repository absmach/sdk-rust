# \TwinsApi

All URIs are relative to *http://localhost:9018*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_twin**](TwinsApi.md#create_twin) | **POST** /twins | Adds new twin
[**get_twin**](TwinsApi.md#get_twin) | **GET** /twins/{twinID} | Retrieves twin info
[**get_twins**](TwinsApi.md#get_twins) | **GET** /twins | Retrieves twins
[**remove_twin**](TwinsApi.md#remove_twin) | **DELETE** /twins/{twinID} | Removes a twin
[**update_twin**](TwinsApi.md#update_twin) | **PUT** /twins/{twinID} | Updates twin info



## create_twin

> create_twin(twin_req_obj)
Adds new twin

Adds new twin to the list of twins owned by user identified using the provided access token. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**twin_req_obj** | [**TwinReqObj**](TwinReqObj.md) | JSON-formatted document describing the twin to create or update. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_twin

> models::TwinResObj get_twin(twin_id)
Retrieves twin info

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**twin_id** | **uuid::Uuid** | Unique twin identifier. | [required] |

### Return type

[**models::TwinResObj**](TwinResObj.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_twins

> models::TwinsPage get_twins(limit, offset, name, metadata)
Retrieves twins

Retrieves a list of twins. Due to performance concerns, data is retrieved in subsets. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**limit** | Option<**i32**> | Size of the subset to retrieve. |  |[default to 10]
**offset** | Option<**i32**> | Number of items to skip during retrieval. |  |[default to 0]
**name** | Option<**String**> | Twin name |  |
**metadata** | Option<**String**> | Metadata filter. Filtering is performed matching the parameter with metadata on top level. Parameter is json.  |  |

### Return type

[**models::TwinsPage**](TwinsPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_twin

> remove_twin(twin_id)
Removes a twin

Removes a twin.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**twin_id** | **uuid::Uuid** | Unique twin identifier. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_twin

> update_twin(twin_id, twin_req_obj)
Updates twin info

Update is performed by replacing the current resource data with values provided in a request payload. Note that the twin's ID cannot be changed. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**twin_id** | **uuid::Uuid** | Unique twin identifier. | [required] |
**twin_req_obj** | [**TwinReqObj**](TwinReqObj.md) | JSON-formatted document describing the twin to create or update. | [required] |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

