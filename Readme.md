# llmtext 🤖🔗📝

**Tired of outdated LLM answers? `llmtext` scrapes entire documentation sites into a single file, giving your AI the up-to-date context it needs.**

[![Crates.io](https://img.shields.io/crates/v/llmtext.svg)](https://crates.io/crates/llmtext)
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/your-username/llmtext/rust.yml?branch=main)](https://github.com/sir-kokabi/llmtext/actions)

---

## The Problem

You're working with the latest version of a framework like React, Rust's Tokio, or Vue. You ask your favorite Large Language Model (LLM) a question, but it confidently gives you an answer that's... wrong. It's using an API from two versions ago.

The reason? LLMs have a **knowledge cutoff date**. They don't know about the latest updates, API changes, or new features in the documentation you're reading. The only way to get accurate answers is to manually copy and paste page after page of documentation into the prompt. It's tedious, error-prone, and kills your productivity.

## The Solution

**Enter `llmtext`**.

`llmtext` is a lightning-fast command-line tool that automates this entire process. You give it one URL, and it intelligently finds all the internal links, fetches their content, and compiles everything into a **single, clean Markdown file**.

This file is perfectly formatted to be used as a context source for any LLM, effectively giving it a "brain upgrade" with the latest information.

## ✨ Key Features

-   **🚀 Single-URL Scraping**: Provide one URL (like the homepage of a documentation site), and `llmtext` will discover and scrape all internal pages.
-   **🎯 Multi-URL Fetching**: Provide a list of specific URLs to fetch only the pages you need.
-   **⚡ Blazing Fast**: Uses asynchronous requests and parallel downloads to fetch content quickly.
-   **📋 Clipboard Integration**: Instantly copy the entire Markdown output to your clipboard with the `-C` flag.
-   **🤖 LLM-Ready Output**: Converts all content to clean Markdown, ideal for pasting into LLM prompts.
-   **🤫 Quiet by Default**: No unnecessary output, but a `-v` / `--verbose` flag is available to see all the details.

## 📦 Installation

There are several ways to install `llmtext`, depending on your needs.

### Option 1: From Crates.io (Recommended)

This is the easiest method for most users. If you have the Rust toolchain, you can install the latest stable version directly from the official Rust package registry.

```bash
cargo install llmtext
```

### Option 2: From GitHub (Latest Development Version)

If you want the absolute latest features and updates that haven't been published to Crates.io yet, you can install directly from the `main` branch of this repository.

```bash
cargo install --git https://github.com/sir-kokabi/llmtext.git
```

To update to the latest version later, simply run the same command again with the `--force` flag:
```bash
cargo install --git https://github.com/sir-kokabi/llmtext.git --force
```

### Option 3: For Contributors (Build from Local Clone)

If you plan to contribute to the project, you'll need to clone the repository and build it locally. This allows you to make changes and test them.

```bash
git clone https://github.com/sir-kokabi/llmtext.git
cd llmtext

cargo install --path .
```

##  usage

### 1. Scrape an Entire Site

This is the primary use case. Just provide the base URL of the documentation.

```bash
llmtext https://tokio.rs/tokio/tutorial
```

This command will find all links within `tokio.rs` that are part of the tutorial, fetch them, and save them to a file named `tokio.rs_tokio_tutorial.md`.

### 2. Fetch a Specific List of URLs

If you only need a few pages, list them out.

```bash
llmtext https://react.dev/learn/state-a-components-memory https://react.dev/learn/responding-to-events
```

### 3. Command-Line Options

Customize the behavior with these flags:

| Flag                        | Description                                          | Example                                                  |
| --------------------------- | ---------------------------------------------------- | -------------------------------------------------------- |
| `-o`, `--output <PATH>`     | Specify a custom output file name.                   | `llmtext -o react-docs.md https://react.dev`             |
| `-P`, `--parallel <NUM>`    | Set the number of parallel downloads.                | `llmtext --parallel 5 <URL>`                             |
| `-C`, `--clipboard`         | Copy the final Markdown to the clipboard.            | `llmtext -C <URL>`                                       |
| `-v`, `--verbose`           | Show detailed processing steps and all fetched URLs. | `llmtext -v <URL>`                                       |

## 🚀 Example Workflow: From Scraping to AI Prompt

Let's solve a real-world problem: getting up-to-date answers about the Rust `tokio` library.

**Step 1: Scrape the documentation with `llmtext`**

We'll run the tool and copy the output directly to the clipboard.

```bash
llmtext --clipboard https://tokio.rs/tokio/tutorial
```

**Step 2: Prepare your LLM Prompt**

Now, go to your favorite LLM (like ChatGPT, Claude, etc.) and use a prompt structure like this. The key is to instruct the model to *only* use the context you provide.

```
Based *only* on the following context, please answer my question. Do not use any of your prior knowledge.

--- CONTEXT ---

[PASTE THE CONTENT FROM YOUR CLIPBOARD HERE]

--- END CONTEXT ---

My Question: How do I implement a graceful shutdown for a Tokio TCP server using the mini-redis broadcast channel example?
```

**Step 3: Get an Accurate, Up-to-Date Answer!**

The LLM will now generate an answer based *exclusively* on the latest documentation you provided, completely bypassing its outdated knowledge.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions, issues, and feature requests are welcome!

Feel free to check the [issues page](https://github.com/sir-kokabi/llmtext/issues).
