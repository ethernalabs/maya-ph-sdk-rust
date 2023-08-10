use async_trait::async_trait;
use maya_client_sdk::MayaClient;
use reqwest::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Response,
};
use serde::Serialize;

pub use self::payment::CardDetails;

pub mod payment;
