//! Auth service user
//!

use chrono::{DateTime, Utc};
use chrono_tz::Tz;
pub use openidconnect::StandardClaims;
use openidconnect::{core::CoreGenderClaim, GenderClaim};
use phonenumber::PhoneNumber;
use sea_orm::FromJsonQueryResult;
use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use url::Url;

pub type StandardUserProfile = StandardClaims<CoreGenderClaim>;

/// 符合 Standard Claims 的用户档案结构体
///
/// 不同于 [StandardUserProfile]，这个是当前库定义的版本，更符合业务开发维护的需要。
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromJsonQueryResult)]
pub struct UserProfile {
    /// Subject - Identifier for the End-User at the Issuer.
    pub sub: String,

    /// End-User's full name in displayable form including all name parts,
    /// possibly including titles and suffixes,
    /// ordered according to the End-User's locale and preferences.
    pub name: String,

    /// Given name(s) or first name(s) of the End-User.
    /// Note that in some cultures, people can have multiple given names;
    /// all can be present, with the names being separated by space characters.
    pub given_name: Option<String>,

    /// Surname(s) or last name(s) of the End-User. Note that in some cultures,
    /// people can have multiple family names or no family name; all can be present,
    /// with the names being separated by space characters.
    pub family_name: Option<String>,

    /// Middle name(s) of the End-User. Note that in some cultures,
    /// people can have multiple middle names; all can be present,
    /// with the names being separated by space characters.
    /// Also note that in some cultures, middle names are not used.
    pub middle_name: Option<String>,

    /// Casual name of the End-User that may or may not be the same as the `given_name`.
    /// For instance, a `nickname` value of `Mike` might be returned alongside a `given_name` value of `Michael`.
    pub nickname: Option<String>,

    /// Shorthand name by which the End-User wishes to be referred to at the RP,
    /// such as `janedoe` or `j.doe`. This value MAY be any valid JSON string
    /// including special characters such as `@`, `/`, or whitespace.
    /// The RP MUST NOT rely upon this value being unique,
    /// as discussed in [Section 5.7](https://openid.net/specs/openid-connect-core-1_0.html#ClaimStability).
    pub preferred_username: Option<String>,

    /// URL of the End-User's profile page. The contents of this Web page SHOULD be about the End-User.
    pub profile: Option<Url>,

    /// URL of the End-User's profile picture.
    /// This URL MUST refer to an image file (for example, a PNG, JPEG, or GIF image file),
    /// rather than to a Web page containing an image.
    /// Note that this URL SHOULD specifically reference a profile photo of the End-User suitable for
    /// displaying when describing the End-User, rather than an arbitrary photo taken by the End-User.
    pub picture: Option<Url>,

    /// URL of the End-User's Web page or blog. This Web page SHOULD contain information published
    /// by the End-User or an organization that the End-User is affiliated with.
    pub website: Option<Url>,

    /// End-User's preferred e-mail address. Its value MUST conform
    /// to the [RFC 5322](https://openid.net/specs/openid-connect-core-1_0.html#RFC5322) [RFC5322] addr-spec syntax.
    /// The RP MUST NOT rely upon this value being unique, as discussed in
    /// [Section 5.7](https://openid.net/specs/openid-connect-core-1_0.html#ClaimStability).
    pub email: Option<String>,

    /// True if the End-User's e-mail address has been verified; otherwise false.
    /// When this Claim Value is `true`, this means that the OP took affirmative steps
    /// to ensure that this e-mail address was controlled
    /// by the End-User at the time the verification was performed.
    /// The means by which an e-mail address is verified is context specific,
    /// and dependent upon the trust framework or contractual agreements within which the parties are operating.
    pub email_verified: Option<bool>,

    /// End-User's gender. Values defined by this specification are `female` and `male`.
    /// Other values MAY be used when neither of the defined values are applicable.
    pub gender: Option<Gender>,

    /// End-User's birthday, represented as an
    /// [ISO 8601-1](https://openid.net/specs/openid-connect-core-1_0.html#ISO8601-1) [ISO8601‑1] `YYYY-MM-DD` format.
    /// The year MAY be `0000`, indicating that it is omitted.
    /// To represent only the year, `YYYY` format is allowed.
    /// Note that depending on the underlying platform's date related function,
    /// providing just year can result in varying month and day,
    /// so the implementers need to take this factor into account to correctly process the dates.
    pub birthdate: Option<String>,

