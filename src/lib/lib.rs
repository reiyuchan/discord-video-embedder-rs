use anyhow::{Ok, Result, anyhow};
use reqwest;
use std::{collections::HashSet, path};
use tokio;
use tokio_util::io::ReaderStream;

const CATBOX_URL: &str = "https://catbox.moe/user/api.php";
const BASE_URL: &str = "https://discord.nfp.is/";
const UA: &str = "discord-video-embedder/v0.1.0";

pub struct DiscordEmbedder {
    client: reqwest::Client,
}

impl DiscordEmbedder {
    pub fn new(client: Option<reqwest::Client>) -> DiscordEmbedder {
        if let Some(client) = client {
            return DiscordEmbedder { client };
        }

        DiscordEmbedder {
            client: reqwest::Client::builder().user_agent(UA).build().unwrap(),
        }
    }

    pub async fn upload(&self, path: &str) -> Result<String> {
        let path = path::Path::new(path);

        let allowed = is_allowed(path);

        if !allowed {
            return Err(anyhow!("extension not allowed"));
        }

        let file =
            reqwest::Body::wrap_stream(ReaderStream::new(tokio::fs::File::open(path).await?));

        let part = reqwest::multipart::Part::stream(file)
            .file_name(path.file_name().unwrap().to_str().unwrap().to_string());
        let form = reqwest::multipart::Form::new()
            .text("reqtype", "fileupload")
            .part("fileToUpload", part);

        let res = self
            .client
            .post(CATBOX_URL)
            .multipart(form)
            .send()
            .await?
            .text()
            .await?;

        Ok(res)
    }

    pub async fn get_embed(&self, url: &str) -> Result<String> {
        let path = path::Path::new(url);

        let allowed = is_allowed(path);

        if !allowed {
            return Err(anyhow!("extension not allowed"));
        }

        let form = reqwest::multipart::Form::new().text("video", url.to_string());

        let res = self
            .client
            .post(BASE_URL)
            .multipart(form)
            .send()
            .await?
            .text()
            .await?;

        let regex = regex::Regex::new("<pre>(.*)</pre>")?;

        match regex.captures(&res) {
            Some(matches) if matches.len() > 1 => Ok(matches[1].to_string()),
            _ => Err(anyhow!("no match found due to request failure")),
        }
    }
}

fn is_allowed(path: &path::Path) -> bool {
    let filter: HashSet<&str> = ["mp4", "avi", "mov", "wmv", "flv", "webm"]
        .into_iter()
        .collect();

    if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
        if !filter.contains(ext) {
            return false;
        }
    }
    return true;
}
