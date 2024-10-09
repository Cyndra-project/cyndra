use headers::{Header, HeaderName};
use http::HeaderValue;

pub static X_cyndra_ADMIN_SECRET: HeaderName = HeaderName::from_static("x-cyndra-admin-secret");

/// Typed header for sending admin secrets to deployers
pub struct XCyndraAdminSecret(pub String);

impl Header for XCyndraAdminSecret {
    fn name() -> &'static HeaderName {
        &X_cyndra_ADMIN_SECRET
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        let value = values
            .next()
            .ok_or_else(headers::Error::invalid)?
            .to_str()
            .map_err(|_| headers::Error::invalid())?
            .to_string();

        Ok(Self(value))
    }

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        if let Ok(value) = HeaderValue::from_str(&self.0) {
            values.extend(std::iter::once(value));
        }
    }
}

pub static X_cyndra_ACCOUNT_NAME: HeaderName = HeaderName::from_static("x-cyndra-account-name");

/// Typed header for sending account names around
#[derive(Default)]
pub struct XCyndraAccountName(pub String);

impl Header for XCyndraAccountName {
    fn name() -> &'static HeaderName {
        &X_cyndra_ACCOUNT_NAME
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i http::HeaderValue>,
    {
        let value = values
            .next()
            .ok_or_else(headers::Error::invalid)?
            .to_str()
            .map_err(|_| headers::Error::invalid())?
            .to_string();

        Ok(Self(value))
    }

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        if let Ok(value) = HeaderValue::from_str(&self.0) {
            values.extend(std::iter::once(value));
        }
    }
}

pub static X_cyndra_PROJECT: HeaderName = HeaderName::from_static("x-cyndra-project");

pub struct XCyndraProject(pub String);

impl Header for XCyndraProject {
    fn name() -> &'static HeaderName {
        &X_cyndra_PROJECT
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i HeaderValue>,
    {
        let value = values
            .next()
            .ok_or_else(headers::Error::invalid)?
            .to_str()
            .map_err(|_| headers::Error::invalid())?
            .to_string();

        Ok(Self(value))
    }

    fn encode<E: Extend<http::HeaderValue>>(&self, values: &mut E) {
        if let Ok(value) = HeaderValue::from_str(self.0.as_str()) {
            values.extend(std::iter::once(value));
        }
    }
}
