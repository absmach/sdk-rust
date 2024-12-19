# \CertsApi

All URIs are relative to *http://localhost:9019*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_cert**](CertsApi.md#create_cert) | **POST** /{domainID}/certs | Creates a certificate for client
[**get_cert**](CertsApi.md#get_cert) | **GET** /{domainID}/certs/{certID} | Retrieves a certificate
[**get_serials**](CertsApi.md#get_serials) | **GET** /{domainID}/serials/{clientID} | Retrieves certificates' serial IDs
[**revoke_cert**](CertsApi.md#revoke_cert) | **DELETE** /{domainID}/certs/{certID} | Revokes a certificate



## create_cert

> create_cert(domain_id, create_cert_request)
Creates a certificate for client

Creates a certificate for client

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**create_cert_request** | Option<[**CreateCertRequest**](CreateCertRequest.md)> | Issues a certificate that is required for mTLS. To create a certificate for a client provide a client id, data identifying particular client will be embedded into the Certificate. x509 and ECC certificates are supported when using when Vault is used as PKI.  |  |

### Return type

 (empty response body)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_cert

> models::Cert get_cert(domain_id, cert_id)
Retrieves a certificate

Retrieves a certificate for a given cert ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**cert_id** | **uuid::Uuid** | Serial of certificate | [required] |

### Return type

[**models::Cert**](Cert.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_serials

> models::SerialsPage get_serials(domain_id, client_id)
Retrieves certificates' serial IDs

Retrieves a list of certificates' serial IDs for a given client ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**client_id** | **uuid::Uuid** | Client ID | [required] |

### Return type

[**models::SerialsPage**](SerialsPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## revoke_cert

> models::Revoke revoke_cert(domain_id, cert_id)
Revokes a certificate

Revokes a certificate for a given cert ID. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**domain_id** | **uuid::Uuid** | Unique domain identifier. | [required] |
**cert_id** | **uuid::Uuid** | Serial of certificate | [required] |

### Return type

[**models::Revoke**](Revoke.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

