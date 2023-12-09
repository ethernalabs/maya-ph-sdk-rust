use async_trait::async_trait;
use maya_client_sdk::MayaClient;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

pub use self::payment::CardDetails;

pub mod payment;
