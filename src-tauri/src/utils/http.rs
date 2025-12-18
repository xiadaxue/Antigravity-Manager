use reqwest::{Client, Proxy};
use crate::modules::config::load_app_config;

/// 创建统一配置的 HTTP 客户端
/// 自动应用全局上游代理设置
pub fn create_client(timeout_secs: u64) -> Client {
    let mut builder = Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_secs));

    // 尝试加载配置以获取代理设置
    if let Ok(config) = load_app_config() {
        if config.proxy.upstream_proxy.enabled && !config.proxy.upstream_proxy.url.is_empty() {
            match Proxy::all(&config.proxy.upstream_proxy.url) {
                Ok(proxy) => {
                    builder = builder.proxy(proxy);
                    tracing::info!("HTTP 客户端已启用上游代理: {}", config.proxy.upstream_proxy.url);
                }
                Err(e) => {
                    tracing::error!("无效的代理地址: {}, 错误: {}", config.proxy.upstream_proxy.url, e);
                }
            }
        }
    }

    builder.build().unwrap_or_else(|_| Client::new())
}
