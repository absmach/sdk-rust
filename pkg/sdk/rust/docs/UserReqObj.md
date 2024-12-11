# UserReqObj

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**first_name** | Option<**String**> | User's first name. | [optional]
**last_name** | Option<**String**> | User's last name. | [optional]
**email** | Option<**String**> | User's email address will be used as its unique identifier. | [optional]
**tags** | Option<**Vec<String>**> | User tags. | [optional]
**credentials** | [**models::UserReqObjCredentials**](UserReqObj_credentials.md) |  | 
**metadata** | Option<[**serde_json::Value**](.md)> | Arbitrary, object-encoded user's data. | [optional]
**profile_picture** | Option<**String**> | User's profile picture URL that is represented as a string. | [optional]
**status** | Option<**String**> | User Status | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


