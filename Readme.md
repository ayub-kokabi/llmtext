# llmtext 🤖🔗📝

**Turn any website into a single Markdown file, ready to be used as updated context for any LLM chatbot**

[![Crates.io](https://img.shields.io/crates/v/llmtext.svg)](https://crates.io/crates/llmtext)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/sir-kokabi/llmtext/rust.yml?branch=main)](https://github.com/sir-kokabi/llmtext/actions)

---

## The Problem

LLMs have a knowledge cutoff date. Ask a question about a new library version, and you'll likely get an outdated answer. Manually copy-pasting documentation into your prompt is slow, tedious, and error-prone.

## The Solution: `llmtext`

`llmtext` is a lightning-fast CLI tool that automates this process. Just give it a URL — it scrapes the page and all internal links it references, converts the content into Markdown, and saves everything as a single file. This file gives your LLM the up-to-date context it needs to provide accurate answers.

![usage](https://github.com/user-attachments/assets/cbd04520-b492-4a14-ae84-3cb59f330465)

## ✨ Key Features

-   **🎯 Easy usage**: Give it one URL, and it scrapes all internal links on that page.
-   **📄 Single Page Mode**: Need just one page? Use the `--single` flag.
-   **📁 Fetch from File**: Process a list of URLs from a text file.
-   **⚡ Blazing Fast**: Built in Rust with async requests for maximum speed.
-   **📋 Clipboard Integration**: Copy the full Markdown output directly to your clipboard.
-   **🤖 LLM-Ready Output**: Clean, structured Markdown perfect for any AI prompt.

## 📦 Installation

```bash
# From Crates.io (Recommended)
cargo install llmtext

# From GitHub (Latest development version)
cargo install --git https://github.com/sir-kokabi/llmtext.git
```

## 🚀 Usage Examples

#### 1. Scrape a Documentation Section

This command finds all links on the target page and scrapes their content.

```bash
# Scrape the React reference documentation
llmtext https://react.dev/reference/react
```
This saves the content to a file like `react.dev_reference_react.md`.

#### 2. Get a Single Page

Use the `-s` or `--single` flag to get content from only one URL, without internal links.

```bash
llmtext -s https://react.dev/reference/react
```

#### 3. Fetch a List of URLs from a File

Create a file `links.txt` with one URL per line:

```
https://react.dev/learn
https://react.dev/learn/describing-the-ui
https://react.dev/learn/adding-interactivity
```

Then, run `llmtext` with the `-u` or `--urls` flag:

```bash
llmtext -u links.txt -o react-docs.md
```

## 📋 Command-Line Options

| Flag                        | Short | Description                                          |
| --------------------------- | ----- | ---------------------------------------------------- |
| `--urls <PATH>`             | `-u`  | Read URLs from a file.                               |
| `--single`                  | `-s`  | Process only the single URL provided.                |
| `--output <PATH>`           | `-o`  | Specify a custom output file name.                   |
| `--parallel <NUM>`          | `-p`  | Set the number of parallel downloads.                |
| `--clipboard`               | `-c`  | Copy the final Markdown to the clipboard.            |
| `--verbose`                 | `-v`  | Show detailed processing steps.                      |

## License
This project is licensed under the MIT License. See the [LICENSE](https://github.com/sir-kokabi/llmtext/blob/main/LICENCE) file for details.

## Contributing
Contributions, issues, and feature requests are welcome! Check the [issues page](https://github.com/sir-kokabi/llmtext/issues).
