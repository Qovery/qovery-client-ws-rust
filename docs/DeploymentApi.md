# \DeploymentApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_deployment_logs_request**](DeploymentApi.md#handle_deployment_logs_request) | **GET** /deployment/logs | 
[**handle_deployment_status_request**](DeploymentApi.md#handle_deployment_status_request) | **GET** /deployment/status | 



## handle_deployment_logs_request

> String handle_deployment_logs_request(organization, cluster, project, environment, version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **uuid::Uuid** |  | [required] |
**cluster** | Option<**uuid::Uuid**> |  | [required] |
**project** | **uuid::Uuid** |  | [required] |
**environment** | **uuid::Uuid** |  | [required] |
**version** | Option<**String**> |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## handle_deployment_status_request

> String handle_deployment_status_request(organization, cluster, project, environment, version)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **uuid::Uuid** |  | [required] |
**cluster** | Option<**uuid::Uuid**> |  | [required] |
**project** | **uuid::Uuid** |  | [required] |
**environment** | Option<**uuid::Uuid**> |  | [required] |
**version** | Option<**String**> |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

