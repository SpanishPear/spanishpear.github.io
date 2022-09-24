use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

/// Something wrong has occurred while fetching an external resource.
#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

/// The possible states a fetch request can be in.
pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}

pub enum UrlType {
    File,
    Url,
}

/// Fetches markdown
///
/// Consult the following for an example of the fetch api by the team behind web_sys:
/// https://rustwasm.github.io/wasm-bindgen/examples/fetch.html
pub async fn fetch_markdown(url: &'static str, url_type: UrlType) -> Result<String, FetchError> {
    match url_type {
        UrlType::File => fetch_file(url).await,
        UrlType::Url => fetch_url(url).await,
    }
}

/// fetch a file from the local filesystem
pub async fn fetch_file(url: &'static str) -> Result<String, FetchError> {
    // convert url to absolute path
    let absolute_url = url.trim_start_matches("./");

    // request from root/url
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(absolute_url, &opts)?;
    Ok(absolute_url.to_string())
}

/// fetch markdown from a remote URL
pub async fn fetch_url(url: &'static str) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = gloo::utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    Ok(text.as_string().unwrap())
}
