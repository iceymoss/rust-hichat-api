use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "icey-cli")]
#[command(about = "网络工具集", version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(name = "check", about = "ICMP连通性测试")]
    Ping(ping::PingCommand),
    
    #[command(name = "hello", about = "打招呼")]
    Hello(hello::Hello),
}

// 统一执行接口
impl Commands {
    pub async fn execute(&self) -> Result<()> {
        match self {
            Commands::Ping(cmd) => cmd.run().await,
            Commands::Hello(cmd) => {cmd.run().await}
        }
    }
}

pub mod ping;
pub mod hello;