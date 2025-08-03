use crate::models::PageData;
use color_eyre::eyre::{Context, Result};
use futures::{stream, StreamExt};
use regex::Regex;
use scraper::{Html, Selector};
use std::path::Path;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

/// Streams the Markdown content to disk.
/// If `keep_in_memory` is true, it also returns the full content.
pub async fn save_to_markdown_async(
    pages: &[PageData],
    path: &Path,
    keep_in_memory: bool,
) -> Result<Option<String>> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .await
        .with_context(|| format!("cannot create {path:?}"))?;

    let rendered_chunks: Vec<String> = stream::iter(pages.iter().cloned().enumerate())
        .map(|(i, page)| {
            tokio::task::spawn_blocking(move || {
                let document = Html::parse_document(&page.html);
                let body_selector = Selector::parse("body").unwrap();

                // Fallback to full HTML if body is not found
                let body_html = document
                    .select(&body_selector)
                    .next()
                    .map_or(page.html.clone(), |body| body.inner_html());

                // Remove script tags to avoid including JavaScript code in the markdown
                let script_regex = Regex::new(r"(?is)<script.*?</script>").unwrap();
                let clean_html = script_regex.replace_all(&body_html, "");

                let final_md = html2md::parse_html(&clean_html);

                let mut s = String::new();
                if !final_md.trim().is_empty() {
                    if i > 0 {
                        s.push_str("\n\n\n");
                    }
                    s.push_str(&final_md);
                }
                s
            })
        })
        .buffer_unordered(num_cpus::get())
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .filter_map(|r| r.ok())
        .filter(|s| !s.is_empty())
        .collect();

    for chunk in &rendered_chunks {
        file.write_all(chunk.as_bytes()).await?;
    }

    if keep_in_memory {
        Ok(Some(rendered_chunks.concat()))
    } else {
        Ok(None)
    }
}