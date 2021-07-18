use regex::Regex;
use std::collections::HashMap;

pub struct Youtube {}

impl Youtube {
    pub fn new() -> Self {
        Youtube {}
    }

    pub async fn get_live(&self, room_url: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let mut cid = "".to_owned();
        let mut vurl = if room_url.contains("youtube.com/channel/") {
            let re = Regex::new(r"youtube.com/channel/([^/?]+)").unwrap();
            cid.push_str(re.captures(room_url).ok_or("get_live err 1")?[1].to_string().as_str());
            format!("https://www.youtube.com/channel/{}/live", &cid)
        } else {
            room_url.to_string()
        };
        let mut ret = HashMap::new();
        let mut hls_manifest = "".to_owned();
        for _ in 0u8..=1u8 {
            let resp = client
                .get(&vurl)
                .header("User-Agent", crate::utils::gen_ua())
                .header("Accept-Language", "en-US")
                .header("Referer", "https://www.youtube.com/")
                .send()
                .await?
                .text()
                .await?;
            let c = || -> Result<serde_json::Value, Box<dyn std::error::Error>> {
                let re = Regex::new(r"ytInitialPlayerResponse\s*=\s*(\{.+?\});.*?</script>").unwrap();
                let j: serde_json::Value = serde_json::from_str(re.captures(&resp).ok_or("get_live err 4")?[1].to_string().as_ref())?;
                if j.pointer("/videoDetails/isLive").ok_or("get_live err 7")?.as_bool().ok_or("get_live err 7-2")? == false {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "no stream",
                    )));
                }
                Ok(j)
            };
            let j = if let Ok(it) = c() {
                it
            } else {
                if cid.is_empty() {
                    break;
                }
                let ch_url = format!("https://www.youtube.com/channel/{}/videos", &cid);
                let resp = client
                    .get(&ch_url)
                    .header("User-Agent", crate::utils::gen_ua())
                    .header("Accept-Language", "en-US")
                    .header("Referer", "https://www.youtube.com/")
                    .send()
                    .await?
                    .text()
                    .await?;
                let re = fancy_regex::Regex::new(r#""gridVideoRenderer"((.(?!"gridVideoRenderer"))(?!"style":"UPCOMING"))+"label":"(LIVE|LIVE NOW|PREMIERING NOW)"([\s\S](?!"style":"UPCOMING"))+?("gridVideoRenderer"|</script>)"#).unwrap();
                let t = re.captures(&resp)?.ok_or("get_live err 2")?.get(0).ok_or("get_live err 2 2")?.as_str();
                let re = Regex::new(r#""gridVideoRenderer".+?"videoId":"(.+?)""#).unwrap();
                let vid = re.captures(t).ok_or("get_live err 3")?[1].to_string();
                vurl.clear();
                vurl.push_str(format!("https://www.youtube.com/watch?v={}", &vid).as_str());
                continue;
            };
            hls_manifest.clear();
            hls_manifest.push_str(j.pointer("/streamingData/hlsManifestUrl").ok_or("get_live err 5")?.as_str().ok_or("get_live err 5-2")?);
            ret.insert(
                String::from("title"),
                j.pointer("/videoDetails/title").ok_or("get_live err 8")?.as_str().ok_or("get_live err 8-2")?.to_owned(),
            );
            break;
        }
        if hls_manifest.is_empty() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "no stream",
            )));
        }
        let resp = client
            .get(&hls_manifest)
            .header("User-Agent", crate::utils::gen_ua())
            .header("Accept-Language", "en-US")
            .header("Referer", "https://www.youtube.com/")
            .send()
            .await?
            .text()
            .await?;
        // println!("{}",&resp);
        let re = Regex::new(r#"[\s\S]+\n(http\S+?)\n"#).unwrap();
        ret.insert(
            String::from("url"),
            format!(
                "{}",
                re.captures(&resp).ok_or("get_live err 6")?[1].to_string()
            ),
        );
        Ok(ret)
    }
}
