use anyhow::Result;
use structopt::StructOpt;
use tikv_client::RawClient;

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
    //let client = RawClient::new(&opts.address)
    let mut readline = rustyline::Editor::<()>::new();

    loop {
        let line = readline.readline(">> ")?;
    }
}
