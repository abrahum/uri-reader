mod error;
mod http;
mod test;

pub use error::UReadError;
use http::{http, http_client, https_client};
use hyper::header::{HeaderName, HeaderValue};
use hyper::http::Error as HttpError;
use hyper::Uri;
use std::{collections::HashMap, str::FromStr};

pub async fn uget<K, V>(s: &str, headers: HashMap<K, V>) -> Result<Vec<u8>, UReadError>
where
    HeaderName: TryFrom<K>,
    <HeaderName as TryFrom<K>>::Error: Into<HttpError>,
    HeaderValue: TryFrom<V>,
    <HeaderValue as TryFrom<V>>::Error: Into<HttpError>,
{
    let uri = uri_parse(s)?;
    match uri.scheme_str() {
        Some("file") => file(uri).await,
        Some("http") => http(uri, http_client(), headers).await,
        Some("https") => http(uri, https_client(), headers).await,
        Some("base64") => b64(uri),
        // Some("ftp") => todo!(),
        Some(scheme) => Err(UReadError::UnsupportedScheme(scheme.to_string())),
        None => Err(UReadError::EmptyScheme),
    }
}

pub fn uri_parse(s: &str) -> Result<Uri, UReadError> {
    // uri crate didn't parse `file:///` as a scheme.
    // use path() to get path.
    Uri::from_str(&s.replace("file:///", "file://_/")).map_err(UReadError::Uri)
}

pub fn b64(uri: Uri) -> Result<Vec<u8>, UReadError> {
    base64::decode(uri.authority().map(|a| a.as_str()).unwrap_or("")).map_err(UReadError::Base64)
}

pub async fn file(uri: Uri) -> Result<Vec<u8>, UReadError> {
    use tokio::{fs::File, io::AsyncReadExt};
    let path = std::path::PathBuf::from(uri.path());
    let mut file = File::open(path).await?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;
    Ok(buf)
}
