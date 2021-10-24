use std::{
    collections::{HashMap, LinkedList},
    sync::{atomic::AtomicBool, Arc, Mutex},
    thread::sleep,
    time::Duration,
};

use crate::interface::*;

pub struct QLivePlayerLib {
    emit: QLivePlayerLibEmitter,
    dm_fifo: Arc<Mutex<LinkedList<HashMap<String, String>>>>,
    stop_flag: Arc<AtomicBool>,
    is_streamer_loading: Arc<AtomicBool>,
}

impl QLivePlayerLibTrait for QLivePlayerLib {
    fn new(emit: QLivePlayerLibEmitter) -> QLivePlayerLib {
        let dm_fifo = Arc::new(Mutex::new(LinkedList::<HashMap<String, String>>::new()));
        QLivePlayerLib {
            emit,
            dm_fifo,
            stop_flag: Arc::new(AtomicBool::new(false)),
            is_streamer_loading: Arc::new(AtomicBool::new(true)),
        }
    }

    fn emit(&mut self) -> &mut QLivePlayerLibEmitter {
        &mut self.emit
    }

    fn get_danmaku(&mut self) -> String {
        let mut ret = String::new();
        if let Ok(mut df) = self.dm_fifo.lock() {
            while let Some(d) = df.pop_front() {
                if d.get("msg_type").unwrap_or(&"other".to_owned()).eq("danmaku") {
                    ret.push_str(
                        format!(
                            "{}[{}] {}\n",
                            d.get("color").unwrap_or(&"ffffff".to_owned()),
                            d.get("name").unwrap_or(&"unknown".to_owned()),
                            d.get("content").unwrap_or(&" ".to_owned()),
                        )
                        .as_str(),
                    );
                }
            }
        }
        ret
    }

    fn get_url(&self, room_url: String, extras: String) -> String {
        let _ = env_logger::try_init();
        crate::get_url(&room_url, &extras)
    }

    fn run_danmaku_client(&mut self, url: String) -> () {
        let _ = env_logger::try_init();
        crate::run_danmaku_client(&url, self.dm_fifo.clone(), self.stop_flag.clone());
        ()
    }

    fn stop_danmaku_client(&mut self) -> () {
        self.stop_flag.fetch_or(true, std::sync::atomic::Ordering::SeqCst);
        sleep(Duration::from_millis(1200));
    }

    fn check_streamer_loading(&self) -> () {
        while self.is_streamer_loading.load(std::sync::atomic::Ordering::SeqCst) {
            sleep(std::time::Duration::from_millis(100));
        }
    }

    fn run_streamer(&self, streamer_type: String, url: String, extra: String) -> () {
        let _ = env_logger::try_init();
        crate::run_streamer(
            streamer_type,
            url,
            extra,
            self.is_streamer_loading.clone(),
            self.stop_flag.clone(),
        );
    }
}
