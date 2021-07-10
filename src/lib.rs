use tokio::runtime::Builder;

mod implementation;
pub mod interface;
pub mod streamfinder;
pub mod utils;

pub fn get_url(url: &str, extras: &str) -> String {
    let mut ret = String::new();
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async {
        if url.contains("live.bilibili.com") {
            let b = streamfinder::bilibili::Bilibili::new();
            match b.get_live(url).await {
                Ok(it) => {
                    ret.push_str(it["title"].as_str());
                    ret.push_str("-qlpsplit-");
                    ret.push_str(it["url"].as_str());
                }
                _ => {
                    ret.push_str("qlp_nostream");
                }
            };
        } else if url.contains("bilibili.com/") {
            let b = streamfinder::bilibili::Bilibili::new();
            match b.get_video(url, extras).await {
                Ok(it) => {
                    for v in it {
                        ret.push_str(v.as_str());
                        ret.push_str("-qlpsplit-");
                    }
                }
                _ => {
                    ret.push_str("qlp_nostream");
                }
            };
        } else if url.contains("douyu.com") {
            let b = streamfinder::douyu::Douyu::new();
            match b.get_live(url).await {
                Ok(it) => {
                    ret.push_str(it["title"].as_str());
                    ret.push_str("-qlpsplit-");
                    ret.push_str(it["url"].as_str());
                }
                _ => {
                    ret.push_str("qlp_nostream");
                }
            };
        } else if url.contains("huya.com") {
            let b = streamfinder::huya::Huya::new();
            match b.get_live(url).await {
                Ok(it) => {
                    ret.push_str(it["title"].as_str());
                    ret.push_str("-qlpsplit-");
                    ret.push_str(it["url"].as_str());
                }
                _ => {
                    ret.push_str("qlp_nostream");
                }
            };
        } else if url.contains("youtube.com/") {
            let b = streamfinder::youtube::Youtube::new();
            match b.get_live(url).await {
                Ok(it) => {
                    ret.push_str(it["title"].as_str());
                    ret.push_str("-qlpsplit-");
                    ret.push_str(it["url"].as_str());
                }
                _ => {
                    ret.push_str("qlp_nostream");
                }
            };
        } else if url.contains("twitch.tv/") {
            let b = streamfinder::twitch::Twitch::new();
            match b.get_live(url).await {
                Ok(it) => {
                    ret.push_str(it["title"].as_str());
                    ret.push_str("-qlpsplit-");
                    ret.push_str(it["url"].as_str());
                }
                _ => {
                    ret.push_str("qlp_nostream");
                }
            };
        } else {
            ret.push_str("qlp_nostream");
        }
    });
    ret
}
