use chrono::Utc;
use task2::run;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long)]
    access_token: String,

    #[arg(long)]
    hashtag: String,

    #[arg(long)]
    hours: u32,
}

fn main() {
    let args = Args::parse();
    let result = tokio::runtime::Runtime::new().unwrap().block_on(
        run(&args.access_token, &args.hashtag, Utc::now(), args.hours)
    );
    match result {
        Ok(data) =>
            println!("{}", data.iter()
                .map(|value| { value.to_string() })
                .collect::<Vec<String>>()
                .join(", ")),
        Err(err) =>
            eprintln!("{}: {:#?}", err.to_string(), err.root_cause())
    }
}