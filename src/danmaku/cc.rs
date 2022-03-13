// Danmuku for CC, Credit: https://github.com/wbt5/real-url/
use reqwest::Url;
use std::mem::size_of_val;

// use tokio::time::sleep;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use std::time::SystemTime;

pub struct CC {
    api1: String,
    channel_id: String,
    room_id: String,
    gametype: String,
}

impl CC {
    pub fn new() -> Self {
        CC {
            api1: "https://api.cc.163.com/v1/activitylives/anchor/lives".to_string(),
            channel_id: "".to_string(),
            room_id: "".to_string(),
            gametype: "".to_string(),
        }
    }

    fn get_reg(&self) -> Vec<u8> {
        let sid = 6144;
        let cid = 2;

        let update_req_info = json!({
            "22": 640u32,
            "23": 360u32,
            "24": "web",
            "25": "Linux",
            "29": "163_cc",
            "30": "",
            "31": "Mozilla/5.0 (Linux; Android 5.0; SM-G900P Build/LRX21T) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/72.0.3626.121 Mobile Safari/538.36",
        });
        let mac_add = "776ffa9d-b14d-49e0-adec-3b4b94cdc5b9".to_string() + "@web.cc.163.com";
        let device_token = mac_add.clone();

        let n: u64 = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(t) => t.as_secs(),
            _ => 0,
        };

        let data = json!({
            "web-cc": n * 1000,
            "macAdd": mac_add,
            "device_token": device_token,
            "page_uuid": "1268abe2-ff5c-4920-a775-acec9db304cd",
            "update_req_info": update_req_info,
            "system": "win",
            "memory": 1u32,
            "version": 1u32,
            "webccType": 4253u32
        });
        let mut reg_data : Vec<u8> =  Vec::new();
        let s = structure!("<HHI");
        reg_data = s.pack(sid, cid, 0).unwrap();
        // println!("reg_data before: {:?}", reg_data);

        reg_data.append(&mut self.encode_dict(data));

        // println!("reg_data: {:?}", reg_data);

