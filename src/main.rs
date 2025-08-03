mod models;
mod output;
mod scraper;
mod utils;

use clap::{ArgGroup, Parser};
use color_eyre::eyre::{Context, Result, bail};
use futures::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use promkit::preset::confirm::Confirm;
use reqwest::Client;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use tokio::fs;
use url::Url;

/// A powerful CLI tool to scrape URLs and save their content as a single Markdown file.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(["targets", "urls_from_file"]),
))]
struct Cli {
    /// URLs to process.
    /// - If one URL is provided, all internal links on that page are also scraped.
    /// - If multiple URLs are given, or with the --single flag, only those specific pages are processed.
    #[arg(value_name = "URL")]
    targets: Vec<Url>,

    /// Read a list of URLs from a file (one URL per line).
    #[arg(long = "urls", value_name = "PATH")]
    urls_from_file: Option<PathBuf>,

    /// Process only the given URL, without scraping its internal links.
    /// This flag only has an effect when a single URL is given.
    #[arg(long)]
    single: bool,

    /// The name of a custom output file.
    /// If not provided, a name is generated automatically from the first input URL.
    #[arg(long)]
    output: Option<PathBuf>,

    /// Copy the final Markdown output to the clipboard after saving the file.
    #[arg(long)]
    clipboard: bool,

    /// Bypass confirmation prompts for scraping multiple links.
    #[arg(long)]
    yes: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let raw_urls = if let Some(path) = &cli.urls_from_file {
        let content = fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read URL file: {}", path.display()))?;
        content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .filter_map(|line| line.parse::<Url>().ok())
            .collect::<Vec<_>>()
    } else {
        cli.targets
    };

    if raw_urls.is_empty() {
        bail!("No valid URLs were provided.");
    }

    let output_path = cli
        .output
        .unwrap_or_else(|| utils::gen_filename(&raw_urls[0]));

    let client = Client::builder()
        .user_agent(format!(
            "{}/{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .timeout(Duration::from_secs(20))
        .build()
        .context("failed to build HTTP client")?;

    let urls_to_fetch = if cli.urls_from_file.is_none() && raw_urls.len() == 1 && !cli.single {
        println!("🔍 Discovering links...");
        let discovered_links = scraper::extract_and_sort_links(&client, &raw_urls[0])
            .await
            .context("failed to extract internal links")?;

        if !cli.yes && !discovered_links.is_empty() {
            println!(
                "\nFound {} internal links to process:",
                discovered_links.len()
            );
            for (i, url) in discovered_links.iter().enumerate() {
                println!("   {:<3} - {}", i + 1, url);
            }

            println!();

            let mut confirm = Confirm::new("Proceed with scraping these links?");

            match confirm.run().await {
                Ok(answer) => {
                    if !["y", "yes"].contains(&answer.to_lowercase().as_str()) {
                        println!("🚫 Operation aborted by user.");
                        return Ok(());
                    }
                }
                Err(e) => {
                    bail!("Failed to run confirmation prompt: {}", e);
                }
            }
        }
        println!();
        discovered_links
    } else {
        raw_urls
    };

    if urls_to_fetch.is_empty() {
        println!("No URLs found to process.");
        return Ok(());
    }

    let multi = MultiProgress::new();
    let bar_style = ProgressStyle::default_bar()
        .template("{bar:40.cyan/blue} {pos}/{len} ({eta})\n{msg}")
        .unwrap();

    let pb = multi.add(ProgressBar::new(urls_to_fetch.len() as u64));
    pb.set_style(bar_style);
    pb.set_message("Starting download...");

    let fetch_stream = futures::stream::iter(urls_to_fetch.iter().cloned().map(|url| {
        let c = client.clone();
        tokio::spawn(scraper::fetch_page(c, url))
    }))
    .buffer_unordered(num_cpus::get());

    tokio::pin!(fetch_stream);

    let mut fetched_pages = Vec::new();
    let mut fetch_errors = Vec::new();

    while let Some(res) = fetch_stream.next().await {
        match res {
            Ok(Ok(page)) => {
                let msg = if page.url.as_str().len() > 80 {
                    format!("Downloading {}...", &page.url.as_str()[..77])
                } else {
                    format!("Downloading {}", page.url)
                };
                pb.set_message(msg);
                fetched_pages.push(page);
            }
            Ok(Err(e)) => fetch_errors.push(e),
            Err(e) => eprintln!("⚠️ Task join error: {e}"),
        }
        pb.inc(1);
    }
    pb.finish_and_clear();

    println!("📝 Converting pages to Markdown...");

    let url_order_map: HashMap<_, _> = urls_to_fetch
        .iter()
        .enumerate()
        .map(|(i, url)| (url, i))
        .collect();
    fetched_pages.sort_by_key(|page| url_order_map.get(&page.url).copied().unwrap_or(usize::MAX));

    let markdown_opt = output::save_to_markdown_async(&fetched_pages, &output_path, cli.clipboard)
        .await
        .context("failed to save markdown")?;

    println!("\n✅ {} pages successfully processed.", fetched_pages.len());
    println!(
        "📄 Output saved to: {}",
        dunce::canonicalize(&output_path)?.display()
    );

    if !fetch_errors.is_empty() {
        println!("\n⚠️  {} pages failed to fetch:", fetch_errors.len());
        let mut rate_limited = false;
        for err in &fetch_errors {
            println!("   - {}: {}", err.url, err.reason);
            if err.reason.contains("429") || err.reason.contains("Too Many Requests") {
                rate_limited = true;
            }
        }
        if rate_limited {
            println!(
                "\n💡 It looks like you are being rate-limited; try rerunning after a short wait."
            );
        }
    }

    if cli.clipboard {
        if let Some(md) = markdown_opt {
            let mut cb = arboard::Clipboard::new().context("clipboard init")?;
            cb.set_text(md).context("clipboard copy")?;
            println!("📋 Content copied to clipboard.");
        }
    }

    Ok(())
}
