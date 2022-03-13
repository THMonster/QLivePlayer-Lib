use std::collections::HashMap;
use url::Url;

pub struct CC {
    api1: String,
    api2: String,
}

impl CC {
    pub fn new() -> Self {
        CC {
            api1: "https://api.cc.163.com/v1/activitylives/anchor/lives".to_owned(),
            api2: "https://cc.163.com/live/channel".to_owned(),
        }
    }

    pub async fn get_live(&self, room_url: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let rid = Url::parse(room_url)?.path_segments().ok_or("RID parse error 1")?.last().ok_or("rid parse error 2")?.to_string();
        let client = reqwest::Client::new();
        let mut ret = HashMap::new();
        let mut param1 = Vec::new();

        // Set first parameters
        param1.push(("anchor_ccid", &rid));

        let resp =
            client.get(&self.api1).header("User-Agent", crate::utils::gen_ua()).query(&param1).send().await?.json::<serde_json::Value>().await?;
        // println!("resp: {}", resp.to_string());
        let j = resp.pointer(&format!("/data/{}/channel_id", rid)).ok_or("Cannot parse json")?;
        let channel_id = j.to_string();
        // println!("channel id: {}", j.to_string());

        // Get the actual video address
        param1.clear();
        param1.push(("channelids", &channel_id));
        let resp =
            client.get(&self.api2).header("User-Agent", crate::utils::gen_ua()).query(&param1).send().await?.json::<serde_json::Value>().await?;
        println!("resp: {}", serde_json::to_string_pretty(&resp).unwrap());

        let j = resp.pointer("/data/0/sharefile").ok_or("Cannot parse json")?;
        let url = j.to_string();
        println!("Real url: {}", j.to_string());

        ret.insert(String::from("url"), url);

        // Title
        ret.insert(
            String::from("title"),
            format!(
                "{} - {}",
                resp.pointer("/data/0/title").ok_or("json err")?.as_str().ok_or("Cannot convert to string")?,
                resp.pointer("/data/0/nickname").ok_or("json err")?.as_str().ok_or("Cannot convert to string")?,
            ),
        );

        // Return value
        Ok(ret)
    }
}
