# default_api

All URIs are relative to *http://localhost:9090/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
**ping**](default_api.md#ping) | **GET** /ping | 疎通確認
**updateMergedImage**](default_api.md#updateMergedImage) | **PUT** /merged-images | 複数画像を束ねたファイルの指定枚目だけ更新する
**uploadImage**](default_api.md#uploadImage) | **POST** /images | １枚の画像をDDS形式に変換し、ストレージにアップロードする
**uploadMergedImage**](default_api.md#uploadMergedImage) | **POST** /merged-images | 複数枚の画像をDDS形式に変換し、1ファイルにまとめ、ストレージにアップロードする


# **ping**
> models::SuccessResponse ping()
疎通確認

### Required Parameters
This endpoint does not need any parameter.

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateMergedImage**
> models::SuccessResponse updateMergedImage(signed_url, index, metadata, file)
複数画像を束ねたファイルの指定枚目だけ更新する

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **signed_url** | **String**| ストレージサービスの署名付きURL | 
  **index** | **i32**| 束ねたファイルの指定枚目 | 
  **metadata** | **String**| 画像ファイルのメタデータのJSON配列 | 
  **file** | **swagger::ByteArray**|  | 

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: multipart/form-data
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **uploadImage**
> models::SuccessResponse uploadImage(signed_url, metadata, optional)
１枚の画像をDDS形式に変換し、ストレージにアップロードする

UdonのStringLoadingの制約により、結果ファイルは10MB以下でないといけない

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **signed_url** | **String**| ストレージサービスの署名付きURL | 
  **metadata** | **String**| 画像ファイルのメタデータのJSON配列 | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **signed_url** | **String**| ストレージサービスの署名付きURL | 
 **metadata** | **String**| 画像ファイルのメタデータのJSON配列 | 
 **file** | **swagger::ByteArray**|  | 

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: multipart/form-data
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **uploadMergedImage**
> models::SuccessResponse uploadMergedImage(signed_url, metadata, optional)
複数枚の画像をDDS形式に変換し、1ファイルにまとめ、ストレージにアップロードする

UdonのStringLoadingの制約により、結果ファイルは10MB以下でないといけない

### Required Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
  **signed_url** | **String**| ストレージサービスの署名付きURL | 
  **metadata** | **String**| 画像ファイルのメタデータのJSON配列 | 
 **optional** | **map[string]interface{}** | optional parameters | nil if no parameters

### Optional Parameters
Optional parameters are passed through a map[string]interface{}.

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **signed_url** | **String**| ストレージサービスの署名付きURL | 
 **metadata** | **String**| 画像ファイルのメタデータのJSON配列 | 
 **files** | [**swagger::ByteArray**](swagger::ByteArray.md)|  | 

### Return type

[**models::SuccessResponse**](SuccessResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: multipart/form-data
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

