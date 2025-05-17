mod hello;

use clap::Args;
use crate::commands::hello::hello::hello_say;

// 参数
#[derive(Args)]
pub struct Hello {
    pub say: String,
}

impl Hello {
    pub async fn run(&self) -> anyhow::Result<()> {
        hello_say(&self.say).await
    }
}