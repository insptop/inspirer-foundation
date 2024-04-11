use headers::{Error, Header, HeaderName, HeaderValue};
use uuid::Uuid;

pub static APP_ID_HEADER_NAME: HeaderName = HeaderName::from_static("x-auth-app-id");

pub struct AppId(pub Uuid);

impl Header for AppId {
    fn name() -> &'static HeaderName {
        &APP_ID_HEADER_NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        values
            .next()
            .and_then(|v| Uuid::parse_str(v.to_str().ok()?).ok())
            .map(AppId)
            .ok_or_else(Error::invalid)
    }

    fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
        let mut buf = Uuid::encode_buffer();
        let uuid = self
            .0
            .as_hyphenated()
            .encode_lower(&mut buf);

        if let Ok(value) = HeaderValue::from_str(uuid) {
            values.extend(std::iter::once(value));
        }
    }
}
