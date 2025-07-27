use crate::models::PageData;
use color_eyre::eyre::{Context, Result};
use futures::{StreamExt, stream};
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
        println!("📝 Generating markdown → {}", path.display());
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
                let mut s = String::new();
                if i > 0 {
                    s.push_str("\n\n\n");
                }
                s.push_str(&format!(
                    "{} ========================================================= \n\n",
                    page.url
                ));
                s.push_str(&html2md::parse_html(&page.html));
                s
            })
        })
        .buffer_unordered(num_cpus::get())
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .map(|r| r.expect("spawn_blocking panicked"))
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
