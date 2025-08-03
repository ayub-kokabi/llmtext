use crate::models::{FetchError, PageData};
use color_eyre::eyre::{Context, Result};
use rayon::prelude::*;
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use url::Url;

/// Finds the most common path prefix among a list of URLs using parallel processing.
/// This helps to identify the primary content section (e.g., "/docs/").
fn find_best_prefix(urls: &[Url]) -> Option<String> {
    let prefix_counts: HashMap<String, usize> = urls
        .par_iter()
        .filter(|url| url.path() != "/")
        .flat_map(|url| {
            let segments: Vec<&str> = url.path().split('/').filter(|s| !s.is_empty()).collect();
            (0..segments.len())
                .into_par_iter() // This is the fix to make the inner iterator parallel
                .map(move |i| format!("/{}/", segments[0..=i].join("/")))
        })
        .fold(HashMap::new, |mut acc, prefix| {
            *acc.entry(prefix).or_insert(0) += 1;
            acc
        })
        .reduce(HashMap::new, |mut acc, other| {
            for (k, v) in other {
                *acc.entry(k).or_insert(0) += v;
            }
            acc
        });

    let threshold = ((urls.len() as f64 * 0.7).ceil() as usize).max(2);

    prefix_counts
        .into_iter()
        .filter(|&(_, count)| count >= threshold)
        .max_by_key(|&(_, count)| count)
        .map(|(prefix, _)| prefix)
}

/// Fetches a single page, extracts all unique INTERNAL links, filters them
/// by the most common path prefix, and then sorts them.
pub async fn extract_and_sort_links(client: &Client, base_url: &Url) -> Result<Vec<Url>> {
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

    let all_internal_urls: Vec<_> = document
        .select(&link_selector)
        .filter_map(|element| element.value().attr("href"))
        .filter_map(|href| base_url.join(href).ok())
        .map(|mut url| {
            url.set_fragment(None);
            url
        })
        .filter(|url| url.host_str() == base_host)
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();

    let best_prefix_opt = find_best_prefix(&all_internal_urls);

    let mut final_urls: Vec<_> = if let Some(prefix) = best_prefix_opt {
        all_internal_urls
            .into_iter()
            .filter(|url| url.path().starts_with(&prefix))
            .collect()
    } else {
        all_internal_urls
    };

    if !final_urls.iter().any(|u| u == base_url) {
        final_urls.push(base_url.clone());
    }

    final_urls.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    final_urls.dedup();

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
