# Reddit.Console

CLI for scraping Reddit posts, comments, communities, and users via Apify. YAML-first output optimized for LLM agent consumption.

## Installation

```bash
dotnet tool install -g Reddit.Console
```

## Prerequisites

Set your Apify API token:

```bash
export APIFY_TOKEN=your-token-here
```

Or pass it per-command with `--api-key`.

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

### Common options

```bash
--api-key <KEY>       Apify token (or set APIFY_TOKEN env var)
--max-items <N>       Maximum items to return (default: 10)
--max-posts <N>       Maximum posts per page (default: 10)
--max-comments <N>    Maximum comments per post (default: 10)
--since <DATE>        Only posts after this date
--comments-since      Only comments after this date
```

## License

MIT
