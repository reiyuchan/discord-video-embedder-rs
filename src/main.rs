use crate::help::help;
use dve;
use std::env;
use tokio;

mod help;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let app = dve::DiscordEmbedder::new(None);

    if args.len() == 3 {
        match args[1].as_str() {
            "upload" => match app.upload(&args[2]).await {
                Ok(r) => println!("{}", r),
                Err(e) => eprintln!("{}", e),
            },
            "embed" => match app.get_embed(&args[2]).await {
                Ok(r) => println!("{}", r),
                Err(e) => eprintln!("{}", e),
            },
            _ => help(),
        }
    } else {
        help();
        return;
    }
}
