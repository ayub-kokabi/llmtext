use crate::models::PageData;
use std::collections::HashMap;
use url::Url;

/// Sorts pages based on a predefined URL order.
///
/// # Arguments
/// * `verbose` - If true, prints a status message.
pub fn sort_pages_by_url_order(
    mut pages: Vec<PageData>,
    url_order: &[Url],
    verbose: bool,
) -> Vec<PageData> {
    if verbose {
        println!(
            "↕️  Sorting {} fetched pages into the correct order...",
            pages.len()
        );
    }

    let order_map: HashMap<_, _> = url_order
        .iter()
        .enumerate()
        .map(|(i, url)| (url, i))
        .collect();

    pages.sort_by_key(|page| order_map.get(&page.url).copied().unwrap_or(usize::MAX));

    pages
}
