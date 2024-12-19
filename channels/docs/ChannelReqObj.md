# ChannelReqObj

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | Free-form channel name. Channel name is unique on the given hierarchy level. | 
**description** | Option<**String**> | Channel description, free form text. | [optional]
**parent_id** | Option<**String**> | Id of parent channel, it must be existing channel. | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Arbitrary, object-encoded channels's data. | [optional]
**status** | Option<**String**> | Channel Status | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


