# \RemoteDebugApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_shell_remote_debug**](RemoteDebugApi.md#handle_shell_remote_debug) | **GET** /shell/debug | 



## handle_shell_remote_debug

> String handle_shell_remote_debug(organization, cluster, flavor, tty_width, tty_height, node_selector)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **uuid::Uuid** |  | [required] |
**cluster** | **uuid::Uuid** |  | [required] |
**flavor** | [**DebugFlavor**](.md) |  | [required] |
**tty_width** | **i32** |  | [required] |
**tty_height** | **i32** |  | [required] |
**node_selector** | Option<**String**> |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/plain

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

