//! HTTP client for the Apify API and status to [`ErrorCode`] mapping.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use ureq::Agent;
use ureq::http::Response;

use crate::error::{Error, ErrorCode, Result};

const DEFAULT_BASE: &str = "https://api.apify.com/v2/";
const MAX_BODY: u64 = 512 * 1024 * 1024;

#[derive(Clone)]
pub struct Client {
    agent: Agent,
    base: String,
    token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProxyConfig {
    pub use_apify_proxy: bool,
    pub apify_proxy_groups: Vec<String>,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self { use_apify_proxy: true, apify_proxy_groups: vec!["RESIDENTIAL".to_string()] }
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searches: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_community_name: Option<String>,
    pub search_posts: bool,
    pub search_comments: bool,
    pub search_communities: bool,
    pub search_users: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "includeNSFW")]
    pub include_nsfw: bool,
    pub max_items: u32,
    pub max_post_count: u32,
    pub max_comments: u32,
    pub skip_comments: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_date_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_date_limit: Option<String>,
    pub proxy: ProxyConfig,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StartUrl {
    pub url: String,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ScrapeInput {
    pub start_urls: Vec<StartUrl>,
    pub skip_comments: bool,
    pub skip_user_posts: bool,
    pub skip_community: bool,
    pub max_items: u32,
    pub max_post_count: u32,
    pub max_comments: u32,
    pub max_communities_count: u32,
    pub max_user_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_date_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_date_limit: Option<String>,
    pub proxy: ProxyConfig,
}

impl Client {
    pub fn new(token: &str) -> Client {
        let agent: Agent = Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(360)))
            .timeout_connect(Some(Duration::from_secs(15)))
            .http_status_as_error(false)
            .user_agent(concat!("reddit-cli/", env!("CARGO_PKG_VERSION")))
            .build()
            .into();

        let mut base = std::env::var("REDDIT_API_URL")
            .or_else(|_| std::env::var("APIFY_API_URL"))
            .ok()
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| DEFAULT_BASE.to_string());
        if !base.ends_with('/') {
            base.push('/');
        }

        Client { agent, base, token: token.trim().to_string() }
    }

    pub fn search(&self, input: &SearchInput) -> Result<Value> {
        let path = format!("acts/trudax~reddit-scraper-lite/run-sync-get-dataset-items?token={}", self.token);
        let body = serde_json::to_value(input).map_err(|e| Error::invalid(e.to_string()))?;
        self.post(&path, &body)
    }

    pub fn scrape(&self, input: &ScrapeInput) -> Result<Value> {
        let path = format!("acts/trudax~reddit-scraper-lite/run-sync-get-dataset-items?token={}", self.token);
        let body = serde_json::to_value(input).map_err(|e| Error::invalid(e.to_string()))?;
        self.post(&path, &body)
    }

    pub fn me(&self) -> Result<Value> {
        let path = format!("users/me?token={}", self.token);
        self.get(&path)
    }

    pub fn get(&self, path: &str) -> Result<Value> {
        let url = format!("{}{}", self.base, path);
        let req = self
            .agent
            .get(&url)
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("Accept", "application/json");

        let response = req.call().map_err(transport_error)?;
        read(response)
    }

    pub fn post(&self, path: &str, body: &Value) -> Result<Value> {
        let url = format!("{}{}", self.base, path);
        let json = serde_json::to_vec(body).expect("a Value always serializes");
        let req = self
            .agent
            .post(&url)
            .header("Authorization", &format!("Bearer {}", self.token))
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");

        let response = req.send(&json[..]).map_err(transport_error)?;
        read(response)
    }
}

fn read(mut response: Response<ureq::Body>) -> Result<Value> {
    let status = response.status().as_u16();
    let bytes = response.body_mut().with_config().limit(MAX_BODY).read_to_vec().map_err(transport_error)?;

    if !(200..300).contains(&status) {
        let body = String::from_utf8_lossy(&bytes).trim().to_string();
        return Err(status_error(status, &body));
    }

    if bytes.iter().all(u8::is_ascii_whitespace) {
        return Ok(Value::Object(Default::default()));
    }

    serde_json::from_slice(&bytes).or_else(|_| Ok(Value::String(String::from_utf8_lossy(&bytes).into_owned())))
}

fn transport_error(e: ureq::Error) -> Error {
    match e {
        ureq::Error::Timeout(_) => {
            Error::new(ErrorCode::Network, "The request timed out.").fix("Retry once, then stop.")
        }
        other => Error::new(ErrorCode::Network, "Could not reach the Apify API.")
            .detail(other.to_string())
            .fix("Retry once, then stop."),
    }
}

pub fn status_error(status: u16, body: &str) -> Error {
    let mut detail = format!("HTTP {status}");
    if !body.is_empty() {
        detail.push_str(": ");
        detail.push_str(body);
    }

    match status {
        401 => Error::new(ErrorCode::AuthRequired, "The API token was rejected or expired.")
            .detail(detail)
            .fix("Check your Apify token: reddit login <name> --api-key <key> --force"),
        403 => Error::new(ErrorCode::AuthRequired, "Access denied by Apify.")
            .detail(detail)
            .fix("Check your Apify account permissions and subscription."),
        404 => Error::new(ErrorCode::NotFound, "The resource was not found.").detail(detail),
        429 => {
            Error::new(ErrorCode::RateLimited, "Rate limited by Apify.").detail(detail).fix("Back off before retrying.")
        }
        400 | 422 => Error::new(ErrorCode::InvalidInput, "The API refused the request.").detail(detail),
        s if s >= 500 => Error::new(ErrorCode::Network, "Apify returned a server error.")
            .detail(detail)
            .fix("Retry; if it persists, check Apify status at https://status.apify.com"),
        _ => Error::new(ErrorCode::Error, "The request failed.").detail(detail),
    }
}
