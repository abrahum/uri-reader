#![doc = include_str!("../README.md")]

mod error;
mod http;
mod test;

pub use error::UReadError;
use http::{http, http_client, https_client};
use hyper::header::{HeaderName, HeaderValue};
use hyper::http::Error as HttpError;
use hyper::Uri;
use std::path::Path;
use std::{collections::HashMap, str::FromStr};

pub async fn uget(s: &str) -> Result<Vec<u8>, UReadError> {
    uget_with_headers::<&str, &str>(s, [].into()).await
}

#[derive(Debug)]
pub enum Scheme<'a> {
    Http(Uri),
    Https(Uri),
    Base64(&'a str),
    File(&'a Path),
}

pub async fn uget_with_headers<K, V>(s: &str, headers: HashMap<K, V>) -> Result<Vec<u8>, UReadError>
where
    HeaderName: TryFrom<K>,
    <HeaderName as TryFrom<K>>::Error: Into<HttpError>,
    HeaderValue: TryFrom<V>,
    <HeaderValue as TryFrom<V>>::Error: Into<HttpError>,
{
    match uri_parse(s) {
        Some(Scheme::File(path)) => file(path).await,
        Some(Scheme::Http(uri)) => http(uri, http_client(), headers).await,
        Some(Scheme::Https(uri)) => http(uri, https_client(), headers).await,
        Some(Scheme::Base64(s)) => b64(s),
        // Some("ftp") => todo!(),
        None => Err(UReadError::EmptyScheme),
    }
}

pub fn uri_parse<'a>(s: &'a str) -> Option<Scheme<'a>> {
    if let Some(b64) = s.strip_prefix("base64://") {
        Some(Scheme::Base64(b64))
    } else if let Some(file) = s.strip_prefix("file:///") {
        Some(Scheme::File(Path::new(file)))
    } else if let Ok(uri) = Uri::from_str(s) {
        match uri.scheme_str() {
            Some("http") => Some(Scheme::Http(uri)),
            Some("https") => Some(Scheme::Https(uri)),
            _ => None,
        }
    } else {
        None
    }
}

pub fn b64(s: &str) -> Result<Vec<u8>, UReadError> {
    base64::decode(s).map_err(UReadError::Base64)
}

pub async fn file(path: &Path) -> Result<Vec<u8>, UReadError> {
    use tokio::{fs::File, io::AsyncReadExt};
    let mut file = File::open(path).await?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;
    Ok(buf)
}
