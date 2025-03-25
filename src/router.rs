use std::collections::BTreeSet;

use crate::commons::model::{
    SIO_GetIndexReq, SIO_GetStudyReq, SIO_PostStudyReq, SocketIO_Req, SocketIO_Resp,
};
use crate::commons::unitily::{log_print, string_default_val, CONFIG as config};
use crate::commons::wechat;
use crate::dal::study::{self};
use crate::entities;
use salvo::http::StatusError;
use salvo::jwt_auth::{ConstDecoder, HeaderFinder};
use salvo::prelude::*;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha1::{Digest, Sha1};
use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

const SECRET_KEY: &str = "YOUR_SECRET_KEY";

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaims {
    sid: String,
    sub: String,
    exp: i64,
}

#[handler]
fn login() {}

#[handler]
async fn check(depot: &mut Depot) -> Result<String, StatusError> {
    match depot.jwt_auth_state() {
        JwtAuthState::Authorized => {
            let data = depot.jwt_auth_data::<JwtClaims>().unwrap();
            Ok(format!(
                "Hi {}, have logged in successfully!",
                data.claims.sub
            ))
        }
        JwtAuthState::Unauthorized => Err(StatusError::unauthorized()),
        _ => Err(StatusError::forbidden()),
    }
}

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

async fn get_last_index(level: i32) -> i64 {
    match study::get_last_index(level).await {
        Ok(m) => m,
        Err(_) => 0,
    }
}

