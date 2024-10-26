use anyhow::{anyhow, Context};
use clap::Parser;
use log::trace;

use crate::{cli::MyCli, error::MyError};
/// Set up crate logging and environment variables.
pub(crate) fn setup() -> Result<MyCli, MyError> {
    dotenvy::dotenv().ok();
    let cli = MyCli::parse();
    env_logger::builder().filter_level(cli.log_level()).init();
    std::env::var("DOTENV_OK").with_context(|| anyhow!("failed to load dotenv"))?;
    Ok(cli)
}