        reg_data
    }

    fn get_beat(&self) -> Vec<u8> {
        let sid = 6144;
        let cid = 5;
        let data = json!({});
        let mut beat_data = structure!("<HHI").pack(sid, cid, 0).unwrap();
        beat_data.append(&mut self.encode_dict(data));
        beat_data
    }

    fn get_join(&self, data_cid: u32, data_gametype: u32, data_roomid: u32) -> Vec<u8> {
        let sid = 512;
        let cid = 1;
        let data = json!({
            "cid": data_cid,
            "gametype": data_gametype,
            "roomId": data_roomid
        });

        let mut join_data = structure!("<HHI").pack(sid, cid, 0).unwrap();
        join_data.append(&mut self.encode_dict(data));

        join_data
    }


    // FIXME: Might be wrong, however, we leave it for what it is
    fn encode_num(&self, mut e: i64) -> Vec<u8> {
        if e <= 255 {
            let mut t = structure!("!B").pack(e as u8).unwrap();
            return t;
        } 
        if e > 255 && e <= 65535 {
            let mut b = b"\xcd".to_vec();

            b.append(&mut structure!("!H").pack(e as u16).unwrap());
            return b;
        }

        // Else
        let mut t = Vec::new();
        let r = 9;
        let n = 0;
        let mut i = 52;
        let o = 8;
        let mut s = 8 * o - i - 1;
        let c = (1 << s) - 1;
        let h = c >> 1;
        let l: f64;
        if i == 23 {
            l = f64::powi(2.0, -24) - f64::powi(2.0, -77);
        } else {
            l = 0.0;
        }

        let mut d;
        if n != 0 {
            d = 0;
        } else {
            d = o - 1;
        }

        let p;
        if n != 0 {
            p = 1;
        } else {
            p = -1;
        }

        let y;
        if e < 0 || e == 0 && 1 / e != 0 {
            y = 1;
        } else {
            y = 0;
        }

        let mut f = 0;
        let mut u;
        let mut a: f64 = 0.0;
        while i >= 8 {
            e = i64::abs(e);
            f = f32::floor(f32::log2(e as f32)) as i32;
            u = f32::powi(2.0, -1 * f);

            if  (e as f64) * (u as f64) < 1.0 {
                f -= 1;
                u *= 2.0;
            }

            if f + h >= 1 {
                e += (1.0 / u) as i64;
            } else {
                e += (l * f64::powi(2.0, 1-h)) as i64;
            }

            if (e as f64)* (u as f64) >= 2.0 {
                f += 1;
                u /= 2.0;
            }

            if f + h >= c {
                a = 0.0;
                f = c;
            } else if f + h >= 1 {
                a = ((e as f64) * (u as f64) - 1.0) as f64 * f64::powi(2.0, i);
                f += h;
            } else {
                a = e as f64 * f64::powi(2.0, h - 1) * f64::powi(2.0, i);
                f = 0;
            }

            t.append(&mut (255 & a as u8).to_be_bytes().to_vec());
            d += p;
            a /= 256.0;
            i -= 8;
        }

        f = f << i | a as i32;
        s += i;
        while s > 0 {
            t.append(&mut (255 & f as u8).to_be_bytes().to_vec());
            d += p;
            f /= 256;
            s -= 8;
        }

        let tlen = t.len();
        t[tlen - 1] = t[tlen - 1] | (128 * y);

        t.reverse();

        let mut arr = b"\xcb".to_vec();
        arr.append(&mut t);

        arr
    }

    fn encode_str(&self, r: &str) -> Vec<u8> {
        // println!("Encode {}", r);
        let n = r.len();
        let i = 5 + 3 * n;
        let s;
        let f;

        if n < 32 {
            s = 1;
            f = 1;
        } else if n <= 255 {
            s = 2;
            f = 2;
        } else if n <= 65535 {
            s = 3;
            f = 3;
        } else {
            s = 5;
            f = 5;
        }

        let b;
        if s == 1 {
            b = 160 + n;
        } else if s <= 3 {
            b = 215 + s;
        } else {
            b = 219;
        }

        let mut e = Vec::new();
        if f == 1 {
            e = (b as u8).to_be_bytes().to_vec();
        } else {
            e.append(&mut (b as u8).to_be_bytes().to_vec());
            e.append(&mut (n as u8).to_be_bytes().to_vec());
        }

        e.append(&mut r.as_bytes().to_vec());

        e
    }

    fn encode_dict(&self, d: serde_json::Value) -> Vec<u8> {
        let n = d.as_object().unwrap().len();
        // println!("size_of_val: {}", n);
        let r : usize;
        if n < 16 {
            r = 128 + n;
        } else if n <= 65535 {
            r = 222;
        } else {
            r = 223;
        }

        let mut t = (r as u8).to_be_bytes().to_vec();

        for (key, value) in d.as_object().unwrap() {
            // println!("==> Process ({}, {})", key, value);
            // println!("<== Before KEY: {:?}", t);
            t.append(&mut self.encode_str(key));
            // println!("<== After KEY: {:?}", t);
            if value.is_number() {
                t.append(&mut self.encode_num(value.as_i64().unwrap()));
            } else if value.is_string() {
                t.append(&mut self.encode_str(value.as_str().unwrap()));
            } else if value.is_object() {
                t.append(&mut self.encode_dict(value.to_owned()));
            }
            // println!("<== After Value: {:?}", t);
        }

        t
    }

    fn r(t: u32, fmt: String) {
    }

    pub async fn get_ws_info(&mut self, url: &str) -> Result<(String, Vec<u8>), Box<dyn std::error::Error>> {
        let mut reg_data = Vec::new();
        let rid = Url::parse(url)?.path_segments().ok_or("rid parse error 1")?.last().ok_or("rid parse error 2")?.to_string();
        let client = reqwest::Client::new();
        let mut param1 = Vec::new();
        param1.push(("anchor_ccid", &rid));
        let resp =
            client.get(&self.api1).header("User-Agent", crate::utils::gen_ua()).query(&param1).send().await?.json::<serde_json::Value>().await?;
        println!("resp: {}", resp.to_string());
        let j = resp.pointer(&format!("/data/{}", rid)).ok_or("Cannot parse json 1")?;
        self.channel_id = j.pointer("/channel_id").ok_or("Cannot parse json 2")?.to_string();
        self.room_id = j.pointer("/room_id").ok_or("Cannot parse json 3")?.to_string();
        self.gametype = j.pointer("/gametype").ok_or("Cannot parse json 4")?.to_string();
        reg_data = self.get_reg();

        Ok(("done".to_string(), reg_data))
    }
}
