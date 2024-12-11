# User

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | User unique identifier. | [optional]
**first_name** | Option<**String**> | User's first name. | [optional]
**last_name** | Option<**String**> | User's last name. | [optional]
**tags** | Option<**Vec<String>**> | User tags. | [optional]
**email** | Option<**String**> | User email for example email address. | [optional]
**credentials** | Option<[**models::UserCredentials**](User_credentials.md)> |  | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Arbitrary, object-encoded user's data. | [optional]
**profile_picture** | Option<**String**> | User's profile picture URL that is represented as a string. | [optional]
**status** | Option<**String**> | User Status | [optional]
**created_at** | Option<**String**> | Time when the group was created. | [optional]
**updated_at** | Option<**String**> | Time when the group was created. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


