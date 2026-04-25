# AuthenticationApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**authCallback**](#authcallback) | **POST** /api/auth/{org_id}/callback | Process authentication callback|
|[**authInvalidateToken**](#authinvalidatetoken) | **DELETE** /api/auth/tokens/current | |
|[**authResendVerification**](#authresendverification) | **POST** /api/auth/verification/resend | |
|[**authSignIn**](#authsignin) | **POST** /api/auth/signin/signin | |
|[**authSignUp**](#authsignup) | **POST** /api/auth/signup | |
|[**authSocialCallback**](#authsocialcallback) | **POST** /api/auth/social/callback | |
|[**authSocialList**](#authsociallist) | **GET** /api/auth/social/providers | |
|[**authSocialStart**](#authsocialstart) | **POST** /api/auth/social/start | |
|[**authStart**](#authstart) | **POST** /api/auth/{org_id}/start | Start authentication flow|
|[**authTest**](#authtest) | **GET** /api/auth/test | Test authentication|
|[**authVerify**](#authverify) | **POST** /api/auth/verification/{verification_id} | |
|[**authVerifyTfa**](#authverifytfa) | **POST** /api/auth/signin/verify_tfa | |

# **authCallback**
> AuthCallbackResponse authCallback(authCallbackRequest)

\\ Handle the callback from the OIDC server and generate an API key to be used for future requests

### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    AuthCallbackRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let orgId: string; // (default to undefined)
let authCallbackRequest: AuthCallbackRequest; //

const { status, data } = await apiInstance.authCallback(
    orgId,
    authCallbackRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **authCallbackRequest** | **AuthCallbackRequest**|  | |
| **orgId** | [**string**] |  | defaults to undefined|


### Return type

**AuthCallbackResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Bad Request |  -  |
|**402** | Payment Required |  -  |
|**403** | Forbidden |  -  |
|**404** | Not Found |  -  |
|**422** | Unprocessable Entity |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **authInvalidateToken**
> authInvalidateToken()


### Example

```typescript
import {
    AuthenticationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

const { status, data } = await apiInstance.authInvalidateToken();
```

### Parameters
This endpoint does not have any parameters.


### Return type

void (empty response body)

### Authorization

[api_auth_token](../README.md#api_auth_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Bad Request |  -  |
|**402** | Payment Required |  -  |
|**403** | Forbidden |  -  |
|**404** | Not Found |  -  |
|**422** | Unprocessable Entity |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **authResendVerification**
> authResendVerification(resendVerificationRequest)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    ResendVerificationRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let resendVerificationRequest: ResendVerificationRequest; //

const { status, data } = await apiInstance.authResendVerification(
    resendVerificationRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **resendVerificationRequest** | **ResendVerificationRequest**|  | |


### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Bad Request |  -  |
|**402** | Payment Required |  -  |
|**403** | Forbidden |  -  |
|**404** | Not Found |  -  |
|**422** | Unprocessable Entity |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **authSignIn**
> SignInResponse authSignIn(signInRequest)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    SignInRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let signInRequest: SignInRequest; //

const { status, data } = await apiInstance.authSignIn(
    signInRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **signInRequest** | **SignInRequest**|  | |


### Return type

**SignInResponse**

### Authorization

[verified_captcha](../README.md#verified_captcha)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Bad Request |  -  |
|**402** | Payment Required |  -  |
|**403** | Forbidden |  -  |
|**404** | Not Found |  -  |
|**422** | Unprocessable Entity |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **authSignUp**
> authSignUp(createUserRequest)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    CreateUserRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let createUserRequest: CreateUserRequest; //

const { status, data } = await apiInstance.authSignUp(
    createUserRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **createUserRequest** | **CreateUserRequest**|  | |


### Return type

void (empty response body)

### Authorization

[verified_captcha](../README.md#verified_captcha)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Bad Request |  -  |
|**402** | Payment Required |  -  |
|**403** | Forbidden |  -  |
|**404** | Not Found |  -  |
|**422** | Unprocessable Entity |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **authSocialCallback**
> SocialAuthCallbackResponse authSocialCallback(socialAuthCallbackRequest)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    SocialAuthCallbackRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let socialAuthCallbackRequest: SocialAuthCallbackRequest; //

const { status, data } = await apiInstance.authSocialCallback(
    socialAuthCallbackRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **socialAuthCallbackRequest** | **SocialAuthCallbackRequest**|  | |


### Return type

**SocialAuthCallbackResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Bad Request |  -  |
|**402** | Payment Required |  -  |
|**403** | Forbidden |  -  |
|**404** | Not Found |  -  |
|**422** | Unprocessable Entity |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **authSocialList**
> ListSocialAuthProvidersResponse authSocialList()


### Example

```typescript
import {
    AuthenticationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

const { status, data } = await apiInstance.authSocialList();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**ListSocialAuthProvidersResponse**

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

# **authSocialStart**
> StartSocialAuthResponse authSocialStart(startSocialAuthRequest)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    StartSocialAuthRequest
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let startSocialAuthRequest: StartSocialAuthRequest; //

const { status, data } = await apiInstance.authSocialStart(
    startSocialAuthRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **startSocialAuthRequest** | **StartSocialAuthRequest**|  | |


### Return type

**StartSocialAuthResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Bad Request |  -  |
|**402** | Payment Required |  -  |
|**403** | Forbidden |  -  |
|**404** | Not Found |  -  |
|**422** | Unprocessable Entity |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **authStart**
> StartAuthResponse authStart()

\\ Generate an OIDC URL with the configured provider to start authentication

### Example

```typescript
import {
    AuthenticationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let orgId: string; // (default to undefined)
let redirectUrl: string; // (default to undefined)

const { status, data } = await apiInstance.authStart(
    orgId,
    redirectUrl
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **orgId** | [**string**] |  | defaults to undefined|
| **redirectUrl** | [**string**] |  | defaults to undefined|


### Return type

**StartAuthResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Bad Request |  -  |
|**402** | Payment Required |  -  |
|**403** | Forbidden |  -  |
|**404** | Not Found |  -  |
|**422** | Unprocessable Entity |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **authTest**
> AuthTestResponse authTest()

\\ Checks whether the provided Authorization header is valid and returns the user\'s ID

### Example

```typescript
import {
    AuthenticationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

const { status, data } = await apiInstance.authTest();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**AuthTestResponse**

### Authorization

[api_auth_token](../README.md#api_auth_token)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Bad Request |  -  |
|**402** | Payment Required |  -  |
|**403** | Forbidden |  -  |
|**404** | Not Found |  -  |
|**422** | Unprocessable Entity |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **authVerify**
> authVerify()


### Example

```typescript
import {
    AuthenticationApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let verificationId: string; // (default to undefined)

const { status, data } = await apiInstance.authVerify(
    verificationId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **verificationId** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Bad Request |  -  |
|**402** | Payment Required |  -  |
|**403** | Forbidden |  -  |
|**404** | Not Found |  -  |
|**422** | Unprocessable Entity |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **authVerifyTfa**
> SignInResponse authVerifyTfa(verifyTFARequest)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    VerifyTFARequest
} from './api';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let verifyTFARequest: VerifyTFARequest; //

const { status, data } = await apiInstance.authVerifyTfa(
    verifyTFARequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **verifyTFARequest** | **VerifyTFARequest**|  | |


### Return type

**SignInResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Bad Request |  -  |
|**402** | Payment Required |  -  |
|**403** | Forbidden |  -  |
|**404** | Not Found |  -  |
|**422** | Unprocessable Entity |  -  |
|**500** | Internal Server Error |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

