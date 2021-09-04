use anyhow::{Context, Result};
use hyper::client::connect::dns::GaiResolver;
use hyper::client::HttpConnector;
use hyper_rustls::HttpsConnector;
use std::path::Path;
use yup_oauth2::authenticator::Authenticator;
use yup_oauth2::ServiceAccountAuthenticator;

pub async fn auth(
    service_account_key_path: impl AsRef<Path>,
) -> Result<Authenticator<HttpsConnector<HttpConnector<GaiResolver>>>> {
    let service_account_key = yup_oauth2::read_service_account_key(service_account_key_path)
        .await
        .context("Failed to read service account key")?;

    let auth = ServiceAccountAuthenticator::builder(service_account_key)
        .build()
        .await
        .context("Failed to authorize with service account key")?;

    Ok(auth)
}
