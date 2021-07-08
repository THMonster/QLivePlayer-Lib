use tokio::runtime::Builder;

#[test]
fn test_bilibili_live() {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let b = qliveplayer_lib::streamfinder::bilibili::Bilibili::new();
            println!(
                "{:?}",
                b.get_live("https://live.bilibili.com/3").await.unwrap()
            );
        });
}

#[test]
fn test_douyu_live() {
    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async move {
            let b = qliveplayer_lib::streamfinder::douyu::Douyu::new();
            println!(
                "{:?}",
                b.get_live("https://www.douyu.com/9999").await.unwrap()
            );
        });
}
