# CreateConfigRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**external_id** | **String** | External ID (MAC address or some unique identifier). | 
**external_key** | **String** | External key. | 
**client_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | ID of the corresponding SuperMQ Client. | [optional]
**channels** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> |  | [optional]
**content** | Option<**String**> |  | [optional]
**name** | Option<**String**> |  | [optional]
**client_cert** | Option<**String**> | Client Certificate. | [optional]
**client_key** | Option<**String**> | Client Private Key. | [optional]
**ca_cert** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


