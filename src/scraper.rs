use crate::models::{FetchError, PageData};
use color_eyre::eyre::{Context, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use url::Url;

/// Fetches a single page, extracts all unique INTERNAL links, and then sorts them.
///
/// # Arguments
/// * `verbose` - If true, prints the list of found URLs.
pub async fn extract_and_sort_links(
    client: &Client,
    base_url: &Url,
    verbose: bool,
) -> Result<Vec<Url>> {
    let response = client
        .get(base_url.clone())
        .send()
        .await
        .with_context(|| format!("Failed to fetch initial URL: {base_url}"))?;

    if !response.status().is_success() {
        color_eyre::eyre::bail!(
            "Initial URL fetch failed with status: {}",
            response.status()
        );
    }

    let html = response.text().await?;
    let document = Html::parse_document(&html);
    let link_selector = Selector::parse("a[href]").unwrap();

    let base_host = base_url.host_str();

    let unique_urls_set: std::collections::HashSet<_> = document
        .select(&link_selector)
        .filter_map(|element| element.value().attr("href"))
        .filter_map(|href| base_url.join(href).ok())
        .map(|mut url| {
            url.set_fragment(None);
            url
        })
        .filter(|url| url.host_str() == base_host)
        .filter(|url| !url.path().contains("/cdn-cgi/l/email-protection"))
        .collect();

    let mut final_urls: Vec<_> = unique_urls_set.into_iter().collect();
    final_urls.sort_by(|a, b| a.path().cmp(b.path()));

    if verbose {
        println!("\n🔍 Found {} internal links to process:", final_urls.len());
        for url in &final_urls {
            println!("   - {url}");
        }
        println!();
    }

    Ok(final_urls)
}

/// Fetches the content of a single page.
pub async fn fetch_page(client: Client, url: Url) -> Result<PageData, FetchError> {
    match client.get(url.clone()).send().await {
        Ok(resp) => {
            let status = resp.status();
            if !status.is_success() {
                return Err(FetchError {
                    url,
                    reason: format!("HTTP {status}"),
                });
            }
            match resp.text().await {
                Ok(html) => Ok(PageData { url, html }),
                Err(e) => Err(FetchError {
                    url,
                    reason: format!("read body: {e}"),
                }),
            }
        }
        Err(e) => Err(FetchError {
            url,
            reason: format!("network: {e}"),
        }),
    }
}
