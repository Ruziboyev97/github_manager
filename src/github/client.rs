use reqwest::{header, Client};
use std::error::Error;

#[derive(Clone)]
pub struct GitHubClient {
    client: Client,
    username: String,
}

impl GitHubClient {
    pub fn new(token: &str, username: &str) -> Result<Self, Box<dyn Error>> {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", token))?,
        );

        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static("rust-github-manager"),
        );

        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/vnd.github.v3+json"),
        );

        // build the http client with headers

        let client = Client::builder().default_headers(headers).build()?;

        Ok(GitHubClient {
            client,
            username: username.to_string(),
        })
    }

    pub fn get_client(&self) -> &Client {
        &self.client
    }

    pub fn get_username(&self) -> &str {
        &self.username
    }
}
