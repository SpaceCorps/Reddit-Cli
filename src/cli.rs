//! The command line argument definitions.

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "reddit",
    version,
    about = "CLI for scraping Reddit posts, comments, communities, and users via Apify",
    after_help = "An LLM agent should start with: reddit agent-readme",
    propagate_version = true,
    disable_help_subcommand = true
)]
pub struct Cli {
    /// Print raw JSON instead of YAML, for scripting
    #[arg(long, global = true)]
    pub json: bool,

    /// Apify API token (or set APIFY_TOKEN env var)
    #[arg(long, global = true, value_name = "KEY")]
    pub api_key: Option<String>,

    /// Account to run against (see 'reddit accounts list')
    #[arg(short = 'a', long, global = true, value_name = "ACCOUNT")]
    pub account: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Search Reddit for posts, comments, communities, or users
    Search(SearchArgs),

    /// Scrape a Reddit URL (post, community, or user page)
    Scrape(ScrapeArgs),

    /// Log in with an Apify API token
    Login(LoginArgs),

    /// Manage configured accounts and their API keys
    #[command(subcommand)]
    Accounts(AccountsCommand),

    /// Print the operating manual for an LLM agent driving this CLI
    AgentReadme,
}

// -------------------------------------------------------------------------------------------------
// search

#[derive(Args, Clone, Debug)]
pub struct SearchArgs {
    /// Search query
    #[arg(value_name = "QUERY")]
    pub query: String,

    /// Limit search to a specific community (e.g. 'programming')
    #[arg(long, value_name = "NAME")]
    pub community: Option<String>,

    /// Search for comments instead of posts
    #[arg(long)]
    pub comments: bool,

    /// Search for communities
    #[arg(long)]
    pub communities: bool,

    /// Search for users
    #[arg(long)]
    pub users: bool,

    /// Sort by: relevance, hot, top, new, rising, comments
    #[arg(long, default_value = "new", value_name = "SORT")]
    pub sort: String,

    /// Filter by time: all, hour, day, week, month, year
    #[arg(long, value_name = "TIME")]
    pub time: Option<String>,

    /// Maximum items to return
    #[arg(long, default_value = "10", value_name = "N")]
    pub max_items: u32,

    /// Maximum posts per page
    #[arg(long, default_value = "10", value_name = "N")]
    pub max_posts: u32,

    /// Maximum comments per post
    #[arg(long, default_value = "10", value_name = "N")]
    pub max_comments: u32,

    /// Skip scraping comments on posts
    #[arg(long)]
    pub skip_comments: bool,

    /// Include NSFW content
    #[arg(long)]
    pub nsfw: bool,

    /// Only include posts after this date (e.g. 2025-01-01)
    #[arg(long, value_name = "DATE")]
    pub since: Option<String>,

    /// Only include comments after this date
    #[arg(long, value_name = "DATE")]
    pub comments_since: Option<String>,
}

// -------------------------------------------------------------------------------------------------
// scrape

#[derive(Args, Clone, Debug)]
pub struct ScrapeArgs {
    /// Reddit URL to scrape (post, community, or user page)
    #[arg(value_name = "URL")]
    pub url: String,

    /// Skip scraping comments
    #[arg(long)]
    pub skip_comments: bool,

    /// Skip scraping user posts when visiting user pages
    #[arg(long)]
    pub skip_user_posts: bool,

    /// Skip community info (still scrapes posts)
    #[arg(long)]
    pub skip_community: bool,

    /// Maximum items to save
    #[arg(long, default_value = "10", value_name = "N")]
    pub max_items: u32,

    /// Maximum posts per page
    #[arg(long, default_value = "10", value_name = "N")]
    pub max_posts: u32,

    /// Maximum comments per post
    #[arg(long, default_value = "10", value_name = "N")]
    pub max_comments: u32,

    /// Maximum community pages to scrape
    #[arg(long, default_value = "2", value_name = "N")]
    pub max_communities: u32,

    /// Maximum user pages to scrape
    #[arg(long, default_value = "2", value_name = "N")]
    pub max_users: u32,

    /// Only include posts after this date (e.g. 2025-01-01)
    #[arg(long, value_name = "DATE")]
    pub since: Option<String>,

    /// Only include comments after this date
    #[arg(long, value_name = "DATE")]
    pub comments_since: Option<String>,
}

// -------------------------------------------------------------------------------------------------
// login

#[derive(Args, Clone, Debug)]
pub struct LoginArgs {
    /// Account name to store (default: "default")
    #[arg(value_name = "NAME", default_value = "default")]
    pub name: String,

    /// Apify API token (prompted for securely if omitted)
    #[arg(long, value_name = "KEY", conflicts_with = "api_key_stdin")]
    pub api_key: Option<String>,

    /// Read the API key from stdin
    #[arg(long)]
    pub api_key_stdin: bool,

    /// Do not open the browser to the API keys page automatically
    #[arg(long)]
    pub no_browser: bool,

    /// Replace the key on an account that already exists
    #[arg(long)]
    pub force: bool,

    /// Store the key without calling the API to check it first
    #[arg(long)]
    pub no_verify: bool,
}

// -------------------------------------------------------------------------------------------------
// accounts

#[derive(Subcommand, Clone, Debug)]
pub enum AccountsCommand {
    /// Add an account and store its Apify token in the OS keystore
    Add {
        /// Account name
        #[arg(value_name = "NAME")]
        name: String,

        /// Apify API token (prompted for securely if omitted)
        #[arg(long, value_name = "KEY", conflicts_with = "api_key_stdin")]
        api_key: Option<String>,

        /// Read the API key from stdin
        #[arg(long)]
        api_key_stdin: bool,

        /// Replace the key on an account that already exists
        #[arg(long)]
        force: bool,

        /// Store the key without calling the API to check it first
        #[arg(long)]
        no_verify: bool,
    },

    /// List configured accounts and report keystore status
    List {
        /// Call the API to check that each account's stored key still works
        #[arg(long)]
        check: bool,
    },

    /// Test that an account's stored key is valid
    Test {
        /// Account name to test
        #[arg(value_name = "NAME")]
        name: String,
    },

    /// Remove an account and delete its key from the OS keystore
    Remove {
        /// Account name to remove
        #[arg(value_name = "NAME")]
        name: String,

        /// Do not prompt for confirmation
        #[arg(long)]
        yes: bool,
    },
}
