use diff_engine::*;

use clap::Parser;
use colored::Colorize;
use glob::glob;
use reqwest::{blocking::Client, header::CONTENT_TYPE};
use serde_json::{to_string, Value};
use similar::TextDiff;
use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Compare this variant...
    variant_a: String,

    /// ...to this variant.
    variant_b: String,

    #[arg(long)]
    config: Option<PathBuf>,
}

fn fetch(
    target: &str,
    query: &String,
    headers: &HashMap<String, String>,
    args: &QueryArgs,
) -> String {
    let client = Client::new();

    let mut req = client
        .post(target)
        .header(CONTENT_TYPE, "application/json")
        .body(format!(
            "{{ \"query\": \"{}\", \"variables\": {} }}",
            &query.escape_default(),
            to_string(&args.0).unwrap()
        ));

    for (header, value) in headers.into_iter() {
        req = req.header(header, value);
    }

    req.send().unwrap().text().unwrap()
}

fn run_diff(config: &Config, target_a: &str, target_b: &str) {
    for file in glob(&config.query_path).unwrap() {
        let file = file.unwrap();
        let query = read_to_string(&file).unwrap();
        let query_name = file.file_stem().unwrap().to_str().unwrap();

        let args = match config.args.0.get(query_name) {
            Some(args) => arg_product(args),
            None => vec![QueryArgs(HashMap::new())],
        };

        let filename = file.to_str().unwrap();
        let border_thin: String = filename.chars().into_iter().map(|_| '-').collect();
        let border_thik: String = filename.chars().into_iter().map(|_| '=').collect();

        println!("{border_thik}=");
        println!("{}", filename.blue());

        for arg_set in args {
            println!("{border_thin}-");

            let response_a = fetch(target_a, &query, &config.headers, &arg_set);
            let response_b = fetch(target_b, &query, &config.headers, &arg_set);

            let response_a =
                serde_json::to_string_pretty(&serde_json::from_str::<Value>(&response_a).unwrap())
                    .unwrap();

            let response_b =
                serde_json::to_string_pretty(&serde_json::from_str::<Value>(&response_b).unwrap())
                    .unwrap();

            let diff = TextDiff::from_lines(&response_a, &response_b);

            for arg in arg_set.0.into_iter() {
                println!("{}: {}", arg.0.yellow(), arg.1);
            }

            println!("{border_thin}-");

            let udiff = diff.unified_diff();
            let udiff = udiff.to_string();

            if udiff.len() == 0 {
                println!("{}", udiff);
            } else {
                println!("No Change");
            }
        }

        println!("{border_thik}=");
    }
}

fn main() {
    let cli = Cli::parse();

    let config = read_to_string(cli.config.unwrap_or("./graphql-diff.toml".into())).unwrap();
    let config: Config = toml::from_str(&config).unwrap();

    run_diff(&config, &cli.variant_a, &cli.variant_b);
}
