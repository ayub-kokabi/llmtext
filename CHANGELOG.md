# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3] - 2025-08-01

### Added
- **Intelligent Link Filtering**: Implemented a heuristic-based algorithm to automatically detect the primary content prefix of a website (e.g., `/docs/`). This filters out irrelevant site-wide navigation links, resulting in a highly focused and relevant scrape.
- **Interactive Confirmation Prompt**: Before initiating a multi-page scrape, the tool now displays a list of relevant URLs found and requires user confirmation to proceed.
- **Content Cleaning with Readability**: Integrated the `readability-rust` library to parse and extract only the main article content from each page. This removes clutter like ads, navigation bars, and footers, ensuring the final Markdown output is clean and LLM-ready.

### Changed
- The URL header and separator (`=====`) are no longer added to the Markdown output, creating a seamless, single-document feel.

## [0.1.2] - 2025-07-27

### Fixed
- Resolved a build failure for users installing with `cargo install` on systems that use OpenSSL by switching to the `rustls-tls` feature for `reqwest`. This removes the dependency on native OpenSSL libraries.

## [0.1.1] - 2025-07-27

### Fixed
- Resolved a build failure.