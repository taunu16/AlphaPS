#![feature(try_blocks)]

use anyhow::Result;

mod logging;
mod net;
mod game;
mod excel;
#[macro_use]
mod util;

use logging::init_tracing;

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    net::gateway::listen("0.0.0.0", 23301).await?;

    Ok(())
}
