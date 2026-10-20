/// 本地连接配置；保存名称和地址不代表已经建立网络连接。
#[derive(Debug)]
pub(crate) struct Connection {
    pub(crate) name: String,
    pub(crate) bootstrap_servers: String,
}
