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
