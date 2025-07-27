use std::path::PathBuf;
use url::Url;

pub fn gen_filename(first: &Url) -> PathBuf {
    let host = first.host_str().unwrap_or("output");
    let path = first.path().trim_matches('/');
    let name = if path.is_empty() {
        host.to_owned()
    } else {
        format!("{host}_{path}")
    };
    PathBuf::from(format!("{}.md", sanitize_filename::sanitize(name)))
}
