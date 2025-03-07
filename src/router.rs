use salvo::{handler, Router};

#[handler]
pub fn login(){
    
}

pub fn set_router()->Router{
    Router::new().push(Router::with_path("/login.do").post(login))
}