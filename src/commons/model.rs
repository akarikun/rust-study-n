use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct StudyN_Req {
    pub level: i32,
    pub index: i32,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SocketIO_Req {
    pub msg: String,
    pub arg: Option<serde_json::value::Value>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct SocketIO_Resp {
    pub status: i32,
    pub msg: String,
    pub data: Option<serde_json::value::Value>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Socket_IO_Study_N_Entity_Resp {
    pub level: i32,
    pub index: i32,
    pub content: String,
    pub a: String,
    pub b: String,
    pub c: String,
    pub d: String,
    pub remark: String,
    pub result: i32,
    pub r#type: i32,
}
