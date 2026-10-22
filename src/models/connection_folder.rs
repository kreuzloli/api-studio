/// 本机连接目录的标识，与 Kafka 消费组 ID 属于不同概念。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ConnectionFolderId(pub(crate) String);

/// 连接目录；没有父目录时位于根层级。
#[derive(Debug)]
pub(crate) struct ConnectionFolder {
    pub(crate) id: ConnectionFolderId,
    pub(crate) name: String,
    pub(crate) parent_id: Option<ConnectionFolderId>,
}

#[cfg(test)]
mod tests {
    use super::{ConnectionFolder, ConnectionFolderId};

    #[test]
    fn child_folder_references_parent() {
        let root = ConnectionFolder {
            id: ConnectionFolderId("development".to_owned()),
            name: "Development".to_owned(),
            parent_id: None,
        };

        let child = ConnectionFolder {
            id: ConnectionFolderId("local".to_owned()),
            name: "Local".to_owned(),
            parent_id: Some(root.id.clone()),
        };

        assert!(root.parent_id.is_none());
        assert_eq!(child.parent_id.as_ref(), Some(&root.id));
        assert_eq!(child.name, "Local");
    }
}
