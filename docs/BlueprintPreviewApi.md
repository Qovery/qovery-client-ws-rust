# \BlueprintPreviewApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**handle_blueprint_preview_request**](BlueprintPreviewApi.md#handle_blueprint_preview_request) | **GET** /blueprint/preview | 



## handle_blueprint_preview_request

> models::BlueprintPreviewResult handle_blueprint_preview_request(organization, cluster, preview_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **uuid::Uuid** | Organization id — for permission scoping. Frontend already has it. | [required] |
**cluster** | **uuid::Uuid** | Cluster id — for permission scoping. Frontend already has it. | [required] |
**preview_id** | **uuid::Uuid** | Preview id — the UUIDv7 q-core returned from `POST /api/blueprint/{id}/update/preview`. Doubles as the Pub/Sub channel suffix: `core.blueprint.preview.{preview_id}`. | [required] |

### Return type

[**models::BlueprintPreviewResult**](BlueprintPreviewResult.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

