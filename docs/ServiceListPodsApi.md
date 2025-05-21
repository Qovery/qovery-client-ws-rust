# \ServiceListPodsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_service_list_pods_request**](ServiceListPodsApi.md#handle_service_list_pods_request) | **GET** /service/pods | 



## handle_service_list_pods_request

> models::ServiceListPodsResponseDto handle_service_list_pods_request(organization, cluster, project, environment, service)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **uuid::Uuid** |  | [required] |
**cluster** | **uuid::Uuid** |  | [required] |
**project** | **uuid::Uuid** |  | [required] |
**environment** | **uuid::Uuid** |  | [required] |
**service** | **uuid::Uuid** |  | [required] |

### Return type

[**models::ServiceListPodsResponseDto**](ServiceListPodsResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

