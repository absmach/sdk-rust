# GroupReqObj

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Free-form group name. Group name is unique on the given hierarchy level. | 
**description** | Option<**String**> | Group description, free form text. | [optional]
**parent_id** | Option<**String**> | Id of parent group, it must be existing group. | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Arbitrary, object-encoded groups's data. | [optional]
**status** | Option<**String**> | Group Status | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


