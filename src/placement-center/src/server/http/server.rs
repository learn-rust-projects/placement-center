use std::net::SocketAddr;

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use common_base::config::placement_center::placement_center_conf;
use log::info;
use tokio::{select, sync::broadcast};

use crate::server::http::{
    index::index, path_create, path_delete, path_list, path_update, v1_path,
};

pub const ROUTE_ROOT: &str = "/index";
pub const ROUTE_ADD_LEARNER: &str = "/add-learner";
pub const ROUTE_CHANGE_MEMBERSHIP: &str = "/change-membership";
pub const ROUTE_INIT: &str = "/init";
pub const ROUTE_METRICS: &str = "/metrics";
pub const ROUTE_SET: &str = "/set";
pub const ROUTE_GET: &str = "/get";

#[derive(Clone, Default)]
pub struct HttpServerState {}
// #[allow(dead_code)]
// impl HttpServerState {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
pub async fn start_server() {
    let (stop_sx, _) = broadcast::channel::<bool>(10);
    start_http_server(HttpServerState::default(), stop_sx).await;
}
pub async fn start_http_server(state: HttpServerState, stop_sx: broadcast::Sender<bool>) {
    // 读取配置
    let config = placement_center_conf();

    // 组装监听地址和端口
    let ip: SocketAddr = match format!("0.0.0.0:{}", config.http_port).parse() {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e);
        }
    };

    // 构建路由信息
    let app = routes(state);

    let mut stop_rx = stop_sx.subscribe();
    // 绑定端口，如果端口绑定失败，直接退出程序
    let listener = match tokio::net::TcpListener::bind(ip).await {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e);
        }
    };
    // 通过 select 来同时监听进程停止信号和 Server 运行
    select! {
        val = stop_rx.recv() =>{
            if let Ok(flag) = val
                && flag {
                    info!("HTTP Server stopped successfully");
                }
        },
        // 监听服务
        val = axum::serve(listener, app.clone())=>{
            match val{
                Ok(()) => {
                    // info!("HTTP Server started successfully, listening on port {}",config.http_port)
                },
                Err(e) => {
                    // HTTP 服务监听失败，直接退出程序
                    panic!("{}",e);
                }
            }
        }
    }
}

fn routes(state: HttpServerState) -> Router {
    // 定义不同的http path 路径被哪个服务处理
    let common = Router::new()
        .route(&v1_path(&path_list(ROUTE_ROOT)), get(index))
        .route(&v1_path(&path_create(ROUTE_ROOT)), post(index))
        .route(&v1_path(&path_update(ROUTE_ROOT)), put(index))
        .route(&v1_path(&path_delete(ROUTE_ROOT)), delete(index));

    let app = Router::new().merge(common);
    app.with_state(state)
}
