//! Multi-account resolution and identity extraction.

use serde_json::Value;

use crate::client::Client;
use crate::config::{self, AccountConfig, Config};
use crate::error::{Error, ErrorCode, Result};
use crate::secrets;

pub struct Resolved {
    pub name: String,
    #[allow(dead_code)]
    pub config: Option<AccountConfig>,
    pub token: String,
}

impl Resolved {
    pub fn client(&self) -> Client {
        Client::new(&self.token)
    }
}

/// Resolves an API token to use:
/// 1. Direct `--api-key` if provided.
/// 2. Explicit `--account <name>` if provided.
/// 3. `APIFY_TOKEN` or `REDDIT_API_KEY` environment variables.
/// 4. If exactly one account exists in config, that account is used.
/// 5. Otherwise, an informative error is returned.
pub fn resolve(requested_account: Option<&str>, direct_key: Option<&str>) -> Result<Resolved> {
    if let Some(key) = direct_key.map(str::trim).filter(|s| !s.is_empty()) {
        return Ok(Resolved { name: "direct".into(), config: None, token: key.to_string() });
    }

    let config = config::load()?;

    if let Some(requested) = requested_account.map(str::trim).filter(|s| !s.is_empty()) {
        let Some((name, account)) = config.find(requested) else {
            return Err(Error::new(ErrorCode::NoAccount, format!("No account named '{requested}'."))
                .detail(describe(&config))
                .fix("reddit accounts list"));
        };

        let key = secrets::store()?.get(&secrets::account_key(name))?;
        let Some(api_key) = key.filter(|k| !k.trim().is_empty()) else {
            return Err(Error::new(ErrorCode::AuthRequired, format!("Account '{name}' has no stored API key."))
                .detail("The config entry exists but the keystore has nothing under it.")
                .fix(format!("reddit accounts add {name} --api-key <key> --force")));
        };

        return Ok(Resolved { name: name.clone(), config: Some(account.clone()), token: api_key });
    }

    if let Some(env_token) = std::env::var("APIFY_TOKEN")
        .or_else(|_| std::env::var("REDDIT_API_KEY"))
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
    {
        return Ok(Resolved { name: "environment".into(), config: None, token: env_token });
    }

    if config.accounts.len() == 1 {
        let (name, account) = config.accounts.iter().next().unwrap();
        let key = secrets::store()?.get(&secrets::account_key(name))?;
        if let Some(api_key) = key.filter(|k| !k.trim().is_empty()) {
            return Ok(Resolved { name: name.clone(), config: Some(account.clone()), token: api_key });
        }
    }

    if config.accounts.is_empty() {
        return Err(Error::new(
            ErrorCode::AuthRequired,
            "No API token provided. Use --api-key, set APIFY_TOKEN, or run: reddit login",
        )
        .fix("reddit login default"));
    }

    Err(Error::new(
        ErrorCode::NoAccount,
        "Multiple accounts configured. Specify which account to use with --account <name>.",
    )
    .detail(describe(&config))
    .fix("reddit accounts list"))
}

pub fn describe(config: &Config) -> String {
    if config.accounts.is_empty() {
        return "No accounts are configured yet. Run 'reddit accounts add <name> --api-key <key>'.".into();
    }
    let listed: Vec<String> = config
        .sorted()
        .into_iter()
        .map(|(k, v)| if v.identity.trim().is_empty() { k.clone() } else { format!("{k} ({})", v.identity) })
        .collect();
    format!("Configured accounts: {}", listed.join(", "))
}

pub mod identity {
    use super::Value;

    pub fn describe(me: &Value) -> String {
        if !me.is_object() {
            return String::new();
        }

        // Apify returns {"data": {"username": "...", "email": "...", "id": "..."}}
        if let Some(data) = me.get("data").filter(|d| d.is_object()) {
            if let Some(username) = data.get("username").and_then(Value::as_str).filter(|s| !s.is_empty()) {
                return username.to_string();
            }
            if let Some(email) = data.get("email").and_then(Value::as_str).filter(|s| !s.is_empty()) {
                return email.to_string();
            }
            if let Some(id) = data.get("id").and_then(Value::as_str).filter(|s| !s.is_empty()) {
                return id.to_string();
            }
        }

        for key in &["username", "user", "email", "id", "name"] {
            if let Some(s) = me.get(*key).and_then(Value::as_str).filter(|s| !s.is_empty()) {
                return s.to_string();
            }
        }

        String::new()
    }
}
