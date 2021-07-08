use std::collections::HashMap;

pub struct Bilibili {
    api1: String,
    api2: String,
}

impl Bilibili {
    pub fn new() -> Self {
        Bilibili {
            api1: String::from(
                "https://api.live.bilibili.com/xlive/web-room/v2/index/getRoomPlayInfo",
            ),
            api2: String::from(
                "https://api.live.bilibili.com/xlive/web-room/v1/index/getInfoByRoom",
            ),
        }
    }

    pub async fn get_live(
        self,
        room_url: &str,
    ) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let rid = room_url.split("/").last().unwrap();
        let client = reqwest::Client::new();
        let mut ret = HashMap::new();
        let mut param1 = Vec::new();
        param1.push(("room_id", rid));
        param1.push(("no_playurl", "0"));
        param1.push(("mask", "1"));
        param1.push(("qn", "10000"));
        param1.push(("platform", "web"));
        param1.push(("protocol", "0,1"));
        param1.push(("format", "0,2"));
        param1.push(("codec", "0,1"));
        let resp = client
            .get(&self.api1)
            .header("User-Agent", crate::utils::gen_ua())
            .query(&param1)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        let j = resp
            .pointer("/data/playurl_info/playurl/stream/0/format/0/codec/0")
            .ok_or("cannot parse json")?;
        ret.insert(
            String::from("url"),
            format!(
                "{}{}{}",
                j.pointer("/url_info/0/host")
                    .ok_or("json err")?
                    .as_str()
                    .ok_or("cannot convert to string")?,
                j.pointer("/base_url")
                    .ok_or("json err")?
                    .as_str()
                    .ok_or("cannot convert to string")?,
                j.pointer("/url_info/0/extra")
                    .ok_or("json err")?
                    .as_str()
                    .ok_or("cannot convert to string")?
            ),
        );
        param1.clear();
        param1.push(("room_id", rid));
        let resp = client
            .get(&self.api2)
            .header("User-Agent", crate::utils::gen_ua())
            .query(&param1)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        ret.insert(
            String::from("title"),
            format!(
                "{} - {}",
                resp.pointer("/data/room_info/title")
                    .ok_or("json err")?
                    .as_str()
                    .ok_or("cannot convert to string")?,
                resp.pointer("/data/anchor_info/base_info/uname")
                    .ok_or("json err")?
                    .as_str()
                    .ok_or("cannot convert to string")?
            ),
        );
        Ok(ret)
    }
}
