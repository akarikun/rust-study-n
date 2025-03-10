use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SocketIO_Req {
    pub msg: String,
    pub data: Option<Value>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SocketIO_Resp<T> {
    pub status: i32,
    pub msg: String,
    pub data: Option<T>,
}


#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SIO_GetIndexReq {
    pub level:i32,
    pub index:i32,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct SIO_PostStudyReq {
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
