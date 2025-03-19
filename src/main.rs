use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::static_embed;

mod commons;
mod dal;
mod entities;
mod router;
use commons::unitily::{init_env,init_logger};

#[derive(RustEmbed)]
#[folder = "web"]
struct Assets;

#[tokio::main]
async fn main() {
    init_env();
    init_logger();    
    let router = Router::new()
        .push(router::config_router())
        .push(Router::with_path("{**path}").get(static_embed::<Assets>().fallback("index.html")));
    let host = std::env::var("HOST").unwrap();
    let acceptor = TcpListener::new(host.clone()).bind().await;
    println!("http://{}", host);
    Server::new(acceptor).serve(router).await;
}
