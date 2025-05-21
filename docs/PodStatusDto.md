# PodStatusDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**containers** | [**Vec<models::ContainerStatusDto>**](ContainerStatusDto.md) |  | 
**last_events** | [**Vec<models::PodKubernetesEventDto>**](PodKubernetesEventDto.md) |  | 
**name** | **String** |  | 
**restart_count** | **i32** |  | 
**service_version** | **String** |  | 
**started_at** | **i64** |  | 
**state** | [**models::ServiceStateDto**](ServiceStateDto.md) |  | 
**state_message** | **String** |  | 
**state_reason** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


