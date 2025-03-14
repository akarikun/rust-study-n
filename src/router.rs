use crate::commons::model::{
    SIO_GetIndexReq, SIO_GetStudyReq, SIO_PostStudyReq, SocketIO_Req, SocketIO_Resp,
};
use crate::dal::study;
use crate::entities;
use salvo::prelude::*;
use sea_orm::sqlx::types::chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::extract::{Data, SocketRef};
use socketioxide::SocketIo;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use salvo::http::StatusError;
use salvo::jwt_auth::{ConstDecoder, HeaderFinder};

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
    println!("io_study {:?}", data);

    let study_msg_resp = "study_msg_resp";
    let m = match serde_json::from_value::<SocketIO_Req>(data.clone()) {
        Ok(m) => m,
        Err(_) => return,
    };
    // dbg!(&m);
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
            println!("发送成功");
        }
    } else if m.msg == "get_study_list" {
        if let Some(data) = m.data {
            let data = match serde_json::from_value::<SIO_GetStudyReq>(data) {
                Ok(m) => m,
                Err(_) => return,
            };
            let res = match study::get_model(data.id).await {
                Some(m) => m,
                None => return,
            };
            match serde_json::from_value::<SIO_PostStudyReq>(res) {
                Ok(m) => {
                    _ = s.emit(
                        study_msg_resp,
                        SocketIO_Resp::<SIO_PostStudyReq> {
                            status: 1,
                            msg: msg_resp,
                            data: Some(m),
                        },
                    );
                }
                Err(_) => {
                    _ = s.emit(
                        study_msg_resp,
                        SocketIO_Resp::<String> {
                            status: 1,
                            msg: msg_resp,
                            data: Some(format!("")),
                        },
                    );
                }
            };
        }
    } else if m.msg == "post_study" {
        if let Some(data) = m.data {
            let data = match serde_json::from_value::<SIO_PostStudyReq>(data) {
                Ok(m) => m,
                Err(_) => return,
            };
            let id = get_last_index(data.level).await;
            match study::insert(entities::study::Model {
                id: (id + 1) as i32, //SeaORM生成的实体类漏了设置自动增长，这里先手动处理
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
            })
            .await
            {
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
        }
    }
}

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    println!(
        "Socket.IO connected: {:?} {:?} {:?}",
        socket.ns(),
        socket.id,
        data
    );
    if data["token"] == "test" {
        socket.on(
            "study_msg",
            move |s: SocketRef, data: Data<Value>| async move {
                io_study(&s, &data.0).await;
            },
        );
    } else {
        dbg!(&data);
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
        // .push(Router::with_path("/login.do").post(login))
        // .push(Router::with_hoop(auth_handler).path("check").get(check))
        .push(Router::with_path("/socket.io").hoop(layer).goal(hello))
}
