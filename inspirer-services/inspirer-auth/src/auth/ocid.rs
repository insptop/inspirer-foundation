//! OpenId Connect 认证
//!
//! OpenID Connect 1.0 是 OAuth 2.0 协议之上的一个简单身份层。
//! 它使客户端能够根据授权服务器执行的身份验证验证终端用户的身份，
//! 并以可互操作和类似 REST 的方式获取终端用户的基本配置文件信息。
//!
//! 基于规范（[官方链接](https://openid.net/specs/openid-connect-core-1_0.html)）
//! 定义该模块
//!

use openidconnect::core::{CoreAuthPrompt, CoreResponseMode, CoreResponseType};
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

/// Authentication Request
///
/// 相关结构标准的定义可查阅
/// [OpenId Connect Core 3.1.2.1. Authentication Request](https://openid.net/specs/openid-connect-core-1_0.html#AuthRequest)
///
/// > Authorization Servers MUST support the use of the HTTP GET and POST methods
/// > defined in RFC 7231 [RFC7231] at the Authorization Endpoint.
/// > Clients MAY use the HTTP GET or POST methods
/// > to send the Authorization Request to the Authorization Server.
/// > If using the HTTP GET method, the request parameters are serialized using URI Query String Serialization,
/// > per Section 13.1. If using the HTTP POST method,
/// > the request parameters are serialized using Form Serialization, per Section 13.2.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthenticationRequest {
    /// REQUIRED. OpenID Connect requests MUST contain the openid scope value.
    /// If the openid scope value is not present, the behavior is entirely unspecified.
    /// Other scope values MAY be present. Scope values used that are not understood
    /// by an implementation SHOULD be ignored.
    /// See Sections [5.4](https://openid.net/specs/openid-connect-core-1_0.html#ScopeClaims)
    /// and [11](https://openid.net/specs/openid-connect-core-1_0.html#OfflineAccess)
    /// for additional scope values defined by this specification.
    pub scope: String,

    /// REQUIRED. OAuth 2.0 Response Type value that determines the authorization processing flow to be used,
    /// including what parameters are returned from the endpoints used.
    /// When using the Authorization Code Flow, this value is code.
    pub response_type: CoreResponseType,

    /// REQUIRED. OAuth 2.0 Client Identifier valid at the Authorization Server.
    ///
    /// 在该系统中基本是以 Uuid 存在的，但在这个结构内为保障后续兼容设置为 [String] 类型
    pub client_id: String,

    /// REQUIRED. Redirection URI to which the response will be sent.
    /// This URI MUST exactly match one of the Redirection URI values
    /// for the Client pre-registered at the OpenID Provider,
    /// with the matching performed as described in Section 6.2.1 of
    /// [[RFC3986]](https://openid.net/specs/openid-connect-core-1_0.html#RFC3986) (Simple String Comparison).
    /// When using this flow, the Redirection URI SHOULD use the https scheme; however,
    /// it MAY use the http scheme, provided that the Client Type is confidential,
    /// as defined in Section 2.1 of OAuth 2.0,
    /// and provided the OP allows the use of http Redirection URIs in this case. Also,
    /// if the Client is a native application, it MAY use the http scheme with localhost
    /// or the IP loopback literals 127.0.0.1 or [::1] as the hostname.
    /// The Redirection URI MAY use an alternate scheme,
    /// such as one that is intended to identify a callback into a native application.
    pub redirect_uri: Url,

    /// RECOMMENDED. Opaque value used to maintain state between the request and the callback.
    /// Typically, Cross-Site Request Forgery (CSRF, XSRF) mitigation is done
    /// by cryptographically binding the value of this parameter with a browser cookie.
    pub state: Option<String>,

    /// OPTIONAL. Informs the Authorization Server of the mechanism
    /// to be used for returning parameters from the Authorization Endpoint.
    /// This use of this parameter is NOT RECOMMENDED when the Response Mode
    /// that would be requested is the default mode specified for the Response Type.
    pub response_mode: Option<CoreResponseMode>,

    /// OPTIONAL. String value used to associate a Client session with an ID Token,
    /// and to mitigate replay attacks. The value is passed through unmodified
    /// from the Authentication Request to the ID Token.
    /// Sufficient entropy MUST be present in the nonce values
    /// used to prevent attackers from guessing values.
    /// For implementation notes, see [Section 15.5.2](https://openid.net/specs/openid-connect-core-1_0.html#NonceNotes).
    pub nonce: Option<String>,

    /// OPTIONAL. Space-delimited, case-sensitive list of ASCII string values
    /// that specifies whether the Authorization Server prompts the End-User
    /// for reauthentication and consent.
    ///
    /// The defined values: [openidconnect::core::CoreAuthPrompt]
    pub prompt: Option<CoreAuthPrompt>,
}
