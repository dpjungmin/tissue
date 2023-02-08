use crate::{Args, Config};
use anyhow::Result;

#[derive(Debug)]
pub struct App {
    pub args: Args,
    pub config: Config,
}

impl App {
    pub fn new(args: Args, config: Config) -> Result<Self> {
        Ok(Self { args, config })
    }

    pub async fn run(self) -> Result<i32> {
        println!("Hello\rWorld\n");
        Ok(0)
    }
}
