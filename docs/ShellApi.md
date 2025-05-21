# \ShellApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_shell_exec**](ShellApi.md#handle_shell_exec) | **GET** /shell/exec | 



## handle_shell_exec

> String handle_shell_exec(organization, cluster, project, environment, service, pod_name, container_name, command, tty_width, tty_height)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **uuid::Uuid** |  | [required] |
**cluster** | **uuid::Uuid** |  | [required] |
**project** | **uuid::Uuid** |  | [required] |
**environment** | **uuid::Uuid** |  | [required] |
**service** | **uuid::Uuid** |  | [required] |
**pod_name** | Option<**String**> |  | [required] |
**container_name** | Option<**String**> |  | [required] |
**command** | [**Vec<String>**](String.md) |  | [required] |
**tty_width** | **i32** |  | [required] |
**tty_height** | **i32** |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

