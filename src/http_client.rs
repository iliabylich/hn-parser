use anyhow::{Context as _, Result, bail};
use http_body_util::{BodyExt as _, Empty};
use hyper::body::Bytes;
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::{
    client::legacy::{Client as HyperClient, connect::HttpConnector},
    rt::TokioExecutor,
};
use serde::de::DeserializeOwned;

pub(crate) struct Client {
    inner: HyperClient<HttpsConnector<HttpConnector>, Empty<Bytes>>,
}

impl Client {
    pub(crate) fn new() -> Self {
        let connector = HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_only()
            .enable_http1()
            .build();
        let inner = HyperClient::builder(TokioExecutor::new()).build(connector);
        Self { inner }
    }

    pub(crate) async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let uri = url.parse().context("invalid url")?;
        let response = self.inner.get(uri).await.context("request failed")?;
        let status = response.status();
        if !status.is_success() {
            bail!("unexpected status {status} from {url}");
        }
        let body = response
            .into_body()
            .collect()
            .await
            .context("failed to read response body")?
            .to_bytes();
        serde_json::from_slice(&body).context("failed to parse response body")
    }
}
