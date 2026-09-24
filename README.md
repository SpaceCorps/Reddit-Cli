# Reddit CLI

[![Release](https://img.shields.io/github/v/release/SpaceCorps/Reddit-Cli?color=blue&label=version)](https://github.com/SpaceCorps/Reddit-Cli/releases/latest)
[![CI](https://github.com/SpaceCorps/Reddit-Cli/actions/workflows/ci.yml/badge.svg)](https://github.com/SpaceCorps/Reddit-Cli/actions/workflows/ci.yml)
[![Docs](https://img.shields.io/badge/docs-online-success)](https://spacecorps.github.io/Reddit-Cli/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A blazing fast, native command-line tool and agent interface for scraping Reddit posts, comments, communities, and user profiles via [Apify](https://apify.com). Built in Rust Edition 2024 for developers and autonomous AI workflows.

---

## Highlights

- ⚡ **Sub-3ms Startup**: Compiled as a native static binary with zero runtime dependencies. Executes in ~1–3 ms with no .NET, Node, or Python runtime needed.
- 🔐 **OS Keystore Integration**: `reddit login` prompts for your token securely and saves it to native OS vaults (macOS Keychain, Linux Secret Service / Keyutils, Windows DPAPI).
- 🔍 **Comprehensive Search & Scrape**: Search posts, comments, communities, and users; scrape discussion threads, comment hierarchies, and user profiles.
- 🤖 **AI Agent Native**: Machine-readable `--json` output, standardized error envelopes with stable exit codes, and explicit `llms.txt` agent guidance.
- 🌐 **Zero Telemetry**: 100% private local execution with direct outbound HTTPS calls exclusively to Apify's API.

---

## Installation

### Using Cargo

```bash
cargo install --git https://github.com/SpaceCorps/Reddit-Cli --locked
```

### Pre-built Standalone Binaries

Download standalone binary archives directly from the [GitHub Releases](https://github.com/SpaceCorps/Reddit-Cli/releases/latest) page:

| Platform | Architecture | Binary Package |
|:---|:---|:---|
| **macOS** | Apple Silicon (`aarch64`) | [`reddit-v1.0.0-aarch64-apple-darwin.tar.gz`](https://github.com/SpaceCorps/Reddit-Cli/releases/download/v1.0.0/reddit-v1.0.0-aarch64-apple-darwin.tar.gz) |
| **macOS** | Intel (`x86_64`) | [`reddit-v1.0.0-x86_64-apple-darwin.tar.gz`](https://github.com/SpaceCorps/Reddit-Cli/releases/download/v1.0.0/reddit-v1.0.0-x86_64-apple-darwin.tar.gz) |
| **Linux** | x86_64 (musl static) | [`reddit-v1.0.0-x86_64-unknown-linux-musl.tar.gz`](https://github.com/SpaceCorps/Reddit-Cli/releases/download/v1.0.0/reddit-v1.0.0-x86_64-unknown-linux-musl.tar.gz) |
| **Windows**| x64 (MSVC) | [`reddit-v1.0.0-x86_64-pc-windows-msvc.zip`](https://github.com/SpaceCorps/Reddit-Cli/releases/download/v1.0.0/reddit-v1.0.0-x86_64-pc-windows-msvc.zip) |

---

## Prerequisites & Authentication

Get your API token at [Apify Console Integrations](https://console.apify.com/account/integrations).

You can authenticate in any of the following ways:

```bash
# 1. Store securely in OS Keystore
reddit login

# 2. Environment variable
export APIFY_TOKEN="your-apify-token"

# 3. Direct command flag
reddit search "rust 2024" --api-key "your-apify-token"
```

---

## Usage

### Search

```bash
# Search for posts
reddit search "dotnet 10"

# Search within a community
reddit search "async await" --community csharp

# Search for comments
reddit search "performance" --comments

# Search for communities
reddit search "programming" --communities

# Search for users
reddit search "devops" --users

# Sort and filter
reddit search "rust vs go" --sort top --time month --max-items 20

# Output as JSON
reddit search "rust 2024" --json
```

### Scrape

```bash
# Scrape a post with comments
reddit scrape https://www.reddit.com/r/dotnet/comments/abc123/some_post/

# Scrape a community
reddit scrape https://www.reddit.com/r/csharp/ --max-posts 20

# Scrape a user page
reddit scrape https://www.reddit.com/user/someuser/

# Skip comments for faster results
reddit scrape https://www.reddit.com/r/programming/ --skip-comments

# Only posts after a date
reddit scrape https://www.reddit.com/r/dotnet/ --since 2025-01-01
```

### Account Management

```bash
# List configured accounts and verify token validity
reddit accounts list --check

# Test an account's token connectivity
reddit accounts test work

# Remove an account from OS Keystore
reddit accounts remove old-account --yes
```

### Agent Manual

```bash
# View embedded agent manual
reddit agent-readme

# Output as structured JSON
reddit agent-readme --json
```

---

## Options Reference

```text
--api-key <KEY>           Apify token (or set APIFY_TOKEN env var)
-a, --account <ACCOUNT>   Account name to use (see 'reddit accounts list')
--json                    Output JSON instead of YAML
--max-items <N>           Maximum items to return (default: 10)
--max-posts <N>           Maximum posts per page (default: 10)
--max-comments <N>        Maximum comments per post (default: 10)
--skip-comments           Skip comments on posts
--since <DATE>            Only posts after this date (YYYY-MM-DD)
--comments-since <DATE>   Only comments after this date (YYYY-MM-DD)
```

---

## License & Credits

- **License**: MIT License ([LICENSE](LICENSE))
- **Lineage**: Originally created as a .NET prototype (`Reddit.Console`) by [Niels Bosma](https://github.com/nielsbosma/Reddit.Console). Re-architected and maintained as a native Rust CLI by SpaceCorps.
