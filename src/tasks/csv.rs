use super::Task;
use anyhow::Result;
use async_trait::async_trait;
use clap::ArgMatches;

pub struct CsvTask;

#[async_trait]
impl Task for CsvTask {
    fn name(&self) -> &'static str {
        "parse-csv"
    }

    async fn run(&self, args: &ArgMatches) -> Result<()> {
        let input = args.get_one::<String>("input").unwrap();
        let db_url = args.get_one::<String>("db_url").unwrap();
        println!("Parsing {} and uploading to {}", input, db_url);
        Ok(())
    }
}
