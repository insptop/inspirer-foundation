use inspirer_framework::{response::{ok, Resp}, Error};

pub mod auth;

pub async fn test() -> Resp<&'static str> {
    ok("hello world")
}

pub async fn test_err() -> Resp<&'static str> {
    Err(Error::string("error"))
}