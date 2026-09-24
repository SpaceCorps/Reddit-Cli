//! Command dispatcher for the `reddit` CLI.

pub mod accounts;
pub mod login;
pub mod scrape;
pub mod search;

use crate::cli::{Cli, Command};
use crate::error::Result;
use crate::readme;

pub fn run(cli: Cli) -> Result<()> {
    let account = cli.account.as_deref();
    let api_key = cli.api_key.as_deref();

    match cli.command {
        Command::Search(args) => search::run(args, account, api_key),
        Command::Scrape(args) => scrape::run(args, account, api_key),
        Command::Login(args) => login::run(args),
        Command::Accounts(cmd) => accounts::run(cmd),
        Command::AgentReadme => {
            readme::print();
            Ok(())
        }
    }
}
