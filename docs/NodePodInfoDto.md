# NodePodInfoDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**cpu_milli_limit** | Option<**i32**> |  | [optional]
**cpu_milli_request** | Option<**i32**> |  | [optional]
**created_at** | **i64** |  | 
**ephemeral_storage_mib_limit** | Option<**i32**> |  | [optional]
**ephemeral_storage_mib_request** | Option<**i32**> |  | [optional]
**error_container_statuses** | [**Vec<models::NodePodErrorStatusDto>**](NodePodErrorStatusDto.md) |  | 
**images_version** | **std::collections::HashMap<String, String>** |  | 
**memory_mib_limit** | Option<**i32**> |  | [optional]
**memory_mib_request** | Option<**i32**> |  | [optional]
**metrics_usage** | [**models::MetricsUsageDto**](MetricsUsageDto.md) |  | 
**name** | **String** |  | 
**namespace** | **String** |  | 
**qovery_service_info** | Option<[**models::PodQoveryServiceInfoDto**](PodQoveryServiceInfoDto.md)> |  | [optional]
**restart_count** | **i32** |  | 
**status_phase** | [**models::PodStatusPhase**](PodStatusPhase.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


