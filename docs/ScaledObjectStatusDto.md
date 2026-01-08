# ScaledObjectStatusDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**authentications_types** | Option<**String**> |  | [optional]
**composite_scaler_name** | Option<**String**> |  | [optional]
**conditions** | [**Vec<models::ScaledObjectConditionDto>**](ScaledObjectConditionDto.md) |  | 
**external_metric_names** | **Vec<String>** |  | 
**health** | [**std::collections::HashMap<String, models::ScaledObjectHealthDto>**](ScaledObjectHealthDto.md) |  | 
**hpa_name** | Option<**String**> |  | [optional]
**last_active_time** | **i64** |  | 
**name** | **String** |  | 
**original_replica_count** | Option<**i32**> |  | [optional]
**paused_replica_count** | Option<**i32**> |  | [optional]
**resource_metric_names** | **Vec<String>** |  | 
**scale_target_gvkr** | Option<[**models::ScaleTargetGvkrDto**](ScaleTargetGvkrDto.md)> |  | [optional]
**scale_target_kind** | Option<**String**> |  | [optional]
**triggers_types** | Option<**String**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


