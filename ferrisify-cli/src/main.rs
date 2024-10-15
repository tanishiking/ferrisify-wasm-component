use clap::Parser;

mod bindings;

use bindings::tanishiking::ferris_says::ferris_says::say;

#[derive(Parser, Debug)]
pub struct Cli {
    pub content: String,
    pub width: u32,
}

pub fn main() {
    let cli = Cli::parse();
    if let Err(e) = start(&cli) {
        println!("{:?}", e);
    }
}

fn start(cli: &Cli) -> anyhow::Result<()> {
    let result = say(&cli.content.as_str(), cli.width);
    println!("{}", result);
    Ok(())
}
