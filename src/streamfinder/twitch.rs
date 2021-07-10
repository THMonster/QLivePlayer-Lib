use regex::Regex;
use std::collections::HashMap;
use url::Url;

pub struct Twitch {}

impl Twitch {
    pub fn new() -> Self {
        Twitch {}
    }

    pub async fn get_live(
        &self,
        room_url: &str,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let rid = Url::parse(room_url)?
            .path_segments()
            .ok_or("rid parse error 1")?
            .last()
            .ok_or("rid parse error 2")?
            .to_string();
        let client = reqwest::Client::new();
        let mut ret = HashMap::new();
        let resp = client
            .get(format!("https://m.twitch.tv/{}", &rid))
            .header("User-Agent", crate::utils::gen_ua())
            .header("Accept-Language", "en-US")
            .header("Referer", "https://m.twitch.tv/")
            .send()
            .await?
            .text()
            .await?;
        let re = Regex::new(r#""BroadcastSettings\}\|\{.+?":.+?"title":"(.+?)""#).unwrap();
        ret.insert(
            String::from("title"),
            format!(
                "{}",
                re.captures(&resp).ok_or("regex err 1")?[1].to_string()
            ),
        );
        let output = tokio::process::Command::new("streamlink")
            .arg(format!("https://twitch.tv/{}", &rid))
            .arg("best")
            .arg("--stream-url")
            .output();
        let output = output.await?;
        ret.insert(
            String::from("url"),
            format!("{}", std::str::from_utf8(&output.stdout)?.trim()),
        );
        Ok(ret)
    }
}
