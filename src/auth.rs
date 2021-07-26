use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use reqwest::Url;

pub struct Auth {
    headers: Vec<(String, String)>,
}

impl Auth {
    pub fn new_get(
        access_key: &str,
        secret_key: &str,
        date: DateTime<Utc>,
        url: &Url,
        extra_headers: &[(String, String)],
    ) -> Result<Self> {
        make_auth(
            "GET",
            access_key,
            secret_key,
            &date,
            url,
            extra_headers,
            &[],
        )
    }

    pub fn new_put(
        access_key: &str,
        secret_key: &str,
        date: DateTime<Utc>,
        url: &Url,
        extra_headers: &[(String, String)],
        payload: &[u8],
    ) -> Result<Self> {
        make_auth(
            "PUT",
            access_key,
            secret_key,
            &date,
            url,
            extra_headers,
            payload,
        )
    }

    pub fn headers(&self) -> &[(String, String)] {
        &self.headers
    }
}

fn make_auth(
    http_method: &str,
    access_key: &str,
    secret_key: &str,
    date: &DateTime<Utc>,
    url: &Url,
    extra_headers: &[(String, String)],
    payload: &[u8],
) -> Result<Auth> {
    let region = "us-east-1";
    let payload_hash = hash(payload);
    let host = url
        .host_str()
        .ok_or_else(|| anyhow!("No host in the URL"))?;
    let host_port = if let Some(port) = url.port() {
        format!("{}:{}", host, port)
    } else {
        host.to_string()
    };
    let date_zulu = date.format("%Y%m%dT%H%M%SZ").to_string();
    let mut headers: Vec<(String, String)> = vec![
        ("Host".into(), host_port),
        ("X-Amz-Content-sha256".into(), payload_hash.clone()),
        ("X-Amz-Date".into(), date_zulu),
    ];
    headers.extend_from_slice(extra_headers);
    let canonical_headers = make_canonical_headers(&headers);
    let signed_headers = make_signed_headers(&headers);
    let canonical_request = make_canonical_request(
        http_method,
        url.path(),
        "",
        &canonical_headers,
        &signed_headers,
        &payload_hash,
    );
    println!("{}", canonical_request);
    let to_sign = make_string_to_sign(&date, region, &hash(canonical_request.as_bytes()));
    let signing_key = signing_key(secret_key, &date.format("%Y%m%d").to_string(), region);
    let signature = to_hex_string(&hmac_sha256::HMAC::mac(to_sign.as_bytes(), &signing_key));
    let credential = format!(
        "{}/{}/{}/s3/aws4_request",
        access_key,
        date.format("%Y%m%d"),
        region
    );
    let authorization = format!(
        "AWS4-HMAC-SHA256 Credential={}, SignedHeaders={}, Signature={}",
        credential, signed_headers, signature
    );
    let authorization = ("Authorization".into(), authorization);

    let headers: Vec<(String, String)> = headers
        .into_iter()
        .chain(std::iter::once(authorization))
        .collect();
    Ok(Auth { headers })
}

fn hash(data: &[u8]) -> String {
    to_hex_string(&hmac_sha256::Hash::hash(data))
}

fn make_canonical_headers(headers: &[(String, String)]) -> Vec<String> {
    let mut header_strings: Vec<String> = headers
        .iter()
        .map(|(k, v)| format!("{}:{}", k.to_ascii_lowercase(), v))
        .collect();
    header_strings.sort();
    header_strings
}

fn make_signed_headers(headers: &[(String, String)]) -> String {
    let mut header_names: Vec<String> = headers
        .iter()
        .map(|(k, _)| k.to_string().to_ascii_lowercase())
        .collect();
    header_names.sort();
    header_names.join(";")
}

fn make_string_to_sign(date: &DateTime<Utc>, region: &str, canonical_request_hash: &str) -> String {
    let scope = format!("{}/{}/s3/aws4_request", date.format("%Y%m%d"), region);
    format!(
        "{}\n{}\n{}\n{}",
        "AWS4-HMAC-SHA256",
        date.format("%Y%m%dT%H%M%SZ"),
        scope,
        canonical_request_hash
    )
}

fn signing_key(secret_key: &str, date_compact: &str, region: &str) -> [u8; 32] {
    let date_key_string = format!("AWS4{}", secret_key);
    let date_key = hmac_sha256::HMAC::mac(date_compact.as_bytes(), date_key_string.as_bytes());
    let date_region_key = hmac_sha256::HMAC::mac(region.as_bytes(), &date_key);
    let date_region_service_key = hmac_sha256::HMAC::mac("s3".as_bytes(), &date_region_key);
    hmac_sha256::HMAC::mac("aws4_request".as_bytes(), &date_region_service_key)
}

fn to_hex_string(x: &[u8]) -> String {
    x.iter().map(|x| format!("{:02x}", x)).collect()
}

fn make_canonical_request<T: AsRef<str>>(
    method: &str,
    uri: &str,
    query: &str,
    headers: &[T],
    signed_headers: &str,
    payload_hash: &str,
) -> String {
    let headers: String = headers
        .iter()
        .map(|x| format!("{}\n", x.as_ref()))
        .collect();
    format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        method, uri, query, headers, signed_headers, payload_hash
    )
}
