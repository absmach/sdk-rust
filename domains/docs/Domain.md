# Domain

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Domain unique identified. | [optional]
**name** | Option<**String**> | Domain name. | [optional]
**tags** | Option<**Vec<String>**> | domain tags. | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Arbitrary, object-encoded domain's data. | [optional]
**alias** | Option<**String**> | Domain alias. | [optional]
**status** | Option<**String**> | Domain Status | [optional]
**created_by** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | User ID of the user who created the domain. | [optional]
**created_at** | Option<**String**> | Time when the domain was created. | [optional]
**updated_by** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | User ID of the user who last updated the domain. | [optional]
**updated_at** | Option<**String**> | Time when the domain was last updated. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


