use anyhow::Result;
use clap::Parser;
use rust_hichat_api::commands::Cli;

#[tokio::main]
async fn main() -> Result<()> {

    // 解析并执行命令
    let cli = Cli::parse();
    cli.command.execute().await?;
    Ok(())
    
}
