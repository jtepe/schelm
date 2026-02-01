use crate::client::{Error, Result};

/// Normalizes a base URL so that URL joining behaves like appending path segments.
///
/// In particular, `Url::join` treats a base URL without a trailing slash as a "file",
/// so `https://host/v1` joined with `responses` becomes `https://host/responses`.
/// We normalize to ensure the base path ends with `/`.
pub(crate) fn normalize_base_url(mut base_url: url::Url) -> url::Url {
    if !base_url.path().ends_with('/') {
        let new_path = format!("{}/", base_url.path());
        base_url.set_path(&new_path);
    }
    base_url
}

/// Joins a relative endpoint path onto a normalized base URL.
pub(crate) fn join(base_url: &url::Url, path: &str) -> Result<url::Url> {
    let base = normalize_base_url(base_url.clone());
    Ok(base.join(path)?)
}

pub(crate) async fn read_error_body(resp: reqwest::Response) -> Result<Error> {
    let status = resp.status();
    let body = resp.text().await.unwrap_or_default();
    Ok(Error::HttpStatus { status, body })
}
