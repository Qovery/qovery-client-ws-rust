# \ClusterListNodesApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_cluster_list_nodes_request**](ClusterListNodesApi.md#handle_cluster_list_nodes_request) | **GET** /cluster/node | 



## handle_cluster_list_nodes_request

> models::ClusterListNodesResponseDto handle_cluster_list_nodes_request(organization, cluster)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **uuid::Uuid** |  | [required] |
**cluster** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ClusterListNodesResponseDto**](ClusterListNodesResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

