use crate::models::{FetchError, PageData};
use color_eyre::eyre::{Context, Result};
use reqwest::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use url::Url;

/// Finds the most common path prefix among a list of URLs.
/// This helps to identify the primary content section (e.g., "/docs/").
fn find_best_prefix(urls: &[Url]) -> Option<String> {
    let mut prefix_counts: HashMap<String, usize> = HashMap::new();

    for url in urls {
        let path = url.path();
        // Don't count the root path "/" as a prefix for this logic.
        if path == "/" {
            continue;
        }
        let segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        
        if !segments.is_empty() {
             for i in 0..segments.len() {
                // Build prefix up to the current segment
                let prefix_path = format!("/{}/", segments[0..=i].join("/"));
                *prefix_counts.entry(prefix_path).or_insert(0) += 1;
            }
        }
    }

    // Find the prefix with the highest count that appears in a significant number of URLs.
    // A threshold helps avoid choosing overly specific prefixes.
    // Setting it to 70% of total links, but at least 2.
    let threshold = ((urls.len() as f64 * 0.7).ceil() as usize).max(2);

    prefix_counts
        .into_iter()
        .filter(|&(_, count)| count >= threshold) // Must meet the threshold
        .max_by_key(|&(_, count)| count)
        .map(|(prefix, _)| prefix)
}

/// Fetches a single page, extracts all unique INTERNAL links, filters them
/// by the most common path prefix, and then sorts them.
///
/// # Arguments
/// * `verbose` - If true, prints intermediate steps.
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
    
    if verbose {
        println!("🔍 Found a total of {} unique internal links initially.", all_internal_urls.len());
    }
    
    let best_prefix_opt = find_best_prefix(&all_internal_urls);

    let mut final_urls: Vec<_> = if let Some(prefix) = best_prefix_opt {
        if verbose {
            println!("🧠 Identified common path prefix: {}", prefix);
        }
        all_internal_urls
            .into_iter()
            .filter(|url| url.path().starts_with(&prefix))
            .collect()
    } else {
        if verbose {
             println!("⚠️ Could not determine a common path prefix. Keeping all links.");
        }
        all_internal_urls
    };
    
    if !final_urls.iter().any(|u| u == base_url) {
        final_urls.push(base_url.clone());
    }

    // Final sort and deduplication
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