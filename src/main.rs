use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::static_embed;

mod commons;
mod dal;
mod entities;
mod router;
use commons::unitily::{init_logger, CONFIG as config};

#[derive(RustEmbed)]
#[folder = "web"]
struct Assets;

#[tokio::main]
async fn main() {
    init_logger();
    let router = Router::new()
        .push(router::config_router())
        .push(Router::with_path("{**path}").get(static_embed::<Assets>().fallback("index.html")));
    let host = config.clone().host;
    println!("http://{}", host);
    let acceptor = TcpListener::new(host).bind().await;
    Server::new(acceptor).serve(router).await;
}
