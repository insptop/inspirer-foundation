use std::fmt;

use base64::prelude::*;
use serde::Serialize;

/// Base64 standard encoding
///
/// # Example
///
/// ```
/// use inspirer_services::inspirer_auth::helper::base64_encode;
///
/// let data = b"hello world";
///
/// let encoded = base64_encode(data);
///
/// assert_eq!(encoded, "aGVsbG8gd29ybGQ=");
///
/// println!("encoded: {}", encoded);
/// ```
pub fn base64_encode(data: &[u8]) -> String {
    BASE64_STANDARD.encode(data)
}

pub fn display_option<T: fmt::Display>(o: &Option<T>) -> String {
    match o {
        Some(v) => format!("{}", v),
        None => "None".to_string(),
    }
}