async fn io_study(s: &SocketRef, data: &Value) {
    log_print(format!("io_study {:?}", data));

    let study_msg_resp = "study_msg_resp";
    let m = match serde_json::from_value::<SocketIO_Req>(data.clone()) {
        Ok(m) => m,
        Err(_) => return,
    };
    let msg_resp = format!("{}_resp", m.msg);
    if m.msg == "get_last_index" {
        if let Some(data) = m.data {
            let data = match serde_json::from_value::<SIO_GetIndexReq>(data) {
                Ok(m) => m,
                Err(_) => return,
            };
            let id = get_last_index(data.level).await;
            _ = s.emit(
                study_msg_resp,
                SocketIO_Resp::<i64> {
                    status: 1,
                    msg: msg_resp,
                    data: Some(id),
                },
            );
            log_print(format!("发送成功"));
        }
    } else if m.msg == "get_study_list" {
        if let Some(data) = m.data {
            let data = match serde_json::from_value::<SIO_GetIndexReq>(data) {
                Ok(m) => m,
                Err(_) => return,
            };
            let step = data.step.or(Some(0)).unwrap();
            let index = data.index.or(Some(0)).unwrap();
            match study::get_list(index, data.level, step).await {
                Ok(m) => {
                    let vec = match serde_json::from_value::<Vec<SIO_PostStudyReq>>(
                        serde_json::Value::Array(m),
                    ) {
                        Ok(m) => m,
                        Err(e) => panic!("{:?}", e),
                    };
                    _ = s.emit(
                        study_msg_resp,
                        SocketIO_Resp::<Vec<SIO_PostStudyReq>> {
                            status: 1,
                            msg: msg_resp,
                            data: Some(vec),
                        },
                    );
                }
                Err(e) => {
                    log_print(format!("{}", e));
                    _ = s.emit(
                        study_msg_resp,
                        SocketIO_Resp::<Vec<SIO_PostStudyReq>> {
                            status: 0,
                            msg: msg_resp,
                            data: Some(Vec::new()),
                        },
                    );
                }
            };
        }
    } else if m.msg == "get_study" {
        if let Some(data) = m.data {
            let data = match serde_json::from_value::<SIO_GetStudyReq>(data) {
                Ok(m) => m,
                Err(e) => {
                    // panic!("{:?}", e);
                    return;
                }
            };
            match study::get_list(data.index, data.level, 0).await {
                Ok(m) => {
                    if m.len() > 0 {
                        let t = m[0].clone();
                        dbg!(&t);
                        let m = serde_json::from_value::<SIO_PostStudyReq>(t).unwrap();
                        _ = s.emit(
                            study_msg_resp,
                            SocketIO_Resp::<SIO_PostStudyReq> {
                                status: 1,
                                msg: msg_resp,
                                data: Some(m),
                            },
                        );
                        return;
                    }
                }
                Err(e) => {
                    _ = s.emit(
                        study_msg_resp,
                        SocketIO_Resp::<String> {
                            status: 0,
                            msg: msg_resp,
                            data: Some(format!("{:?}", e.sql_err())),
                        },
                    );
                }
            };
        }
    } else if m.msg == "post_study" {
        if !token_valid(&data, "test") {
            _ = s.emit(
                study_msg_resp,
                SocketIO_Resp::<&str> {
                    status: 0,
                    msg: msg_resp,
                    data: Some("没有操作权限"),
                },
            );
            return;
        }

        if let Some(data) = m.data {
            let data = match serde_json::from_value::<SIO_PostStudyReq>(data) {
                Ok(m) => m,
                Err(e) => panic!("{:?}", e),
            };

            let string_checked = |str| {
                let (_, b) = string_default_val(str, "");
                b
            };

            let mut err_msg = "";
            if string_checked(&data.content) {
                err_msg = "题目内容不能为空";
            } else if string_checked(&data.a) {
                err_msg = "答案A内容不能为空";
            } else if string_checked(&data.b) {
                err_msg = "答案B内容不能为空";
            } else if string_checked(&data.c) {
                err_msg = "答案C内容不能为空";
            } else if string_checked(&data.d) {
                err_msg = "答案D内容不能为空";
            }
            if !err_msg.is_empty() {
                _ = s.emit(
                    study_msg_resp,
                    SocketIO_Resp::<&str> {
                        status: 0,
                        msg: msg_resp,
                        data: Some(err_msg),
                    },
                );
                return;
            }

            let mut m = entities::study::Model {
                id: data.id,
                level: data.level,
                index: data.index,
                content: data.content,
                a: data.a,
                b: data.b,
                c: data.c,
                d: data.d,
                remark: data.remark,
                result: data.result,
                r#type: data.r#type,
                create_date: Local::now().naive_local(),
            };
            //添加
            if data.id == 0 {
                let id = get_last_index(data.level).await as i32;
                m.id = id + 1; //SeaORM生成的实体类漏了设置自动增长，这里先手动处理
                match study::insert(m).await {
                    Ok(res) => {
                        _ = s.emit(
                            study_msg_resp,
                            SocketIO_Resp::<i32> {
                                status: 1,
                                msg: msg_resp,
                                data: Some(res.id),
                            },
                        );
                    }
                    Err(e) => {
                        _ = s.emit(
                            study_msg_resp,
                            SocketIO_Resp::<String> {
                                status: 0,
                                msg: msg_resp,
                                data: Some(format!("{:?}", e.sql_err())),
                            },
                        );
                    }
                };
            } else {
                match study::update(m).await {
                    Ok(res) => {
                        dbg!(&res);
                        _ = s.emit(
                            study_msg_resp,
                            SocketIO_Resp::<i32> {
                                status: 1,
                                msg: msg_resp,
                                data: Some(res.id),
                            },
                        );
                    }
                    Err(e) => {
                        dbg!(&e);
                        _ = s.emit(
                            study_msg_resp,
                            SocketIO_Resp::<String> {
                                status: 0,
                                msg: msg_resp,
                                data: Some(format!("{:?}", e.sql_err())),
                            },
                        );
                    }
                };
            }
        }
    }
}

fn token_valid(data: &Value, val: &str) -> bool {
    let token = data["token"].clone();
    token.is_string() && token == val && !val.is_empty()
}

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    log_print(format!(
        "Socket.IO connected: {:?} {:?} {:?}",
        socket.ns(),
        socket.id,
        data
    ));
    if token_valid(&data, "test") {
        socket.on(
            "study_msg",
            move |s: SocketRef, data: Data<Value>| async move {
                io_study(&s, &data.0).await;
            },
        );
    } else {
        _ = socket.disconnect();
    }
}

