use anyhow::Result;
use s3_client::reqwest_helpers::reqwest_headers;
use s3_client::Auth;
use url::Url;

fn main() -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let access_key = "minioadmin";
    let secret_key = "minioadmin";
    let date = chrono::Utc::now();
    let url = Url::parse("http://localhost:9000/testbucket/open-png.mkv")?;
    let auth = Auth::new_get(access_key, secret_key, date, &url, &[])?;
    let response = client.get(url).headers(reqwest_headers(&auth)).send()?;
    if response.status() == reqwest::StatusCode::OK {
        let contents = response.bytes()?;
        std::fs::write("output", contents)?;
    } else {
        println!("Got status: {}", response.status());
        println!("{}", response.text()?);
    }

    Ok(())
}
