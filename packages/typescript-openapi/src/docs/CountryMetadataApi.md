# CountryMetadataApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**countriesListCallingCodes**](#countrieslistcallingcodes) | **GET** /api/countries/calling_codes | |
|[**countriesListNames**](#countrieslistnames) | **GET** /api/countries/names | |

# **countriesListCallingCodes**
> Array<APICountryWithCallingCode> countriesListCallingCodes()


### Example

```typescript
import {
    CountryMetadataApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new CountryMetadataApi(configuration);

const { status, data } = await apiInstance.countriesListCallingCodes();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<APICountryWithCallingCode>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **countriesListNames**
> Array<APICountryWithISOCode> countriesListNames()


### Example

```typescript
import {
    CountryMetadataApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new CountryMetadataApi(configuration);

const { status, data } = await apiInstance.countriesListNames();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<APICountryWithISOCode>**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

