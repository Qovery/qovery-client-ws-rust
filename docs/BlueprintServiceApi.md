# \BlueprintServiceApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_blueprint_service_created_request**](BlueprintServiceApi.md#handle_blueprint_service_created_request) | **GET** /blueprint/service-created | 



## handle_blueprint_service_created_request

> String handle_blueprint_service_created_request(organization, cluster, project, environment)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **uuid::Uuid** |  | [required] |
**cluster** | Option<**uuid::Uuid**> |  | [required] |
**project** | **uuid::Uuid** |  | [required] |
**environment** | **uuid::Uuid** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

