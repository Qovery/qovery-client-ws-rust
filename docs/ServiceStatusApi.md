# \ServiceStatusApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_service_status_request**](ServiceStatusApi.md#handle_service_status_request) | **GET** /service/status | 



## handle_service_status_request

> models::ServiceStatusDto handle_service_status_request(organization, cluster, project, environment)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **uuid::Uuid** |  | [required] |
**cluster** | **uuid::Uuid** |  | [required] |
**project** | Option<**uuid::Uuid**> |  | [required] |
**environment** | Option<**uuid::Uuid**> |  | [required] |

### Return type

[**models::ServiceStatusDto**](ServiceStatusDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

