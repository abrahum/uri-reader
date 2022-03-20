use std::collections::HashMap;

use hyper::header::{HeaderName, HeaderValue};
use hyper::http::Error as HttpError;
use hyper::{client::HttpConnector, Body, Client, Request, Uri};
use hyper_tls::HttpsConnector;

pub(crate) fn http_client() -> Client<HttpConnector, Body> {
    Client::new()
}

pub(crate) fn https_client() -> Client<HttpsConnector<HttpConnector>, Body> {
    Client::builder().build::<_, Body>(HttpsConnector::new())
}

pub async fn http<T, K, V>(
    uri: Uri,
    cli: Client<T>,
    headers: HashMap<K, V>,
) -> Result<Vec<u8>, crate::UReadError>
where
    T: hyper::client::connect::Connect + Clone + Send + Sync + 'static,
    HeaderName: TryFrom<K>,
    <HeaderName as TryFrom<K>>::Error: Into<HttpError>,
    HeaderValue: TryFrom<V>,
    <HeaderValue as TryFrom<V>>::Error: Into<HttpError>,
{
    let mut builder = Request::builder().uri(uri);
    for (k, v) in headers {
        builder = builder.header(k, v);
    }
    let req = builder.body(Body::empty()).unwrap();
    let resp = cli.request(req).await?;
    hyper::body::to_bytes(resp.into_body())
        .await
        .map(|b| b.to_vec())
        .map_err(crate::UReadError::Hyper)
}
