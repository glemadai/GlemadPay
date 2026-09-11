use std::fmt;

pub struct GlemadPayClient {
    http: reqwest::Client,
    base_url: String,
    api_key: String,
}

impl fmt::Debug for GlemadPayClient {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("GlemadPayClient")
            .field("base_url", &self.base_url)
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}

impl GlemadPayClient {
    pub fn new(base_url: impl Into<String>, api_key: impl Into<String>) -> anyhow::Result<Self> {
        let base_url = base_url.into().trim_end_matches('/').to_string();
        let parsed = reqwest::Url::parse(&base_url)?;
        anyhow::ensure!(
            parsed.scheme() == "https"
                && parsed.username().is_empty()
                && parsed.password().is_none()
                && matches!(parsed.path(), "" | "/")
                && parsed.query().is_none()
                && parsed.fragment().is_none(),
            "base_url must be an HTTPS origin"
        );
        let api_key = api_key.into();
        anyhow::ensure!(!api_key.trim().is_empty(), "api_key is required");
        let http = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(25))
            .redirect(reqwest::redirect::Policy::none())
            .build()?;
        Ok(Self {
            http,
            base_url,
            api_key,
        })
    }

    async fn api_request<T: serde::de::DeserializeOwned>(
        &self,
        request: reqwest::RequestBuilder,
    ) -> anyhow::Result<T> {
        let response = request
            .bearer_auth(&self.api_key)
            .header("Glemad-SDK", "rust/1.0.0")
            .send()
            .await?;
        let status = response.status();
        let request_id = response
            .headers()
            .get("x-request-id")
            .and_then(|value| value.to_str().ok())
            .map(str::to_string);
        let body: serde_json::Value = response.json().await?;
        if !status.is_success() {
            return Err(ApiError {
                status: status.as_u16(),
                code: body
                    .get("error")
                    .and_then(|value| value.as_str())
                    .unwrap_or("unknown_error")
                    .to_string(),
                request_id,
                body,
            }
            .into());
        }
        Ok(serde_json::from_value(body)?)
    }
}

#[derive(Debug)]
pub struct ApiError {
    pub status: u16,
    pub code: String,
    pub request_id: Option<String>,
    pub body: serde_json::Value,
}

impl fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "Glemad Pay {}: {}", self.status, self.code)
    }
}

impl std::error::Error for ApiError {}

fn encode_segment(value: &str) -> anyhow::Result<String> {
    anyhow::ensure!(
        !value.is_empty() && value != "." && value != "..",
        "invalid path parameter"
    );
    Ok(urlencoding::encode(value).into_owned())
}

pub fn verify_webhook(
    raw: &[u8],
    signature: &str,
    timestamp: &str,
    secret: &str,
    now: u64,
) -> bool {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    if secret.is_empty()
        || timestamp.is_empty()
        || timestamp.len() > 16
        || !timestamp.bytes().all(|byte| byte.is_ascii_digit())
        || (timestamp.starts_with('0') && timestamp.len() > 1)
    {
        return false;
    }
    let Ok(stamp) = timestamp.parse::<u64>() else {
        return false;
    };
    if now.abs_diff(stamp) > 300 {
        return false;
    }
    let signatures: Vec<_> = signature.split(',').collect();
    if signatures.is_empty()
        || signatures.len() > 2
        || signatures.iter().any(|value| {
            value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit())
        })
    {
        return false;
    }
    let mut valid = false;
    for value in signatures {
        let Ok(bytes) = hex::decode(value) else {
            return false;
        };
        let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(secret.as_bytes()) else {
            return false;
        };
        mac.update(raw);
        valid |= mac.verify_slice(&bytes).is_ok();
    }
    if !valid {
        return false;
    }
    serde_json::from_slice::<serde_json::Value>(raw)
        .ok()
        .and_then(|value| value.get("timestamp").and_then(|stamp| stamp.as_u64()))
        == Some(stamp)
}

pub mod types;
include!("operations.rs");
