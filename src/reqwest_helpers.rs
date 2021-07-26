use crate::Auth;
use reqwest::header::HeaderMap;
use std::str::FromStr;

pub fn reqwest_headers(auth: &Auth) -> HeaderMap {
    auth.headers()
        .iter()
        .map(|(k, v)| {
            (
                reqwest::header::HeaderName::from_str(k).unwrap(),
                reqwest::header::HeaderValue::from_str(v).unwrap(),
            )
        })
        .collect()
}
