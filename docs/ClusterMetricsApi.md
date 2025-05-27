# \ClusterMetricsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_cluster_metrics_request**](ClusterMetricsApi.md#handle_cluster_metrics_request) | **GET** /cluster/metrics | 



## handle_cluster_metrics_request

> models::ClusterStatusDto handle_cluster_metrics_request(organization, cluster)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **uuid::Uuid** |  | [required] |
**cluster** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ClusterStatusDto**](ClusterStatusDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

