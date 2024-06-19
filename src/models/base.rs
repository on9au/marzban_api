use super::{proxy::ProxyHostSecurity, user::UserDataLimitResetStrategy};

pub fn default_usage_coefficient() -> f32 {
    1.0
}

pub fn default_port() -> u32 {
    62050
}

pub fn default_api_port() -> u32 {
    62051
}

pub fn default_true() -> bool {
    true
}

pub fn default_min_node_version() -> String {
    "v0.2.0".to_string()
}

pub fn default_proxy_host_security() -> ProxyHostSecurity {
    ProxyHostSecurity::InboundDefault
}

pub fn default_with_0() -> u32 {
    0
}

pub fn default_data_limit_reset_strategy() -> UserDataLimitResetStrategy {
    UserDataLimitResetStrategy::NoReset
}

pub fn default_empty_string() -> String {
    "".to_string()
}
