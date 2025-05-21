# \ServiceMetricsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_metrics_request**](ServiceMetricsApi.md#handle_metrics_request) | **GET** /service/metrics | 



## handle_metrics_request

> models::ServiceMetricsDto handle_metrics_request(organization, cluster, project, environment, service, service_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **uuid::Uuid** |  | [required] |
**cluster** | **uuid::Uuid** |  | [required] |
**project** | **uuid::Uuid** |  | [required] |
**environment** | **uuid::Uuid** |  | [required] |
**service** | **uuid::Uuid** |  | [required] |
**service_type** | [**ServiceType**](.md) |  | [required] |

### Return type

[**models::ServiceMetricsDto**](ServiceMetricsDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

