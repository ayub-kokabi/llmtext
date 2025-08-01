use crate::models::PageData;
use color_eyre::eyre::{Context, Result};
use futures::{stream, StreamExt};
use readability_rust::{Readability, ReadabilityOptions};
use std::path::Path;
use tokio::{fs::OpenOptions, io::AsyncWriteExt};

/// Streams the Markdown content to disk.
/// If `keep_in_memory` is true, it also returns the full content.
pub async fn save_to_markdown_async(
    pages: &[PageData],
    path: &Path,
    verbose: bool,
    keep_in_memory: bool,
) -> Result<Option<String>> {
    if verbose {
        println!("📝 Generating cleaned markdown → {}", path.display());
    }

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
                let options = ReadabilityOptions {
                    char_threshold: 100,
                    keep_classes: false,
                    ..Default::default()
                };

                let Ok(mut parser) = Readability::new(&page.html, Some(options)) else {
                    return String::new();
                };
                
                let article_result = parser.parse();

                let clean_html = if let Some(article) = article_result {
                    article.content.unwrap_or_default()
                } else {
                    String::new()
                };

                if clean_html.is_empty() {
                    return String::new();
                }

                let mut s = String::new();
                if i > 0 {
                    s.push_str("\n\n\n");
                }
                
                s.push_str(&html2md::parse_html(&clean_html));
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