use std::{
    collections::{HashMap, LinkedList},
    sync::{Arc, Mutex},
};
use log::*;

use tokio::runtime::Builder;

#[test]
fn test_bilibili_video() {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let b = qliveplayer_lib::streamfinder::bilibili::Bilibili::new();
        match b.get_video("https://www.bilibili.com/bangumi/play/ep391743", "").await {
            Ok(it) => {
                println!("{:?}", it);
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        };
    });
}
#[test]
fn test_bilibili_live() {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let b = qliveplayer_lib::streamfinder::bilibili::Bilibili::new();
        println!(
            "{:?}",
            b.get_live("https://live.bilibili.com/3?a=1").await.unwrap()
        );
    });
}

#[test]
fn test_douyu_live() {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let b = qliveplayer_lib::streamfinder::douyu::Douyu::new();
        println!(
            "{:?}",
            b.get_live("https://www.douyu.com/9999").await.unwrap()
        );
    });
}

#[test]
fn test_huya_live() {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let b = qliveplayer_lib::streamfinder::huya::Huya::new();
        println!(
            "{:?}",
            b.get_live("https://www.huya.com/825801").await.unwrap()
        );
    });
}

#[test]
fn test_youtube_live() {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let b = qliveplayer_lib::streamfinder::youtube::Youtube::new();
        println!(
            "{:?}",
            // b.get_live("https://www.youtube.com/watch?v=yrPqE8xf5mM").await.unwrap()
            b.get_live("https://www.youtube.com/channel/UCkngxfPbmGyGl_RIq4FA3MQ").await.unwrap()
        );
    });
}

#[test]
fn test_twitch_live() {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let b = qliveplayer_lib::streamfinder::twitch::Twitch::new();
        println!(
            "{:?}",
            b.get_live("https://www.twitch.tv/okcode").await.unwrap()
        );
    });
}

#[test]
fn test_bilibili_danmaku() {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let b = qliveplayer_lib::danmaku::bilibili::Bilibili::new();
        let dm_fifo = Arc::new(Mutex::new(LinkedList::<HashMap<String, String>>::new()));
        let df1 = dm_fifo.clone();
        tokio::spawn(async move {
            match b.run("https://live.bilibili.com/734", df1).await {
                Ok(_) => {}
                Err(e) => {
                    println!("danmaku client error: {:?}", e);
                }
            };
        });
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            if let Ok(mut df) = dm_fifo.lock() {
                while let Some(d) = df.pop_front() {
                    if d.get("msg_type").unwrap_or(&"other".to_owned()).eq("danmaku") {
                        println!(
                            "{}[{}] {}",
                            d.get("color").unwrap_or(&"ffffff".to_owned()),
                            d.get("name").unwrap_or(&"unknown".to_owned()),
                            d.get("content").unwrap_or(&" ".to_owned()),
                        )
                    }
                }
            }
        }
    });
}

#[test]
fn test_douyu_danmaku() {
    env_logger::init();
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let b = qliveplayer_lib::danmaku::douyu::Douyu::new();
        let dm_fifo = Arc::new(Mutex::new(LinkedList::<HashMap<String, String>>::new()));
        let df1 = dm_fifo.clone();
        tokio::spawn(async move {
            match b.run("https://www.douyu.com/9999", df1).await {
                Ok(_) => {}
                Err(e) => {
                    println!("danmaku client error: {:?}", e);
                }
            };
        });
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            if let Ok(mut df) = dm_fifo.lock() {
                while let Some(d) = df.pop_front() {
                    if d.get("msg_type").unwrap_or(&"other".to_owned()).eq("danmaku") {
                        println!(
                            "{}[{}] {}",
                            d.get("color").unwrap_or(&"ffffff".to_owned()),
                            d.get("name").unwrap_or(&"unknown".to_owned()),
                            d.get("content").unwrap_or(&" ".to_owned()),
                        )
                    }
                }
            }
        }
    });
}

#[test]
fn test_huya_danmaku() {
    env_logger::init();
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let b = qliveplayer_lib::danmaku::huya::Huya::new();
        let dm_fifo = Arc::new(Mutex::new(LinkedList::<HashMap<String, String>>::new()));
        let df1 = dm_fifo.clone();
        tokio::spawn(async move {
            match b.run("https://www.huya.com/kaerlol", df1).await {
                Ok(_) => {}
                Err(e) => {
                    println!("danmaku client error: {:?}", e);
                }
            };
        });
        loop {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            if let Ok(mut df) = dm_fifo.lock() {
                while let Some(d) = df.pop_front() {
                    if d.get("msg_type").unwrap_or(&"other".to_owned()).eq("danmaku") {
                        println!(
                            "{}[{}] {}",
                            d.get("color").unwrap_or(&"ffffff".to_owned()),
                            d.get("name").unwrap_or(&"unknown".to_owned()),
                            d.get("content").unwrap_or(&" ".to_owned()),
                        )
                    }
                }
            }
        }
    });
}

#[test]
fn test_youtube_danmaku() {
    env_logger::init();
    info!("start test!");
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let b = qliveplayer_lib::danmaku::youtube::Youtube::new();
        let dm_fifo = Arc::new(Mutex::new(LinkedList::<HashMap<String, String>>::new()));
        let df1 = dm_fifo.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                if let Ok(mut df) = dm_fifo.lock() {
                    while let Some(d) = df.pop_front() {
                        if d.get("msg_type").unwrap_or(&"other".to_owned()).eq("danmaku") {
                            println!(
                                "{}[{}] {}",
                                d.get("color").unwrap_or(&"ffffff".to_owned()),
                                d.get("name").unwrap_or(&"unknown".to_owned()),
                                d.get("content").unwrap_or(&" ".to_owned()),
                            )
                        }
                    }
                }
            }
        });
        match b.run("https://www.youtube.com/channel/UChAnqc_AY5_I3Px5dig3X1Q", df1).await {
            Ok(_) => {}
            Err(e) => {
                println!("danmaku client error: {:?}", e);
            }
        };
    });
}
