use anyhow::Result;
use structopt::StructOpt;
use tikv_client::RawClient;
use futures::executor::block_on;

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
    println!("writing to database");
    client.put("key".to_owned(), "coconut".to_owned()).await?;

    let value = client.get("key".to_owned()).await?;
    let value = value.unwrap();
    let value = String::from_utf8(value)?;

    println!("value: {}", value);

    Ok(())
}
