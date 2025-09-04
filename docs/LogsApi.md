# \LogsApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_infra_logs_request**](LogsApi.md#handle_infra_logs_request) | **GET** /infra/logs | 
[**handle_service_logs_request**](LogsApi.md#handle_service_logs_request) | **GET** /service/logs | 



## handle_infra_logs_request

> models::ServiceInfraLogResponseDto handle_infra_logs_request(organization, cluster, project, environment, service, infra_component_type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **uuid::Uuid** |  | [required] |
**cluster** | **uuid::Uuid** |  | [required] |
**project** | Option<**uuid::Uuid**> |  | [required] |
**environment** | Option<**uuid::Uuid**> |  | [required] |
**service** | Option<**uuid::Uuid**> |  | [required] |
**infra_component_type** | **String** |  | [required] |

### Return type

[**models::ServiceInfraLogResponseDto**](ServiceInfraLogResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## handle_service_logs_request

> models::ServiceLogResponseDto handle_service_logs_request(organization, cluster, project, environment, service, pod_name, deployment_id, query, start)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **uuid::Uuid** |  | [required] |
**cluster** | **uuid::Uuid** |  | [required] |
**project** | **uuid::Uuid** |  | [required] |
**environment** | **uuid::Uuid** |  | [required] |
**service** | **uuid::Uuid** |  | [required] |
**pod_name** | Option<**String**> |  | [required] |
**deployment_id** | Option<**String**> |  | [required] |
**query** | Option<**String**> |  | [required] |
**start** | Option<**String**> |  | [required] |

### Return type

[**models::ServiceLogResponseDto**](ServiceLogResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

