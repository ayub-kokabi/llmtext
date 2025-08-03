# llmtext 🤖🔗📝

Turn any website into a single, clean Markdown file, ready for your LLM prompts.

Language models have a knowledge cut-off date. `llmtext` solves this by scraping a webpage and all its relevant internal links, converting the content into a single Markdown file. You can then use this file to give your LLM the up-to-date context it needs.

[![Crates.io](https://img.shields.io/crates/v/llmtext.svg)](https://crates.io/crates/llmtext)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

https://github.com/user-attachments/assets/a558970e-4d75-4b41-8c2c-3bb3d5705e13

## ✨ Features

-   **Intelligent Scraping**: Give it one URL, and it smartly finds and scrapes all related pages.
-   **Flexible Input**: Scrape a single page (`--single`) or a list of URLs from a file (`--urls`).
-   **Fast & Efficient**: Built in Rust for maximum speed with parallel downloads.
-   **Clipboard Integration**: Instantly copy the full Markdown output to your clipboard (`--clipboard`).

## 📦 Installation

```bash
# From Crates.io (Recommended)
cargo install llmtext
```

## 🚀 Usage

```bash
# Scrape a site's documentation section
llmtext https://react.dev/reference/react

# Scrape just a single page
llmtext --single https://react.dev/reference/react/useState

# Scrape a list of URLs from a file and save to a custom output file
llmtext --urls my_links.txt --output react-subset.md

# Scrape and copy directly to clipboard, skipping confirmation
llmtext https://vuejs.org/guide/introduction.html --clipboard --yes
```

## License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.