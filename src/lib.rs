mod auth;
pub mod reqwest_helpers;
pub use auth::Auth;

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use url::Url;

    #[test]
    fn get_example_from_aws_doc() {
        let access_key = "AKIAIOSFODNN7EXAMPLE";
        let secret_key = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
        let url = Url::parse("https://examplebucket.s3.amazonaws.com/test.txt").unwrap();
        let date = chrono::DateTime::parse_from_rfc3339("2013-05-24T00:00:00-00:00")
            .unwrap()
            .with_timezone(&Utc);
        let auth = crate::Auth::new_get(
            access_key,
            secret_key,
            date,
            &url,
            &[("Range".into(), "bytes=0-9".into())],
        )
        .unwrap();
        let authorization = auth
            .headers()
            .iter()
            .filter(|(k, _)| k == "Authorization")
            .map(|(_, v)| v)
            .next()
            .unwrap();

        assert_eq!("AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request, SignedHeaders=host;range;x-amz-content-sha256;x-amz-date, Signature=f0e8bdb87c964420e857bd35b5d6ed310bd44f0170aba48dd91039c6036bdb41", authorization);
    }
}
