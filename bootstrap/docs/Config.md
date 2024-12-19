# Config

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**client_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Corresponding SuperMQ Client ID. | [optional]
**magistrala_secret** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Corresponding SuperMQ Client key. | [optional]
**channels** | Option<[**Vec<models::ConfigChannelsInner>**](Config_channels_inner.md)> |  | [optional]
**external_id** | **String** | External ID (MAC address or some unique identifier). | 
**external_key** | **String** | External key. | 
**content** | Option<**String**> | Free-form custom configuration. | [optional]
**state** | Option<[**models::State**](State.md)> |  | [optional]
**client_cert** | Option<**String**> | Client certificate. | [optional]
**ca_cert** | Option<**String**> | Issuing CA certificate. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


