use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Deserializer};

use super::{proxy::ProxyHostSecurity, user::UserDataLimitResetStrategy};

pub(crate) fn default_usage_coefficient() -> f64 {
    1.0
}

pub(crate) fn default_port() -> u16 {
    62050
}

pub(crate) fn default_api_port() -> u16 {
    62051
}

pub(crate) fn default_true() -> bool {
    true
}

pub(crate) fn default_min_node_version() -> String {
    "v0.2.0".to_string()
}

pub(crate) fn default_proxy_host_security() -> ProxyHostSecurity {
    ProxyHostSecurity::InboundDefault
}

pub(crate) fn default_with_0() -> u64 {
    0
}

pub(crate) fn default_data_limit_reset_strategy() -> UserDataLimitResetStrategy {
    UserDataLimitResetStrategy::NoReset
}

pub(crate) fn default_empty_string() -> String {
    "".to_string()
}

pub(crate) fn parse_datetime<'de, D>(
    deserializer: D,
) -> Result<chrono::DateTime<chrono::Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use chrono::{DateTime, NaiveDateTime, Utc};
    let s: &str = serde::Deserialize::deserialize(deserializer)?;
    let naive = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f")
        .map_err(serde::de::Error::custom)?;
    Ok(DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc))
}

pub(crate) fn parse_some_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt: Option<&str> = Deserialize::deserialize(deserializer)?;
    if let Some(s) = opt {
        if s.is_empty() {
            return Ok(None);
        }
        let parse_from_str = NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S%.f");
        let naive_datetime = parse_from_str.map_err(serde::de::Error::custom)?;
        Ok(Some(DateTime::<Utc>::from_naive_utc_and_offset(
            naive_datetime,
            Utc,
        )))
    } else {
        Ok(None)
    }
}
