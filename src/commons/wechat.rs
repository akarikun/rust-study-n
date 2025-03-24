// use reqwest::blocking::Client;

use std::{fs, io::Write};

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

use super::unitily::CONFIG as config;
const WECHAT_TOKEN_FILE_PATH: &str = "wechat_token.json"; // token需要经常变动，这里用新的配置

//https://developers.weixin.qq.com/doc/offiaccount/Getting_Started/Overview.html

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WechatMPToken {
    expires_ts: u64,
    access_token: String,
}
impl WechatMPToken {
    fn save(expires_ts: u64, access_token: String) -> std::io::Result<()> {
        let mut file = match fs::File::open(WECHAT_TOKEN_FILE_PATH) {
            Ok(m) => m,
            Err(_) => {
                if let Ok(new_file) = fs::File::create(WECHAT_TOKEN_FILE_PATH) {
                    new_file
                } else {
                    panic!("Failed to create wechat.json");
                }
            }
        };
        let json_str = serde_json::to_string_pretty(&Self {
            expires_ts,
            access_token,
        })?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }
    pub fn load() -> std::io::Result<Self> {
        if !fs::exists(WECHAT_TOKEN_FILE_PATH)? {
            Self::save(0, format!(""))?;
        }
        let json_str = fs::read_to_string(WECHAT_TOKEN_FILE_PATH)?;
        let token: Self = serde_json::from_str(&json_str)?;
        Ok(token)
    }
    pub async fn get_token(&self) -> std::io::Result<String> {
        let mp_config = config.clone().mp.unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        if now > self.expires_ts as u128 {
            let url = format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
            mp_config.appid, mp_config.appSecret
            );
            let url = "https://google.com";
            if cfg!(debug_assertions) {
                std::env::set_var("ALL_PROXY", "socks5://127.0.0.1:32123");
                // panic!("");
            }
            let client = Client::new();
            if let Ok(response) = client.get(url).send().await {
                if let Ok(text) = response.text().await {
                    println!("{text}");
                    let token: Self = serde_json::from_str(&text)?;
                    Self::save(token.expires_ts, token.access_token)?;
                    return Ok(self.access_token.clone());
                }
            };
            panic!("wechat token获取异常");
        }
        Ok(self.access_token.clone())
    }
    pub async fn get_redirect_login_url(
        &self,
        redirect_uri: String,
        state: String,
    ) -> std::io::Result<String> {
        let mp_config = config.clone().mp.unwrap();

        let scope = "snsapi_userinfo";
        let url = format!("https://open.weixin.qq.com/connect/oauth2/authorize?appid={}&redirect_uri={}&response_type=code&scope={}&state={}#wechat_redirect",mp_config.appid,format!("{}{}",mp_config.host,redirect_uri),scope,state);
        dbg!(&url);
        Ok(url)
    }
    pub async fn user_login(&self, code: String) -> Result<(WechatMPUser, String), String> {
        let mp_config = config.clone().mp.unwrap();
        let url = format!("https://api.weixin.qq.com/sns/oauth2/access_token?appid={}&secret={}&code={}&grant_type=authorization_code",mp_config.appid,mp_config.appSecret,code);
        let client = Client::new();
        if let Ok(response) = client.get(url).send().await {
            let text = response.text().await.unwrap();
            println!("{text}");
            let user = serde_json::from_str(&text).expect("user_login 异常");
            return Ok((user, format!("{}", config.host)));
        };
        Err(format!("user_login 异常"))
    }
    pub async fn get_user(access_token: String, openid: String) -> Result<String, String> {
        let mp_config = config.mp.clone().unwrap();
        let url = format!(
            "https://api.weixin.qq.com/sns/userinfo?access_token=${access_token}&openid=${openid}"
        );
        let client = Client::new();
        if let Ok(response) = client.get(url).send().await {
            let text = response.text().await.unwrap();
            return Ok(text);
        };
        Err(format!("get_user 异常"))
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WechatMPUser {
    pub access_token: Option<String>,
    pub expires_in: Option<u64>,
    pub openid: Option<String>,
    pub refresh_token: Option<String>,
}
