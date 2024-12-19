# ClientWithEmptySecret

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Client unique identifier. | [optional]
**name** | Option<**String**> | Client name. | [optional]
**tags** | Option<**Vec<String>**> | Client tags. | [optional]
**domain_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | ID of the domain to which client belongs. | [optional]
**credentials** | Option<[**models::ClientWithEmptySecretCredentials**](ClientWithEmptySecret_credentials.md)> |  | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Arbitrary, object-encoded client's data. | [optional]
**status** | Option<**String**> | Client Status | [optional]
**created_at** | Option<**String**> | Time when the channel was created. | [optional]
**updated_at** | Option<**String**> | Time when the channel was created. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


