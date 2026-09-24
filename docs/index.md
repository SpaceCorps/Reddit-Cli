# Reddit CLI

A blazing fast native command-line tool and agent interface for scraping Reddit posts, comments, communities, and user profiles via Apify. Built in Rust Edition 2024 for maximum performance, minimal cold-start latency, and seamless LLM agent integration.

- **Source Code**: [GitHub Repository](https://github.com/SpaceCorps/Reddit-Cli)
- **Documentation**: [Website](https://spacecorps.github.io/Reddit-Cli/)
- **License**: MIT
- **Agent Guide**: [llms.txt](https://spacecorps.github.io/Reddit-Cli/llms.txt)

---

## Features

- **Blazing Fast Cold Starts:** Standalone compiled native Rust binary with 1–3 ms startup time. Zero managed runtimes or Node/Python dependencies.
- **Agentic Output Protocol:** Default YAML output for clean human and terminal inspection; `--json` flag for jq and agent tool loops.
- **Secure OS Keystore Integration:** Stores API tokens in macOS Keychain, Windows DPAPI, or Linux Secret Service instead of plaintext files.
- **Full Search & Scrape Capabilities:** Query Reddit posts, comments, subreddits, and users; scrape discussions, comment hierarchies, communities, and user profiles.
- **Deterministic Multi-Account Support:** Manage multiple Apify tokens safely across personal, work, and automation environments.
- **Zero Telemetry:** 100% private local execution with direct outbound HTTPS calls exclusively to Apify's API.

---

## Quickstart

### Installation

```bash
# Via Cargo
cargo install --git https://github.com/SpaceCorps/Reddit-Cli.git --locked

# Or download precompiled binaries from GitHub Releases
```

### Authentication

```bash
# Interactive login (stores token in OS Keystore)
reddit login

# Or set environment variable
export APIFY_TOKEN="your-apify-token"
```

### Search Reddit

```bash
# Search for posts
reddit search "rust 2024"

# Search within a specific community
reddit search "async await" --community rust

# Search for comments
reddit search "performance" --comments

# Search for communities
reddit search "programming" --communities

# Sort and filter
reddit search "rust vs go" --sort top --time month --max-items 20
```

### Scrape Reddit URLs

```bash
# Scrape a post with comments
reddit scrape https://www.reddit.com/r/rust/comments/abc123/some_post/

# Scrape a community page
reddit scrape https://www.reddit.com/r/rust/ --max-posts 20

# Scrape without comments for faster retrieval
reddit scrape https://www.reddit.com/r/programming/ --skip-comments
```

---

## Trust & Resources

- [About the Project](https://spacecorps.github.io/Reddit-Cli/about.html)
- [Authentication Guide](https://spacecorps.github.io/Reddit-Cli/auth.md)
- [Pricing & Free Tier](https://spacecorps.github.io/Reddit-Cli/pricing.md)
- [Privacy Policy](https://spacecorps.github.io/Reddit-Cli/privacy.html)
- [Contact & Support](https://spacecorps.github.io/Reddit-Cli/contact.html)
