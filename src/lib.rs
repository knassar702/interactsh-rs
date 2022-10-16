//! # Interactsh-rs
//! A Rust client library for getting interaction logs from Interactsh servers.
//!
//! ### Basic Use
//! ```
//! use std::time::Duration;
//! use std::thread;
//! use interactsh_rs::prelude::*;
//!
//! async fn run_client() {
//!     // Builds an unregistered client
//!     let client = ClientBuilder::default()
//!         .with_server("oast.pro".into())
//!         .parse_logs(true)
//!         .build()
//!         .unwrap();
//!
//!     // Registers the client with the server and
//!     // returns a registered client
//!     let client = client.register().await.unwrap();
//!     let interaction_url = client.get_interaction_url();
//!     println!("INTERACTION URL: {}", interaction_url);
//!
//!     // Start a poll loop
//!     loop {
//!         thread::sleep(Duration::from_secs(5));
//!
//!         let logs = match client.poll().await.unwrap() {
//!             Some(logs) => logs,
//!             None => continue,
//!         };
//!
//!         // ...Do something with the returned logs...
//!     }
//!
//!     // Once done, deregister the client
//!     client.deregister().await.unwrap();
//! }
//! ```

#![cfg_attr(feature = "nightly", feature(doc_auto_cfg))]
#![cfg_attr(feature = "nightly", feature(error_generic_member_access))]
#![cfg_attr(feature = "nightly", feature(provide_any))]

#[cfg(any(feature = "rustcrypto", feature = "openssl"))]
pub(crate) mod crypto;

#[cfg(any(feature = "reqwest-rustls-tls", feature = "reqwest-native-tls"))]
pub mod client;
pub mod errors;
pub mod interaction_log;

pub mod prelude {
    #[cfg(any(feature = "reqwest-rustls-tls", feature = "reqwest-native-tls"))]
    pub use crate::client::*;
    pub use crate::interaction_log::*;
}