pub fn config_router() -> Router {
    let (layer, io) = SocketIo::new_layer();
    let layer = ServiceBuilder::new()
        .layer(CorsLayer::permissive())
        .layer(layer);

    io.ns("/ws", on_connect);
    let layer = layer.compat();

    let auth_handler: JwtAuth<JwtClaims, ConstDecoder> =
        JwtAuth::new(ConstDecoder::from_secret(SECRET_KEY.as_bytes()))
            .finders(vec![Box::new(HeaderFinder::new())])
            .force_passed(true);

    Router::new()
        .push(wechat_router())
        .push(Router::with_path("/login.do").post(login))
        .push(Router::with_hoop(auth_handler).path("check").get(check))
        .push(Router::with_path("/socket.io").hoop(layer).goal(hello))
}

fn wechat_router() -> Router {
    Router::new()
        .push(Router::with_path("/wechat/mp/").get(wechat_mp))
        .push(Router::with_path("/wechat/mp/login.do").get(wechat_mp_login))
        .push(Router::with_path("/wechat/mp/home.do").get(wechat_mp_home))
}

fn check_signature(signature: &str, timestamp: &str, nonce: &str) -> bool {
    let mut values = BTreeSet::new();
    values.insert(config.mp.clone().unwrap().token);
    values.insert(timestamp.to_owned());
    values.insert(nonce.to_owned());

    let concatenated = values.into_iter().collect::<String>();
    let hash = Sha1::digest(concatenated.as_bytes());

    format!("{:x}", hash) == signature
}

// https://mp.weixin.qq.com/debug/cgi-bin/sandboxinfo?action=showinfo&t=sandbox/index

// 验证页
#[handler]
async fn wechat_mp(req: &mut Request, res: &mut Response) {
    if let (Some(signature), Some(timestamp), Some(nonce), Some(echostr)) = (
        req.query::<String>("signature"),
        req.query::<String>("timestamp"),
        req.query::<String>("nonce"),
        req.query::<String>("echostr"),
    ) {
        if check_signature(&signature, &timestamp, &nonce) {
            res.status_code(StatusCode::OK);
            res.render(echostr);
            return;
        }
    }
    res.status_code(StatusCode::UNAUTHORIZED);
    res.render("Invalid signature");
}
//引导用户登录
#[handler]
async fn wechat_mp_login(req: &mut Request, res: &mut Response) {
    let state = req.query::<String>("state").or(Some(format!(""))).unwrap();
    if state.is_empty() {
        res.status_code(StatusCode::NOT_FOUND);
        return;
    }
    let mut redirect_uri = format!("");
    let wechat_token = wechat::WechatMPToken::load().unwrap();
    let url = wechat_token
        .get_redirect_login_url(redirect_uri, state)
        .await
        .expect("微信公众号参数异常");
    res.render(Redirect::found(url));
    // res.render("login");
}
//获得授权后跳到这里
#[handler]
async fn wechat_mp_home(req: &mut Request, res: &mut Response) {
    if let (Some(code), Some(state), Some(guid)) = (
        req.query::<String>("code"),
        req.query::<String>("state"),
        req.query::<String>("guid"),
    ) {
        let wechat_token = wechat::WechatMPToken::load().unwrap();
        let (user, host) = wechat_token.user_login(code).await.unwrap();
        //绑定
        if !guid.is_empty() {}
        //查询表里是否有openid
        let user_token = format!("");
        //没有的话还要再调用wechat_token.get_user();

        res.render(Redirect::found(format!(
            "{}/wechat/mp/home?t={}&token={}",
            host, state, user_token
        )));
        return;
    }
    res.status_code(StatusCode::NOT_FOUND);
}
