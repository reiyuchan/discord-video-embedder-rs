# discord-video-embedder

A wrapper to help with discord video file embeds in rust.

the wrapper uses:
- [discord.nfp.is](https://discord.nfp.is/) for embed link generation
- [catbox.moe](https://catbox.moe/) for file upload

## Get Library
```sh
cargo add --git github.com/reiyuchan/discord-video-embedder-rs
```

## Example
```rs
use crate::help::help;
use dve;
use std::env;
use tokio;

mod help;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let app = dve::DiscordEmbedder::new(None); // init

    if args.len() == 3 {
        match args[1].as_str() {
            // uploading to catbox
            "upload" => match app.upload(&args[2]).await {
                Ok(r) => println!("{}", r),
                Err(e) => eprintln!("{}", e),
            },
            // fetching embed url
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
```


## License

- [Apache License 2.0](http://www.apache.org/licenses/LICENSE-2.0)

- [MIT](http://opensource.org/licenses/MIT)
