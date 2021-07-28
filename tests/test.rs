use log::*;
use std::{
    collections::{HashMap, LinkedList},
    sync::{Arc, Mutex},
};
use tokio::runtime::Builder;

struct TestUrl {
    bilibili_live_url: String,
    bilibili_video_url: String,
    douyu_live_url: String,
    huya_live_url: String,
    youtube_live_url: String,
    twitch_live_url: String,
}
impl TestUrl {
    pub fn new() -> Self {
        TestUrl {
            bilibili_live_url: "https://live.bilibili.com/3".to_owned(),
            bilibili_video_url: "https://www.bilibili.com/bangumi/play/ep391743".to_owned(),
            douyu_live_url: "https://www.douyu.com/9999".to_owned(),
            huya_live_url: "https://www.huya.com/825801".to_owned(),
            youtube_live_url: "https://www.youtube.com/watch?v=uKACIDRlf9M".to_owned(),
            twitch_live_url: "https://www.twitch.tv/okcode".to_owned(),
        }
    }
}

#[test]
fn test_bilibili_video() {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let u = TestUrl::new();
        let b = qliveplayer_lib::streamfinder::bilibili::Bilibili::new();
        match b.get_video(u.bilibili_video_url.as_ref(), "").await {
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
        let u = TestUrl::new();
        let b = qliveplayer_lib::streamfinder::bilibili::Bilibili::new();
        println!(
            "{:?}",
            b.get_live(u.bilibili_live_url.as_ref()).await.unwrap()
        );
    });
}

#[test]
fn test_douyu_live() {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let u = TestUrl::new();
        let b = qliveplayer_lib::streamfinder::douyu::Douyu::new();
        println!(
            "{:?}",
            b.get_live(u.douyu_live_url.as_ref()).await.unwrap()
        );
    });
}

#[test]
fn test_huya_live() {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let u = TestUrl::new();
        let b = qliveplayer_lib::streamfinder::huya::Huya::new();
        println!(
            "{:?}",
            b.get_live(u.huya_live_url.as_ref()).await.unwrap()
        );
    });
}

#[test]
fn test_youtube_live() {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let u = TestUrl::new();
        let b = qliveplayer_lib::streamfinder::youtube::Youtube::new();
        println!(
            "{:?}",
            b.get_live(u.youtube_live_url.as_ref()).await.unwrap()
        );
    });
}

#[test]
fn test_twitch_live() {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let u = TestUrl::new();
        let b = qliveplayer_lib::streamfinder::twitch::Twitch::new();
        println!(
            "{:?}",
            b.get_live(u.twitch_live_url.as_ref()).await.unwrap()
        );
    });
}

#[test]
fn test_bilibili_danmaku() {
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let u = TestUrl::new();
        let b = qliveplayer_lib::danmaku::bilibili::Bilibili::new();
        let dm_fifo = Arc::new(Mutex::new(LinkedList::<HashMap<String, String>>::new()));
        let df1 = dm_fifo.clone();
        tokio::spawn(async move {
            match b.run(u.bilibili_live_url.as_ref(), df1).await {
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
        let u = TestUrl::new();
        let b = qliveplayer_lib::danmaku::douyu::Douyu::new();
        let dm_fifo = Arc::new(Mutex::new(LinkedList::<HashMap<String, String>>::new()));
        let df1 = dm_fifo.clone();
        tokio::spawn(async move {
            match b.run(u.douyu_live_url.as_ref(), df1).await {
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
        let u = TestUrl::new();
        let b = qliveplayer_lib::danmaku::huya::Huya::new();
        let dm_fifo = Arc::new(Mutex::new(LinkedList::<HashMap<String, String>>::new()));
        let df1 = dm_fifo.clone();
        tokio::spawn(async move {
            match b.run(u.huya_live_url.as_ref(), df1).await {
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
        let u = TestUrl::new();
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
        match b
            .run(
                u.youtube_live_url.as_ref(),
                df1,
            )
            .await
        {
            Ok(_) => {}
            Err(e) => {
                println!("danmaku client error: {:?}", e);
            }
        };
    });
}

#[test]
fn test_twitch_danmaku() {
    env_logger::init();
    Builder::new_current_thread().enable_all().build().unwrap().block_on(async move {
        let u = TestUrl::new();
        let b = qliveplayer_lib::danmaku::twitch::Twitch::new();
        let dm_fifo = Arc::new(Mutex::new(LinkedList::<HashMap<String, String>>::new()));
        let df1 = dm_fifo.clone();
        tokio::spawn(async move {
            match b.run(u.twitch_live_url.as_ref(), df1).await {
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
