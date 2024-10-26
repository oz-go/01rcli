use axum::{http::StatusCode,extract::{Path, State}};
use tokio::net::TcpListener;
use tracing::{info, warn};
use std::path::PathBuf;
use axum::{serve, Router,routing::get};
use std::net::SocketAddr;
use std::sync::Arc;
use anyhow::Result;

#[derive(Debug)]
struct HttpServeState{
    path:PathBuf,
}

pub async fn process_http_serve(path:PathBuf, port:u16)-> Result<()>{
    let addr = SocketAddr::from(([0,0,0,0],port));
    info!("Serving {:?} on  {}",path,addr);
    let state = HttpServeState{path};
    let router = Router::new()
        .route("/*path", get(file_handler).with_state(Arc::new(state)));

    
    let listener = TcpListener::bind(addr).await?;
    
    serve(listener,router).await?;

    Ok(())
}

async fn file_handler(
    State(state):State<Arc<HttpServeState>>,
    Path(path):Path<String>,
    )->(StatusCode,String){
    
    let p = std::path::Path::new(&state.path).join(path);
    info!("Reading file {:?}",p);
    if !p.exists(){
        (StatusCode::NOT_FOUND,
        format!("File {} not found",p.display()))
    }else{
        match tokio::fs::read_to_string(p).await{
            Ok(content) => {
                info!("Read {} bytes",content.len());
                (StatusCode::OK,content)
            }
            Err(e)=>{
                warn!("Error reading file: {:?}",e);
                (StatusCode::INTERNAL_SERVER_ERROR,e.to_string())
            }
        }
    }
}