    /// String from IANA Time Zone Database
    /// [IANA.time‑zones](https://openid.net/specs/openid-connect-core-1_0.html#IANA.time-zones)
    /// representing the End-User's time zone.
    /// For example, `Europe/Paris` or `America/Los_Angeles`.
    pub zoneinfo: Option<Tz>,

    /// End-User's locale, represented as a [BCP47](https://openid.net/specs/openid-connect-core-1_0.html#RFC5646) [RFC5646] language tag.
    /// This is typically an [ISO 639 Alpha-2](https://openid.net/specs/openid-connect-core-1_0.html#ISO639) [ISO639] language code
    /// in lowercase and an [ISO 3166-1 Alpha-2](https://openid.net/specs/openid-connect-core-1_0.html#ISO3166-1) [ISO3166‑1]
    /// country code in uppercase, separated by a dash.
    /// For example, `en-US` or `fr-CA`. As a compatibility note,
    /// some implementations have used an underscore as the separator rather than a dash,
    /// for example, `en_US`; Relying Parties MAY choose to accept this locale syntax as well.
    pub locale: Option<String>,

    /// End-User's preferred telephone number. [E.164](https://openid.net/specs/openid-connect-core-1_0.html#E.164) [E.164]
    /// is RECOMMENDED as the format of this Claim,
    /// for example, `+1 (425) 555-1212` or `+56 (2) 687 2400`.
    /// If the phone number contains an extension,
    /// it is RECOMMENDED that the extension be represented using the [RFC 3966](https://openid.net/specs/openid-connect-core-1_0.html#RFC3966) [RFC3966] extension syntax,
    /// for example, `+1 (604) 555-1234;ext=5678`.
    pub phone_number: Option<PhoneNumber>,

    /// True if the End-User's phone number has been verified;
    /// otherwise false. When this Claim Value is `true`,
    /// this means that the OP took affirmative steps
    /// to ensure that this phone number was controlled
    /// by the End-User at the time the verification was performed.
    /// The means by which a phone number is verified is context specific,
    /// and dependent upon the trust framework or contractual agreements within which the parties are operating.
    /// When true, the `phone_number` Claim MUST be in E.164 format and any extensions MUST be represented in RFC 3966 format.
    pub phone_number_verified: Option<bool>,

    /// End-User's preferred postal address.
    /// The value of the address member is a [JSON](https://openid.net/specs/openid-connect-core-1_0.html#RFC8259) [RFC8259]
    /// structure containing some
    /// or all of the members defined in [Section 5.1.1](https://openid.net/specs/openid-connect-core-1_0.html#AddressClaim).
    pub address: Option<AddressClaim>,

    /// Time the End-User's information was last updated.
    /// Its value is a JSON number representing the number of seconds from 1970-01-01T00:00:00Z
    /// as measured in UTC until the date/time.
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(
    Debug, Clone, Deserialize_enum_str, Serialize_enum_str, PartialEq, Eq, FromJsonQueryResult,
)]
pub enum Gender {
    Male,
    Female,
    #[serde(other)]
    Other(String),
}

impl GenderClaim for Gender {}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, FromJsonQueryResult)]
pub struct AddressClaim {
    /// Full mailing address, formatted for display or use on a mailing label.
    ///
    /// This field MAY contain multiple lines, separated by newlines. Newlines can be represented
    /// either as a carriage return/line feed pair (`\r\n`) or as a single line feed character
    /// (`\n`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,

    /// Full street address component, which MAY include house number, street name, Post Office Box,
    /// and multi-line extended street address information.
    ///
    /// This field MAY contain multiple lines, separated by newlines. Newlines can be represented
    /// either as a carriage return/line feed pair (`\r\n`) or as a single line feed character
    /// (`\n`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,

    /// City or locality component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,

    /// State, province, prefecture, or region component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,

    /// Zip code or postal code component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// Country name component.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// User credential use for login
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "payload", rename_all = "snake_case")]
pub enum UserCredential {
    /// 使用用户名作为登录凭据
    Username {
        /// 用户名
        username: String,
        /// 密码
        password: String,
    },
    /// 使用邮箱作为登录凭据
    Email {
        /// 邮箱
        email: String,
        /// 密码
        password: String,
    },
}
