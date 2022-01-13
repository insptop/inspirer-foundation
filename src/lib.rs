#[macro_use]
extern crate async_trait;

pub mod error;
pub mod service;
pub mod component;

#[cfg(feature = "enable-axum")]
pub mod axum;

pub use error::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
