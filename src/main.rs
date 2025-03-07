use salvo::http::StatusError;
use salvo::jwt_auth::{ConstDecoder, HeaderFinder};
use salvo::prelude::*;
use salvo::serve_static::static_embed;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use socketioxide::{
    extract::{AckSender, Bin, Data, SocketRef},
    SocketIo,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use rust_embed::RustEmbed;

mod entities;
mod router;
mod commons;
use commons::unitily;

const SECRET_KEY: &str = "YOUR_SECRET_KEY";

#[derive(RustEmbed)]
#[folder = "web"]
struct Assets;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    sid: String,
    sub: String,
    exp: i64,
}

fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    println!(
        "Socket.IO connected: {:?} {:?} {:?}",
        socket.ns(),
        socket.id,
        data
    );
    socket.on("login", |s: SocketRef, Data::<Value>(data)| {
        println!("login {:?}", data);
        s.broadcast().emit("login", "login").ok();
        s.emit("login", data).ok();
    });
    socket.on("add user", |s: SocketRef, Data::<Value>(data)| {
        println!("add user {:?}", data);
        s.broadcast().emit("user joined", "111").ok();
        s.emit("add user", data).ok();
    });
}

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    unitily::init_env();

    let (layer, io) = SocketIo::new_layer();
    let layer = ServiceBuilder::new()
        .layer(CorsLayer::permissive())
        .layer(layer);

    io.ns("/", on_connect);
    let layer = layer.compat();


    let auth_handler: JwtAuth<JwtClaims, ConstDecoder> = JwtAuth::new(ConstDecoder::from_secret(SECRET_KEY.as_bytes()))
        .finders(vec![Box::new(HeaderFinder::new())])
        .force_passed(true);
        
    let router = Router::new()
        .push(Router::with_path("/socket.io").hoop(layer).goal(hello))
        .push(Router::with_path("/").get(static_embed::<Assets>().fallback("index.html")))
        .push(router::set_router())
        .push(Router::with_hoop(auth_handler).path("check").get(check))
        .push(Router::with_path("<**path>").get(static_embed::<Assets>()));
    let host = std::env::var("HOST").unwrap();
    let acceptor = TcpListener::new(host.clone()).bind().await;
    println!("http://{}",host);
    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn check(depot: &mut Depot) -> Result<String,StatusError>  {
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
