# \ClusterStatusApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_cluster_status_request**](ClusterStatusApi.md#handle_cluster_status_request) | **GET** /cluster/status | 



## handle_cluster_status_request

> models::ClusterStatusDto handle_cluster_status_request(organization, cluster)


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

