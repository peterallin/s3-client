use anyhow::Result;
use auth::Auth;
use reqwest::header::HeaderMap;
use std::str::FromStr;
use url::Url;

mod auth;

fn main() -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let access_key = "minioadmin";
    let secret_key = "minioadmin";
    let date = chrono::Utc::now();
    let url = Url::parse("http://localhost:9000/testbucket/foo.txt")?;
    let auth = Auth::new_get(access_key, secret_key, date, &url)?;
    let body = client
        .get(url)
        .headers(reqwest_headers(&auth))
        .send()?
        .text()?;
    println!("{}", body);

    Ok(())
}

fn reqwest_headers(auth: &Auth) -> HeaderMap {
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
