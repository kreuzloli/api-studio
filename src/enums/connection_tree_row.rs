/// 侧栏的一行，索引指向原始目录或连接列表；depth 用于计算缩进。
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ConnectionTreeRow {
    Folder {
        folder_index: usize,
        depth: usize,
    },
    Connection {
        connection_index: usize,
        depth: usize,
    },
}
