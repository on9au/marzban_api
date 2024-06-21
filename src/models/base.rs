use super::{proxy::ProxyHostSecurity, user::UserDataLimitResetStrategy};

pub(crate) fn default_usage_coefficient() -> f32 {
    1.0
}

pub(crate) fn default_port() -> u32 {
    62050
}

pub(crate) fn default_api_port() -> u32 {
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

pub(crate) fn default_with_0() -> u32 {
    0
}

pub(crate) fn default_data_limit_reset_strategy() -> UserDataLimitResetStrategy {
    UserDataLimitResetStrategy::NoReset
}

pub(crate) fn default_empty_string() -> String {
    "".to_string()
}
