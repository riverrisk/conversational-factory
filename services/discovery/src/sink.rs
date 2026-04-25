use std::future::Future;
use std::pin::Pin;

use cf_shared::assets::Observation;

pub trait ObservationSink: Send + Sync {
    fn send<'a>(
        &'a self,
        observation: &'a Observation,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>>;
}

pub struct StdoutSink;

impl ObservationSink for StdoutSink {
    fn send<'a>(
        &'a self,
        observation: &'a Observation,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move {
            let line = serde_json::to_string(observation).map_err(|e| e.to_string())?;
            println!("{line}");
            Ok(())
        })
    }
}

pub struct HttpSink {
    base_url: String,
    client: reqwest::Client,
    token: Option<String>,
}

impl HttpSink {
    pub fn new(base_url: String, token: Option<String>) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
            token,
        }
    }
}

impl ObservationSink for HttpSink {
    fn send<'a>(
        &'a self,
        observation: &'a Observation,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'a>> {
        Box::pin(async move {
            let url = format!("{}/api/v1/observations", self.base_url);
            let mut req = self.client.post(&url).json(observation);
            if let Some(token) = &self.token {
                req = req.bearer_auth(token);
            }
            let resp = req.send().await.map_err(|e| e.to_string())?;
            if !resp.status().is_success() {
                return Err(format!("registry returned {}", resp.status()));
            }
            Ok(())
        })
    }
}
