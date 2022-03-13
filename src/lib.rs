use std::{
    collections::{HashMap, LinkedList},
    sync::{atomic::AtomicBool, Arc, Mutex},
};

use futures::pin_mut;
use tokio::{runtime::Builder, time::sleep};

#[macro_use]
extern crate structure;

pub mod danmaku;
mod implementation;
pub mod interface;
pub mod streamer;
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
                    ret.push_str("\n");
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
                        ret.push_str("\n");
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
                    ret.push_str("\n");
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
                    ret.push_str("\n");
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
                    ret.push_str("\n");
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
                    ret.push_str("\n");
                    ret.push_str(it["url"].as_str());
                }
                _ => {
                    ret.push_str("qlp_nostream");
                }
            };
        } else if url.contains("cc.163.com/") {
            let b = streamfinder::cc::CC::new();
            match b.get_live(url).await {
                Ok(it) => {
                    ret.push_str(it["title"].as_str());
                    ret.push_str("\n");
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

pub fn run_danmaku_client(url: &str, dm_fifo: Arc<Mutex<LinkedList<HashMap<String, String>>>>, stop_flag: Arc<AtomicBool>) {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let dmc = async move {
            loop {
                if url.contains("live.bilibili.com") {
                    let b = danmaku::bilibili::Bilibili::new();
                    match b.run(url, dm_fifo.clone()).await {
                        Ok(_) => {}
                        Err(e) => {
                            println!("danmaku client error: {:?}", e);
                        }
                    };
                } else if url.contains("douyu.com/") {
                    let b = danmaku::douyu::Douyu::new();
                    match b.run(url, dm_fifo.clone()).await {
                        Ok(_) => {}
                        Err(e) => {
                            println!("danmaku client error: {:?}", e);
                        }
                    };
                } else if url.contains("huya.com/") {
                    let b = danmaku::huya::Huya::new();
                    match b.run(url, dm_fifo.clone()).await {
                        Ok(_) => {}
                        Err(e) => {
                            println!("danmaku client error: {:?}", e);
                        }
                    };
                } else if url.contains("youtube.com/") {
                    let b = danmaku::youtube::Youtube::new();
                    match b.run(url, dm_fifo.clone()).await {
                        Ok(_) => {}
                        Err(e) => {
                            println!("danmaku client error: {:?}", e);
                        }
                    };
                } else if url.contains("twitch.tv/") {
                    let b = danmaku::twitch::Twitch::new();
                    match b.run(url, dm_fifo.clone()).await {
                        Ok(_) => {}
                        Err(e) => {
                            println!("danmaku client error: {:?}", e);
                        }
                    };
                }
                sleep(tokio::time::Duration::from_secs(1)).await;
            }
        };
        let check_stop = async move {
            loop {
                sleep(tokio::time::Duration::from_secs(1)).await;
                if stop_flag.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
            }
        };
        pin_mut!(dmc);
        pin_mut!(check_stop);
        let _ = futures::future::select(dmc, check_stop).await;
    });
}

pub fn run_streamer(streamer_type: String, url: String, extra: String, loading: Arc<AtomicBool>, stop_flag: Arc<AtomicBool>) {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let check_stop = async move {
            loop {
                sleep(tokio::time::Duration::from_secs(1)).await;
                if stop_flag.load(std::sync::atomic::Ordering::SeqCst) {
                    break;
                }
            }
        };
        let streamer_task = if streamer_type.eq("youtube") {
            async move {
                let yt = streamer::youtube::Youtube::new(url, extra, loading);
                yt.run().await;
            }
        } else {
            return;
        };
        pin_mut!(streamer_task);
        pin_mut!(check_stop);
        let _ = futures::future::select(streamer_task, check_stop).await;
    });
}
