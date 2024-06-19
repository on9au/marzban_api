use serde::{Deserialize, Serialize};

use crate::models::base::default_proxy_host_security;

#[derive(Serialize, Deserialize)]
pub struct ProxyHost {
    remark: String,
    address: String,
    port: Option<u32>,
    sni: Option<String>,
    host: Option<String>,
    path: Option<String>,
    #[serde(default = "default_proxy_host_security")]
    security: ProxyHostSecurity,
    alpn: Option<ProxyHostALPN>,
    fingerprint: Option<ProxyHostFingerprint>,
    allow_insecure: Option<bool>,
    is_disabled: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum ProxyHostALPN {
    // TODO!
}

#[derive(Serialize, Deserialize)]
pub enum ProxyHostFingerprint {
    // TODO!
}

#[derive(Serialize, Deserialize)]
pub enum ProxyHostSecurity {
    #[serde(rename = "inbound_default")]
    InboundDefault,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "tls")]
    Tls,
}

#[derive(Serialize, Deserialize)]
pub struct ProxyInbound {
    tag: String,
    protocol: ProxyTypes,
    network: String,
    tls: String,
    port: ProxyInboundPort,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProxyInboundPort {
    String(String),
    Integer(u32),
}

#[derive(Serialize, Deserialize)]
pub struct ProxySettings {}

#[derive(Serialize, Deserialize)]
pub enum ProxyTypes {
    #[serde(rename = "vmess")]
    Vmess,
    #[serde(rename = "vless")]
    Vless,
    #[serde(rename = "trojan")]
    Trojan,
    #[serde(rename = "shadowsocks")]
    ShadowSocks,
}
