use serde::{Deserialize, Serialize};

use crate::models::base::default_proxy_host_security;

#[derive(Serialize, Deserialize)]
pub struct ProxyHost {
    pub remark: String,
    pub address: String,
    pub port: Option<u32>,
    pub sni: Option<String>,
    pub host: Option<String>,
    pub path: Option<String>,
    #[serde(default = "default_proxy_host_security")]
    pub security: ProxyHostSecurity,
    pub alpn: Option<ProxyHostALPN>,
    pub fingerprint: Option<ProxyHostFingerprint>,
    pub allow_insecure: Option<bool>,
    pub is_disabled: Option<bool>,
}

#[derive(Serialize, Deserialize)]
pub enum ProxyHostALPN {
    #[serde(rename = "h2")]
    H2,
    #[serde(rename = "http/1.1")]
    Http11,
    #[serde(rename = "h2,http/1.1")]
    H2andHttp11,
}

#[derive(Serialize, Deserialize)]
pub enum ProxyHostFingerprint {
    #[serde(rename = "chrome")]
    Chrome,
    #[serde(rename = "firefox")]
    Firefox,
    #[serde(rename = "safari")]
    Safari,
    #[serde(rename = "ios")]
    Ios,
    #[serde(rename = "android")]
    Android,
    #[serde(rename = "edge")]
    Edge,
    #[serde(rename = "360")]
    Fingerprint360,
    #[serde(rename = "qq")]
    Qq,
    #[serde(rename = "random")]
    Random,
    #[serde(rename = "randomized")]
    Randomized,
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
    pub tag: String,
    pub protocol: ProxyTypes,
    pub network: String,
    pub tls: String,
    pub port: ProxyInboundPort,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProxyInboundPort {
    String(String),
    Integer(u32),
}

#[derive(Serialize, Deserialize)]
pub struct ProxySettings {
    pub id: Option<String>,
}

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
