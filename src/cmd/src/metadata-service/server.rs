use clap::Parser;
use common_base::config::placement_center::{
    init_placement_center_conf_by_path, init_placement_center_log, placement_center_conf,
};
use log::info;
use placement_center::server::http::server::start_server;
// 定义默认的配置路径，即当命令行没传配置路径时，默认的配置文件路径
pub const DEFAULT_PLACEMENT_CENTER_CONFIG: &str = "config/placement-center.toml";

// 定义接收哪些参数
#[derive(Parser, Debug)]
#[command(about="Placement Center Server", long_about = None)]
#[command(next_line_help = true)]
struct ArgsParams {
    #[arg(short, long, default_value_t=String::from(DEFAULT_PLACEMENT_CENTER_CONFIG))]
    conf: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 解析命令行参数
    let args = ArgsParams::parse();
    init_placement_center_conf_by_path(&args.conf);
    // 初始化日志
    init_placement_center_log();

    let conf = placement_center_conf(); // 记录日志
    info!("{:?}", conf);
    // start_server();
    start_server().await;
    Ok(())
}
