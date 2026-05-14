use std::{fs, sync::OnceLock};

use serde::Deserialize;

use crate::tool::{create_fold, file_exists, read_file};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct PlacementCenterConfig {
    #[serde(default = "default_node_id")]
    pub node_id: u32,
    #[serde(default = "default_grpc_port")]
    pub grpc_port: usize,
    #[serde(default)]
    pub log: LogConfig,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct LogConfig {
    pub log_config: String,
    pub log_path: String,
}

pub fn default_node_id() -> u32 {
    1
}

pub fn default_grpc_port() -> usize {
    9982
}

static PLACEMENT_CENTER_CONF: OnceLock<PlacementCenterConfig> = OnceLock::new();

pub fn init_placement_center_conf_by_path(config_path: &String) -> &'static PlacementCenterConfig {
    // n.b. static items do not call [`Drop`] on program termination, so if
    // [`DeepThought`] impls Drop, that will not be used for this instance.
    PLACEMENT_CENTER_CONF.get_or_init(|| {
        let content = fs::read_to_string(config_path).unwrap();
        toml::from_str(&content).unwrap()
    })
}

pub fn placement_center_conf() -> &'static PlacementCenterConfig {
    match PLACEMENT_CENTER_CONF.get() {
        Some(config) => config,
        None => {
            panic!(
                "Placement center configuration is not initialized, check the configuration file."
            );
        }
    }
}

pub fn init_placement_center_log() {
    // 1. 获取配置信息
    let conf = placement_center_conf();

    // 2. 检查日志配置 .yaml 文件是否存在
    if !file_exists(&conf.log.log_config) {
        panic!(
            "Logging configuration file {} does not exist",
            conf.log.log_config
        );
    }

    // 3.尝试初始化日志存放目录
    match create_fold(&conf.log.log_path) {
        Ok(()) => {}
        Err(_e) => {
            panic!("Failed to initialize log directory {}", conf.log.log_path);
        }
    }

    // 4. 读取日志配置.yaml 文件的内容
    let content = match read_file(&conf.log.log_config) {
        Ok(data) => data,
        Err(e) => {
            panic!("{}", e.to_string());
        }
    };

    // 5. 替换日志文件的存放路径
    let config_content = content.replace("{$path}", &conf.log.log_path);
    println!("log config: {}", config_content);

    // 6. 解析 yaml 格式的配置文件
    let config = match serde_yaml::from_str(&config_content) {
        Ok(data) => data,
        Err(e) => {
            panic!(
                "Failed to parse the contents of the config file {} with error message :{}",
                conf.log.log_config, e
            );
        }
    };

    // 7. 初始化日志配置
    match log4rs::init_raw_config(config) {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e.to_string());
        }
    }
}

mod tests {

    #[test]
    fn config_init_test() {
        let path = format!(
            "{}/../../../config/placement-center.toml",
            env!("CARGO_MANIFEST_DIR")
        );

        crate::config::placement_center::init_placement_center_conf_by_path(&path);
        let config = crate::config::placement_center::placement_center_conf();
        assert_eq!(config.node_id, 1);
        assert_eq!(config.grpc_port, 1228);
    }
}
