

use clap::command;
use clap::Parser;
use common_base::config::placement_center::{
    init_placement_center_conf_by_path,
    placement_center_conf,
};
use common_base::log::placement_center::init_placement_center_log;
use log::info;
use tokio::sync::broadcast;
use placement_center::start_server;

// 定义默认的配置路径，即当命令行没传配置路径时，默认的配置文件路径
pub const DEFAULT_PLACEMENT_CENTER_CONFIG: &str = "config/placement-center.toml";


// 定义接收哪些参数
#[derive(Parser, Debug)]
#[command(author="robustmq-geek", version="0.0.1", about=" RobustMQ: Next generation cloud-native converged high-performance message queue.", long_about = None)]
#[command(next_line_help = true)]
struct ArgsParams {
    #[arg(short, long, default_value_t=String::from(DEFAULT_PLACEMENT_CENTER_CONFIG))]
    conf: String,
}


#[tokio::main]
async fn main() {
    // 解析命令行参数
    let args = ArgsParams::parse();
    init_placement_center_conf_by_path(&args.conf);
    init_placement_center_log();

    let conf = placement_center_conf();
    info!("{:?}", conf);

    let (stop_send, _) = broadcast::channel(2);
    start_server(stop_send).await;
}