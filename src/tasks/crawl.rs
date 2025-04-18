use super::Task;
use anyhow::Result;
use async_trait::async_trait;
use clap::ArgMatches;

pub struct CrawlTask;

#[async_trait]
impl Task for CrawlTask {
    fn name(&self) -> &'static str {
        "crawl"
    }

    async fn run(&self, args: &ArgMatches) -> Result<()> {
        let url = args.get_one::<String>("url").unwrap();
        println!("Crawling {}", url);
        Ok(())
    }
}
