use std::{
    collections::{BTreeMap, LinkedList},
    convert::TryInto,
    sync::{atomic::AtomicBool, Arc},
};

use log::info;
use tokio::{
    io::AsyncWriteExt,
    net::{UnixListener, UnixStream},
};

pub struct HLS {
    url: String,
    stream_socket: String,
    // stream_port: u16,
    loading: Arc<AtomicBool>,
}

impl HLS {
    pub fn new(url: String, extra: String, loading: Arc<AtomicBool>) -> Self {
        HLS {
            url,
            stream_socket: extra,
            // stream_port: 11111,
            loading,
        }
    }

    async fn decode_m3u8(m3u8: &str) -> Result<BTreeMap<u64, String>, Box<dyn std::error::Error>> {
        info!("{}", &m3u8);
        let lines: Vec<_> = m3u8.split("\n").collect();
        let mut sq = None;
        let mut urls = LinkedList::new();
        let mut i = 0;
        while i < lines.len() {
            if lines[i].starts_with("#EXT-X-MEDIA-SEQUENCE") {
                let re = regex::Regex::new(r#"#EXT-X-MEDIA-SEQUENCE: *([0-9]+)"#).unwrap();
                let t: u64 = re.captures(&lines[i]).ok_or("decode m3u8 err 1")?[1].parse()?;
                sq = Some(t);
            }
            if !lines[i].starts_with("#") {
                urls.push_front(lines[i]);
            }
            i += 1;
        }
        if sq.is_none() || urls.is_empty() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "decode m3u8 failed",
            )));
        }
        let mut sq = sq.unwrap();
        let mut ret = BTreeMap::new();
        while !urls.is_empty() {
            ret.insert(sq, urls.pop_front().unwrap().to_string());
            sq = sq.saturating_sub(1);
        }
        Ok(ret)
    }

    async fn download(&self, mut stream: UnixStream) -> Result<(), Box<dyn std::error::Error>> {
        let mut sq = 0;
        let mut interval: u64 = 1000;
        let client = reqwest::Client::builder().user_agent(crate::utils::gen_ua()).timeout(tokio::time::Duration::from_secs(15)).build()?;
        loop {
            let now = std::time::Instant::now();
            let resp = client.get(&self.url).header("Connection", "keep-alive").send().await?.text().await?;
            let ts_urls = Self::decode_m3u8(&resp).await?;
            info!("m3u8: {:?}", &ts_urls);

            let mut head_sq = None;
            for (k, _) in ts_urls.iter() {
                head_sq = Some(k);
            }
            let head_sq: u64 = *head_sq.ok_or("hls download err 1")?;
            if sq == 0 {
                self.loading.store(false, std::sync::atomic::Ordering::SeqCst);
                sq = head_sq;
            }

            let ts_url = ts_urls.get(&sq).ok_or("hls download err 2")?;
            let mut resp = client.get(ts_url).header("Connection", "keep-alive").send().await?;
            while let Some(chunk) = resp.chunk().await? {
                stream.write_all(&chunk).await?;
            }

            if head_sq > sq {
                interval = interval.saturating_sub(100);
            } else if head_sq < sq {
                info!("hls: {}, {}, {}", &sq, &head_sq, &interval);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                interval += 100;
                continue;
            }
            if head_sq <= sq {
                let elapsed: u64 = now.elapsed().as_millis().try_into()?;
                if elapsed < interval {
                    let sleep_time = interval - elapsed;
                    // info!("v sleep: {}", &sleep_time);
                    tokio::time::sleep(tokio::time::Duration::from_millis(sleep_time)).await;
                }
            }
            sq += 1;
        }
    }

    pub async fn run(&self, arc_self: Arc<HLS>) {
        let stream = {
            let mut listener = None;
            for _ in 0..15 {
                match UnixListener::bind(&self.stream_socket) {
                    Ok(it) => {
                        listener = Some(it);
                        break;
                    }
                    Err(_) => {
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                        continue;
                    }
                };
            }
            match listener.unwrap().accept().await {
                Ok((stream, _addr)) => Some(stream),
                Err(_) => None,
            }
        };
        if stream.is_some() {
            let self1 = arc_self.clone();
            match self1.download(stream.unwrap()).await {
                Ok(it) => it,
                Err(err) => {
                    info!("hls download error: {:?}", err);
                }
            };
        }
        self.loading.store(false, std::sync::atomic::Ordering::SeqCst);
        info!("hls streamer exit");
    }
}
