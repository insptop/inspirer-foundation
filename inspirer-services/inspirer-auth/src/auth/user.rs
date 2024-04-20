//! Auth service user
//!

use openidconnect::core::CoreGenderClaim;
pub use openidconnect::StandardClaims;

pub type StandardUserProfile = StandardClaims<CoreGenderClaim>;
