use url::Url;

#[derive(Debug, Clone)]
pub struct PageData {
    pub url: Url,
    pub html: String,
}

#[derive(Debug)]
pub struct FetchError {
    pub url: Url,
    pub reason: String,
}
