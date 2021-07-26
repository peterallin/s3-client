mod auth;
pub mod reqwest_helpers;
pub use auth::Auth;

#[cfg(test)]
mod tests {
    use crate::Auth;
    use chrono::{DateTime, Utc};
    use url::Url;

    #[test]
    fn get_example_from_aws_doc() {
        let (access_key, secret_key) = aws_example_keys();
        let url = Url::parse("https://examplebucket.s3.amazonaws.com/test.txt").unwrap();
        let auth = Auth::new_get(
            access_key,
            secret_key,
            aws_example_date(),
            &url,
            &[("Range".into(), "bytes=0-9".into())],
        )
        .unwrap();
        let authorization = get_auth_header(&auth);

        assert_eq!("AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request, SignedHeaders=host;range;x-amz-content-sha256;x-amz-date, Signature=f0e8bdb87c964420e857bd35b5d6ed310bd44f0170aba48dd91039c6036bdb41", authorization);
    }

    #[test]
    fn put_example_from_aws_doc() {
        let (access_key, secret_key) = aws_example_keys();
        let url = Url::parse("https://examplebucket.s3.amazonaws.com/test%24file.text").unwrap();
        let auth = Auth::new_put(
            access_key,
            secret_key,
            aws_example_date(),
            &url,
            &[
                ("Date".into(), "Fri, 24 May 2013 00:00:00 GMT".into()),
                ("x-amz-storage-class".into(), "REDUCED_REDUNDANCY".into()),
            ],
            "Welcome to Amazon S3.".as_bytes(),
        )
        .unwrap();
        assert_eq!("AWS4-HMAC-SHA256 Credential=AKIAIOSFODNN7EXAMPLE/20130524/us-east-1/s3/aws4_request, SignedHeaders=date;host;x-amz-content-sha256;x-amz-date;x-amz-storage-class, Signature=98ad721746da40c64f1a55b78f14c238d841ea1380cd77a1b5971af0ece108bd", get_auth_header(&auth))
    }

    fn get_auth_header(auth: &Auth) -> &str {
        auth.headers()
            .iter()
            .filter(|(k, _)| k == "Authorization")
            .map(|(_, v)| v)
            .next()
            .unwrap()
    }

    fn aws_example_keys() -> (&'static str, &'static str) {
        (
            "AKIAIOSFODNN7EXAMPLE",
            "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
        )
    }

    fn aws_example_date() -> DateTime<Utc> {
        chrono::DateTime::parse_from_rfc3339("2013-05-24T00:00:00-00:00")
            .unwrap()
            .with_timezone(&Utc)
    }
}
