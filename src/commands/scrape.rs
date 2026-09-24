//! The `scrape` command implementation.

use std::io::IsTerminal;

use crate::account;
use crate::cli::ScrapeArgs;
use crate::client::{ProxyConfig, ScrapeInput, StartUrl};
use crate::error::Result;
use crate::output;

pub fn run(args: ScrapeArgs, account_name: Option<&str>, api_key: Option<&str>) -> Result<()> {
    let resolved = account::resolve(account_name, api_key)?;
    let client = resolved.client();

    if std::io::stderr().is_terminal() {
        eprintln!("Scraping Reddit (this may take 30–60s)...");
    }

    let input = ScrapeInput {
        start_urls: vec![StartUrl { url: args.url }],
        skip_comments: args.skip_comments,
        skip_user_posts: args.skip_user_posts,
        skip_community: args.skip_community,
        max_items: args.max_items,
        max_post_count: args.max_posts,
        max_comments: args.max_comments,
        max_communities_count: args.max_communities,
        max_user_count: args.max_users,
        post_date_limit: args.since,
        comment_date_limit: args.comments_since,
        proxy: ProxyConfig::default(),
    };

    let doc = client.scrape(&input)?;
    output::write(&doc);
    Ok(())
}
