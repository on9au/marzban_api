//! # Marzban API Client
//!
//! Updated for Marzban v0.7.0
//!
//! Marzban Repository: [Marzban](https://github.com/Gozargah/Marzban)
//!
//! This crate provides a client for the Marzban API.
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! marzban_api_client = "0.2.2"
//! ```
//!
//! or run this command:
//!
//! ```sh
//! cargo add marzban_api
//! ```
//!
//! ## Example
//!
//! Simple usage example:
//!
//! ```no_run
//! use marzban_api::client::MarzbanAPIClient;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = MarzbanAPIClient::new("http://localhost:8000");
//!     let base_url = client.base_url().await.unwrap();
//!     println!("Base URL: {}", base_url);
//! }
//! ```
//!
//! Auth with admin credentials and get a token:
//!
//! ```no_run
//! use marzban_api::client::MarzbanAPIClient;
//! use marzban_api::models::auth::BodyAdminTokenApiAdminTokenPost;
//!
//! #[tokio::main]
//! async fn main() {
//!     let client = MarzbanAPIClient::new("http://localhost:8000");
//!     client.authenticate(&BodyAdminTokenApiAdminTokenPost {
//!         grant_type: Some("password".to_string()),
//!         username: "admin".to_string(),
//!         password: "admin".to_string(),
//!         scope: "".to_string(),
//!         client_id: None,
//!         client_secret: None,
//!     }).await.expect("Failed to authenticate");
//!     // Client is now authenticated and token will be used in future requests
//! }
//! ```

#![forbid(unsafe_code)]
#![deny(unreachable_pub)]

pub mod api;
pub mod client;
pub mod error;
pub mod models;
