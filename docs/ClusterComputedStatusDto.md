# ClusterComputedStatusDto

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**global_status** | [**models::ClusterStatusGlobalStatus**](ClusterStatusGlobalStatus.md) |  | 
**is_max_nodes_size_reached** | **bool** |  | 
**kube_version_status** | [**models::QoveryClusterKubeVersionStatus**](QoveryClusterKubeVersionStatus.md) |  | 
**node_warnings** | [**std::collections::HashMap<String, Vec<models::QoveryNodeFailure>>**](Vec.md) |  | 
**qovery_components_in_failure** | [**Vec<models::QoveryComponentInFailure>**](QoveryComponentInFailure.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


