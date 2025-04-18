pub mod crawl;
pub mod csv;
pub mod random_name;

use anyhow::Result;
use async_trait::async_trait;
use clap::ArgMatches;

#[async_trait]
pub trait Task: Send + Sync {
    fn name(&self) -> &'static str;
    async fn run(&self, args: &ArgMatches) -> Result<()>;
}

pub fn all_tasks() -> Vec<Box<dyn Task>> {
    vec![Box::new(csv::CsvTask), Box::new(crawl::CrawlTask), Box::new(random_name::RandomNameTask)]
}
