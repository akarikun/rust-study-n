use std::{fs, io::Write};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub host: String,
    pub db: String,
    pub dbtype: i32,
    pub dbg: bool,
    pub mp: Option<WechatMPConfig>,
}
impl Config {
    pub fn load() -> std::io::Result<Self> {
        let path = "config.json";
        let err_msg = &format!("<{path}>文件不存在或配置异常");
        if !fs::exists(path).expect(err_msg) {
            let mut file = fs::File::create(path).expect(err_msg);
            let json_str = serde_json::to_string_pretty(&Self {
                host: "127.0.0.1:3000".to_string(),
                db: "sqlite://data.db?mode=rwc".to_string(),
                dbtype: 0,//0:sqlite
                dbg: true,
                mp: Some(WechatMPConfig::default()),
            })?;
            file.write_all(json_str.as_bytes())?;
        }

        let text = fs::read_to_string(path).expect(err_msg);
        let cfg: Self = serde_json::from_str(&text).expect(err_msg);
        Ok(cfg)
    }
}

#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct WechatMPConfig {
    pub token: String,
    pub appid: String,
    pub encodingAESKey: String,
    pub checkSignature: bool,
    pub appSecret: String,
    pub host: String,
}
