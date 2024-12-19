# Key

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | API key unique identifier | [optional]
**issuer_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | In ID of the entity that issued the token. | [optional]
**r#type** | Option<**i32**> | API key type. Keys of different type are processed differently. | [optional]
**subject** | Option<**String**> | User's email or service identifier of API key subject. | [optional]
**issued_at** | Option<**String**> | Time when the key is generated. | [optional]
**expires_at** | Option<**String**> | Time when the Key expires. If this field is missing, that means that Key is valid indefinitely. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


