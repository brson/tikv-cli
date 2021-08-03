#![allow(unused)]

use anyhow::Result;
use structopt::StructOpt;
use tikv_client::RawClient;
use futures::executor::block_on;
use pest::Parser;

#[macro_use]
extern crate pest_derive;

#[derive(StructOpt)]
struct Options {
    #[structopt(default_value = "127.0.0.1:2379")]
    address: String,
}

fn main() -> Result<()> {
    let opts = Options::from_args();
    run(opts)
}

fn run(opts: Options) -> Result<()> {
    let client = RawClient::new(vec![&opts.address[..]]);
    let client = block_on(client)?;
    let mut readline = rustyline::Editor::<()>::new();

    loop {
        let line = readline.readline(">> ")?;
        let res = block_on(run_line(&client, &line));
        if let Err(e) = res {
            println!("error: {}", e);
        }
    }
}

async fn run_line(client: &RawClient, line: &str) -> Result<()> {
    let command = parse_raw_command(line)?;

    match command {
        RawCommand::Put { key, value } => {
            client.put(key, value).await?;
        },
        RawCommand::Get { key } => {
            let value = client.get(key).await?;
            match value {
                Some(value) => {
                    let value = String::from_utf8(value)?;
                    println!("{}", value);
                },
                None => {
                    println!("<none>");
                },
            }
        },
    }

    Ok(())
}

#[derive(Parser)]
#[grammar = "raw_parser.pest"]
struct RawParser;

enum RawCommand {
    Put {
        key: String,
        value: String,
    },
    Get {
        key: String,
    },
}

fn parse_raw_command(line: &str) -> Result<RawCommand> {
    let mut parser = RawParser::parse(Rule::command, line)?;
    let command = parser.next().expect("next");
    println!("{:?}", command);

    Ok(RawCommand::Put {
        key: "k".to_string(),
        value: "v".to_string(),
    })
}
