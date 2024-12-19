# Members

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | User unique identifier. | [optional]
**first_name** | Option<**String**> | User's first name. | [optional]
**last_name** | Option<**String**> | User's last name. | [optional]
**email** | Option<**String**> | User's email address. | [optional]
**tags** | Option<**Vec<String>**> | User tags. | [optional]
**credentials** | Option<[**models::MembersCredentials**](Members_credentials.md)> |  | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Arbitrary, object-encoded user's data. | [optional]
**status** | Option<**String**> | User Status | [optional]
**created_at** | Option<**String**> | Time when the group was created. | [optional]
**updated_at** | Option<**String**> | Time when the group was created. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


