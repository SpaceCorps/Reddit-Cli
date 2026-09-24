# AGENTS.md

Notes for whoever extends or maintains this codebase next.

`reddit` is a native Rust CLI over the Apify Reddit scraper actor API, built to be driven by humans and autonomous LLM agents. It replaced the .NET tool `Reddit.Console` by Niels Bosma and maintains backward compatibility while introducing OS keystores, multi-account management, sub-3ms cold starts, and full agent discovery.

For the manual that an LLM agent reads before executing tasks, run `reddit agent-readme` — that text lives in `src/readme.rs` and is the primary discovery interface. This document is for humans and agents editing the Rust source code.

---

## 1. Development & Quality Commands

```bash
cargo build --release              # target/release/reddit
cargo test                         # unit tests + tests/cli.rs against mock TCP API
cargo clippy --all-targets --locked -- -D warnings
cargo fmt --check
cargo install --path . --locked    # install binary on PATH
```

### Environment Isolation in Tests
Use a temporary configuration directory so development tests never interfere with host credentials:

```bash
export REDDIT_CONFIG_DIR=$(mktemp -d) REDDIT_SECRET_STORE=plaintext REDDIT_ALLOW_PLAINTEXT_STORE=1
```

| Variable | Purpose |
|:---|:---|
| `REDDIT_CONFIG_DIR` | Overrides the configuration directory (default: OS app support) |
| `REDDIT_SECRET_STORE` | Forces a secret store backend: `dpapi`, `keychain`, `libsecret`, `plaintext` |
| `REDDIT_ALLOW_PLAINTEXT_STORE=1` | Permits unencrypted secrets fallback where no OS vault exists |
| `REDDIT_API_URL` | Overrides Apify API endpoint — how `tests/cli.rs` points to mock server |
| `APIFY_TOKEN` / `REDDIT_API_KEY` | Environment fallback API token |

---

## 2. Codebase Layout

```text
src/
  main.rs          Entry point, --json pre-scan, and clap error envelope formatting
  cli.rs           Clap derive hierarchy for commands, flags, and help strings
  client.rs        Blocking HTTP client (ureq + rustls), request models, ErrorCode mapping
  error.rs         ErrorCode enum and structured Error envelope {code, message, detail, remediation}
  output.rs        YAML by default (serde_norway), JSON with --json (serde_json), obj! macro
  account.rs       Multi-account resolution, me endpoint parsing, token resolution hierarchy
  config.rs        config.yaml metadata, atomic file writes, 0600 permissions, cross-process lock
  secrets.rs       macOS Keychain, Linux secret-tool, Windows DPAPI, plaintext fallback
  readme.rs        agent-readme embedded manual and rules
  commands/
    mod.rs         Command dispatcher
    search.rs      Reddit search command implementation
    scrape.rs      Reddit URL scraping command implementation
    login.rs       Interactive & stdin token login
    accounts.rs    Account management (add, list, test, remove)
tests/
  cli.rs           In-process TCP mock HTTP test suite verifying end-to-end functionality
```

---

## 3. Core Architectural Tenets

1. **Blocking HTTP over Tokio Runtime:** A CLI executes a few sequential or scoped parallel HTTP calls. Tokio would add 20–40 MB binary bloat and 10–20 ms startup time. `ureq 3.4` with `rustls` delivers instant 1–3 ms cold-starts.
2. **Keychain Security via `/usr/bin/security`:** On macOS, using `/usr/bin/security` avoids Keychain code-signing authorization prompts that otherwise occur whenever an unsigned debug binary is rebuilt.
3. **YAML Default, JSON with `--json`:** YAML provides high readability for human inspection in terminal logs; `--json` provides structured, key-order-preserved data for `jq` and agent tool loops.
4. **Stderr for Diagnostics:** Status messages (e.g. "Searching Reddit...") and errors are strictly written to stderr so that stdout remains pure, parseable YAML or JSON.
5. **Deterministic Error Codes:** Non-zero exits map to stable string identifiers in `ErrorCode` (`auth_required`, `not_found`, `rate_limited`, `invalid_input`, `no_account`, `network`, `error`).
