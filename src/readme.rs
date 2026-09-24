//! The manual an agent reads before its first call. Markdown by default so it can be pasted
//! into a system prompt; `--json` gives the same rules as structured data.

use crate::{obj, output};

pub fn print() {
    if output::json() {
        output::write(&obj! {
            "tool" => "reddit",
            "apiVersion" => API_VERSION,
            "rules" => RULES,
            "exitCodes" => obj! {
                "0" => "ok",
                "1" => "error - unclassified, report and stop",
                "2" => "network - retry once, then stop",
                "3" => "auth_required - stop, surface the remediation to a human",
                "4" => "not_found - do not retry",
                "5" => "rate_limited - back off before retrying",
                "6" => "invalid_input - fix the call",
                "7" => "no_account - run reddit accounts list",
            },
        });
        return;
    }
    println!("{README}");
}

pub const API_VERSION: &str = "1.0.0";

const RULES: &[&str] = &[
    "Set APIFY_TOKEN or use --api-key / --account <name> for authentication.",
    "Run 'reddit accounts list' if multiple accounts exist to inspect configured credentials.",
    "On code auth_required, stop and surface the remediation string. Do not retry without valid credentials.",
    "Reddit scraping is powered by Apify actor trudax/reddit-scraper-lite and can take 15-60 seconds.",
    "Use --json when you are going to parse or pipe the output into jq.",
    "Default output format is clean YAML for terminal readability.",
];

const README: &str = r#"# reddit - agent operating manual

CLI for scraping Reddit posts, comments, communities, and users via Apify.
Results are YAML on stdout by default, errors are structured envelopes on stderr,
and `--json` switches both to JSON.

## Authentication

An Apify API token is required. You can provide it in any of the following ways:
1. Environment variable: `export APIFY_TOKEN=your-token-here` (or `REDDIT_API_KEY`)
2. Flag: `--api-key <KEY>`
3. OS Keystore / Multi-Account:
   `reddit login [<name>] [--api-key <key>]`
   `reddit accounts add <name> --api-key <key>`
   Then reference it with `--account <name>` (short `-a <name>`).

## Commands

### Search Reddit
Search Reddit for posts, comments, communities, or users:

    reddit search "rust vs go"
    reddit search "async await" --community rust
    reddit search "performance" --comments
    reddit search "programming" --communities
    reddit search "devops" --users
    reddit search "database" --sort top --time month --max-items 20

Options:
    --community <NAME>     Limit search to a specific community (e.g. 'rust')
    --comments             Search for comments instead of posts
    --communities          Search for communities
    --users                Search for users
    --sort <SORT>          Sort by: relevance, hot, top, new, rising, comments (default: new)
    --time <TIME>          Filter by time: all, hour, day, week, month, year
    --max-items <N>        Maximum items to return (default: 10)
    --max-posts <N>        Maximum posts per page (default: 10)
    --max-comments <N>     Maximum comments per post (default: 10)
    --skip-comments        Skip scraping comments on posts
    --nsfw                 Include NSFW content
    --since <DATE>         Only include posts after this date (e.g. 2025-01-01)
    --comments-since <DATE> Only include comments after this date

### Scrape Reddit URL
Scrape a Reddit post with comments, a subreddit community, or a user page:

    reddit scrape https://www.reddit.com/r/rust/comments/abc123/some_post/
    reddit scrape https://www.reddit.com/r/rust/ --max-posts 20
    reddit scrape https://www.reddit.com/user/someuser/
    reddit scrape https://www.reddit.com/r/programming/ --skip-comments
    reddit scrape https://www.reddit.com/r/rust/ --since 2025-01-01

Options:
    --skip-comments        Skip scraping comments
    --skip-user-posts      Skip scraping user posts when visiting user pages
    --skip-community       Skip community info (still scrapes posts)
    --max-items <N>        Maximum items to save (default: 10)
    --max-posts <N>        Maximum posts per page (default: 10)
    --max-comments <N>     Maximum comments per post (default: 10)
    --max-communities <N>  Maximum community pages to scrape (default: 2)
    --max-users <N>        Maximum user pages to scrape (default: 2)
    --since <DATE>         Only include posts after this date (e.g. 2025-01-01)
    --comments-since <DATE> Only include comments after this date

### Account Management
Manage multiple Apify accounts stored securely in native OS keystores (macOS Keychain,
Windows DPAPI, Linux Secret Service):

    reddit login [name] [--api-key <key>]
    reddit accounts add <name> --api-key <key> [--force]
    reddit accounts list [--check]
    reddit accounts test <name>
    reddit accounts remove <name> [--yes]

## Exit Codes

| Code | Name | Meaning |
|---|---|---|
| 0 | `ok` | Command succeeded |
| 1 | `error` | General failure |
| 2 | `network` | Network timeout or Apify connection error |
| 3 | `auth_required` | Missing or invalid API token |
| 4 | `not_found` | Resource not found |
| 5 | `rate_limited` | Apify rate limit encountered |
| 6 | `invalid_input` | Missing or invalid arguments |
| 7 | `no_account` | Requested account not found or ambiguous |
"#;
