//! The `search` command implementation.

use std::io::IsTerminal;

use crate::account;
use crate::cli::SearchArgs;
use crate::client::{ProxyConfig, SearchInput};
use crate::error::Result;
use crate::output;

pub fn run(args: SearchArgs, account_name: Option<&str>, api_key: Option<&str>) -> Result<()> {
    let resolved = account::resolve(account_name, api_key)?;
    let client = resolved.client();

    let has_type_flag = args.comments || args.communities || args.users;
    let search_posts = !has_type_flag;

    if std::io::stderr().is_terminal() {
        eprintln!("Searching Reddit (this may take 30–60s)...");
    }

    let input = SearchInput {
        searches: Some(vec![args.query]),
        search_community_name: args.community,
        search_posts,
        search_comments: args.comments,
        search_communities: args.communities,
        search_users: args.users,
        sort: Some(args.sort),
        time: args.time,
        include_nsfw: args.nsfw,
        max_items: args.max_items,
        max_post_count: args.max_posts,
        max_comments: args.max_comments,
        skip_comments: args.skip_comments,
        post_date_limit: args.since,
        comment_date_limit: args.comments_since,
        proxy: ProxyConfig::default(),
    };

    let doc = client.search(&input)?;
    output::write(&doc);
    Ok(())
}
