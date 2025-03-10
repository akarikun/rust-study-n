use crate::commons::model::{SIO_GetIndexReq, SIO_PostStudyReq, SocketIO_Req, SocketIO_Resp};
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

async fn io_study(s: &SocketRef, data: &Value) {
    println!("io_study {:?}", data);

    let study_msg_resp = "study_msg_resp";
    let m = match serde_json::from_value::<SocketIO_Req>(data.clone()) {
        Ok(m) => m,
        Err(_) => return,
    };
    dbg!(&m);
    if m.msg == "get_last_index" {
        if let Some(data) = m.data {
            let data = match serde_json::from_value::<SIO_GetIndexReq>(data) {
                Ok(m) => m,
                Err(_) => return,
            };

            let list = match study::get_list(data.index, data.level).await {
                Ok(m) => m,
                Err(_) => return,
            };
            let mut index = 0;
            if list.len() > 0 {
                index = list.last().unwrap().as_i64().unwrap();
            }
            _ = s.emit(
                study_msg_resp,
                SocketIO_Resp::<i64> {
                    status: 1,
                    msg: format!(""),
                    data: Some(index),
                },
            );
            println!("发送成功");
        }
    } else if m.msg == "post_study" {
    }

    // println!("{:?}", m);
    // if m.is_ok() {
    //     // let m = m.unwrap();
    //     // _ = study::insert(entities::study::Model {
    //     //     id: 0,
    //     //     level: m.level,
    //     //     index: m.index,
    //     //     content: m.content,
    //     //     a: m.a,
    //     //     b: m.b,
    //     //     c: m.c,
    //     //     d: m.d,
    //     //     remark: m.remark,
    //     //     result: m.result,
    //     //     r#type: m.r#type,
    //     //     create_date: Local::now().naive_local(),
    //     // })
    //     // .await;
    //     // _ = s.emit(
    //     //     "post_resp",
    //     //     SocketIO_Resp {
    //     //         status: 1,
    //     //         msg: format!("提交成功"),
    //     //         data: None,
    //     //     },
    //     // );
    // } else {
    //     // _ = s.emit(
    //     //     "post_study_resp",
    //     //     SocketIO_Resp {
    //     //         status: 0,
    //     //         msg: format!("参数异常001"),
    //     //         data: None,
    //     //     },
    //     // );
    // }
}
fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    socket.on(
        "study_msg",
        move |s: SocketRef, data: Data<Value>| async move {
            io_study(&s, &data.0).await;
        },
    );
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
        .push(Router::with_path("/login.do").post(login))
        .push(Router::with_hoop(auth_handler).path("check").get(check))
        .push(Router::with_path("/socket.io").hoop(layer).goal(hello))
}
