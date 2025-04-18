use anyhow::Result;
use clap::{Arg, Command};

mod tasks;

#[tokio::main]
async fn main() -> Result<()> {
    let task_list = tasks::all_tasks();
    let mut app = Command::new("taskmaster")
        .version("0.1")
        .about("Taskmaster CLI Tool")
        .subcommand_required(true)
        .arg_required_else_help(true);

    for task in &task_list {
        let sub = match task.name() {
            "parse-csv" => Command::new("parse-csv")
                .about("Parse a CSV and load to DB")
                .arg(
                    Arg::new("input")
                        .long("input")
                        .short('i')
                        .num_args(1)
                        .required(true)
                        .help("Path to input CSV"),
                )
                .arg(
                    Arg::new("db_url")
                        .long("db-url")
                        .short('d')
                        .num_args(1)
                        .required(true)
                        .help("Database connection string"),
                ),
            "crawl" => Command::new("crawl")
                .about("Crawl a website and download images")
                .arg(
                    Arg::new("url")
                        .long("url")
                        .short('u')
                        .num_args(1)
                        .required(true)
                        .help("URL to crawl"),
                ),
            "random-name" => Command::new("random-name")
                .about("Pick a random name from a file")
                .arg(Arg::new("file")
                    .long("file")
                    .short('f')
                    .num_args(1)
                    .required(true)
                    .help("File with one name per line")),
            _ => continue,
        };
        app = app.subcommand(sub);
    }

    let matches = app.get_matches();
    if let Some((name, sub_matches)) = matches.subcommand() {
        for task in &task_list {
            if task.name() == name {
                return task.run(sub_matches).await;
            }
        }
    }

    Ok(())
}
