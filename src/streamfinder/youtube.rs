use regex::Regex;
use std::collections::HashMap;

pub struct Youtube {}

impl Youtube {
    pub fn new() -> Self {
        Youtube {}
    }

    pub async fn get_live(
        &self,
        room_url: &str,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let vurl = if room_url.contains("youtube.com/channel/") {
            let re = Regex::new(r"youtube.com/channel/([^/?]+)").unwrap();
            let cid = re.captures(room_url).ok_or("regex err 1")?[1].to_string();
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
            let t = re.captures(&resp)?.ok_or("regex err 2")?.get(0).ok_or("regex err 2 2")?.as_str();
            let re = Regex::new(r#""gridVideoRenderer".+?"videoId":"(.+?)""#).unwrap();
            let vid = re.captures(t).ok_or("regex err 3")?[1].to_string();
            format!("https://www.youtube.com/watch?v={}", &vid)
        } else {
            room_url.to_string()
        };
        let mut ret = HashMap::new();
        let resp = client
            .get(&vurl)
            .header("User-Agent", crate::utils::gen_ua())
            .header("Accept-Language", "en-US")
            .header("Referer", "https://www.youtube.com/")
            .send()
            .await?
            .text()
            .await?;
        let re = Regex::new(r"<title>(.+?) . YouTube</title>").unwrap();
        ret.insert(
            String::from("title"),
            format!(
                "{}",
                re.captures(&resp).ok_or("regex err 4")?[1].to_string()
            ),
        );
        let re = Regex::new(r#""hlsManifestUrl":"([^"]+)""#).unwrap();
        let hls_manifest = re.captures(&resp).ok_or("regex err 5")?[1].to_string();
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
                re.captures(&resp).ok_or("regex err 6")?[1].to_string()
            ),
        );
        Ok(ret)
    }
}
