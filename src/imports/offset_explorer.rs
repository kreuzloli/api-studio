//! 外部连接配置导入。

use quick_xml::{
    Reader, XmlVersion,
    events::{BytesStart, Event},
};

use crate::{
    enums::ImportError,
    models::{ConnectionImport, ImportedConnection, ImportedFolder},
};

/// 检查必填属性；只判断空白，不修改原始有效文本。
fn required_attribute(
    value: Option<String>,
    attribute: &'static str,
) -> Result<String, ImportError> {
    let value = value.ok_or(ImportError::MissingAttribute(attribute))?;
    if value.trim().is_empty() {
        return Err(ImportError::EmptyAttribute(attribute));
    }
    Ok(value)
}

/// 读取 connection 标签的基本属性，并检查名称和地址是否缺失或为空。
pub(crate) fn parse_connection(
    element: &BytesStart<'_>,
    version: XmlVersion,
) -> Result<ImportedConnection, ImportError> {
    let mut conn = ImportedConnection::new();
    let mut name = None::<String>;
    let mut bootstrap_servers = None::<String>;

    for attr in element.attributes() {
        let attr = attr.map_err(quick_xml::Error::from)?;
        let value = attr.normalized_value(version)?.into_owned();
        match attr.key.as_ref() {
            "name" => name = Some(value),
            "bootstrap_servers" => bootstrap_servers = Some(value),
            "groupId" => conn.folder_external_id = Some(value),
            "group" => conn.folder_name_hint = Some(value),
            _ => {}
        }
    }
    conn.name = required_attribute(name, "name")?;
    conn.bootstrap_servers = required_attribute(bootstrap_servers, "bootstrap_servers")?;
    Ok(conn)
}

/// 读取目录属性，父目录由遍历 XML 的调用方提供。
fn parse_folder(
    element: &BytesStart<'_>,
    version: XmlVersion,
    parent_external_id: Option<String>,
) -> Result<ImportedFolder, ImportError> {
    let mut folder = ImportedFolder::new();
    let mut external_id = None::<String>;
    let mut name = None::<String>;
    for attr in element.attributes() {
        let attr = attr.map_err(quick_xml::Error::from)?;
        let value = attr.normalized_value(version)?.into_owned();
        match attr.key.as_ref() {
            "id" => external_id = Some(value),
            "name" => name = Some(value),
            _ => {}
        }
    }
    folder.external_id = required_attribute(external_id, "id")?;
    folder.name = required_attribute(name, "name")?;
    folder.parent_external_id = parent_external_id;
    Ok(folder)
}

/// 收集连接与目录记录；文件结构和引用完整性校验将在后续补充。
fn parse_import(xml: &str) -> Result<ConnectionImport, ImportError> {
    let mut reader = Reader::from_str(xml);
    let mut version = XmlVersion::Implicit1_0;
    let mut folder_stack = Vec::<String>::new();
    let mut res = ConnectionImport::new();
    loop {
        match reader.read_event()? {
            Event::Start(e) if e.name().as_ref() == "group" => {
                let folder = parse_folder(&e, version, folder_stack.last().cloned())?;
                folder_stack.push(folder.external_id.clone());
                res.folders.push(folder);
            }
            Event::Empty(e) if e.name().as_ref() == "group" => {
                let folder = parse_folder(&e, version, folder_stack.last().cloned())?;
                res.folders.push(folder);
            }
            Event::End(e) if e.name().as_ref() == "group" => {
                folder_stack.pop();
            }
            Event::Start(e) | Event::Empty(e) if e.name().as_ref() == "connection" => {
                let conn = parse_connection(&e, version)?;
                res.connections.push(conn);
            }
            Event::Decl(declaration) => {
                version = declaration.xml_version()?;
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(res)
}

#[cfg(test)]
mod tests {

    use crate::{
        enums::ImportError,
        imports::offset_explorer::{parse_folder, parse_import},
        models::ImportedFolder,
    };

    use super::parse_connection;
    use quick_xml::{XmlVersion, events::Event, reader::Reader};

    #[test]
    fn reads_basic_xml_events() -> Result<(), quick_xml::Error> {
        let xml = include_str!("../../tests/fixtures/offset_explorer_basic.xml");
        let mut reader = Reader::from_str(xml);
        let mut tags = Vec::<(&str, String)>::new();
        loop {
            match reader.read_event()? {
                Event::Start(e) => {
                    tags.push(("start", e.name().as_ref().to_owned()));
                }
                Event::Empty(e) => {
                    tags.push(("empty", e.name().as_ref().to_owned()));
                }
                Event::End(e) => {
                    tags.push(("end", e.name().as_ref().to_owned()));
                }
                Event::Eof => break,
                _ => {}
            }
        }
        assert_eq!(
            tags,
            vec![
                ("start", "connections".to_owned()),
                ("empty", "connection".to_owned()),
                ("start", "groups".to_owned()),
                ("start", "group".to_owned()),
                ("empty", "group".to_owned()),
                ("end", "group".to_owned()),
                ("empty", "group".to_owned()),
                ("end", "groups".to_owned()),
                ("end", "connections".to_owned()),
            ]
        );
        Ok(())
    }

    #[test]
    fn reads_connection_name_attribute() -> Result<(), quick_xml::Error> {
        let mut reader = Reader::from_str(r#"<connection name="Local &amp; Demo" />"#);
        let mut name = None::<String>;
        if let Event::Empty(e) = reader.read_event()? {
            assert_eq!(e.name().as_ref(), "connection");
            for attr in e.attributes() {
                let attr = attr?;
                if attr.key.as_ref() == "name" {
                    // Implicit1_0 表示这段没有 XML 声明的样例按 XML 1.0 处理。
                    // into_owned() 将返回的 Cow<str> 转为自有的 String；需要时复制，已经拥有字符串时直接取出。
                    name = Some(attr.normalized_value(XmlVersion::Implicit1_0)?.into_owned());
                }
            }
        }
        assert_eq!(name.as_deref(), Some("Local & Demo"));
        Ok(())
    }

    #[test]
    fn reads_connection_attributes() -> Result<(), ImportError> {
        let mut reader = Reader::from_str(
            r#"<connection
                name="Local &amp; Demo"
                bootstrap_servers="localhost:9092"
                group="Local"
                groupId="2"
            />"#,
        );
        let Event::Empty(element) = reader.read_event()? else {
            panic!("Expected an empty connection element");
        };
        assert_eq!(element.name().as_ref(), "connection");
        let connection = parse_connection(&element, XmlVersion::Implicit1_0)?;
        assert_eq!(connection.name, "Local & Demo");
        assert_eq!(connection.bootstrap_servers, "localhost:9092");
        assert_eq!(connection.folder_external_id.as_deref(), Some("2"));
        assert_eq!(connection.folder_name_hint.as_deref(), Some("Local"));
        Ok(())
    }

    #[test]
    fn rejects_missing_or_empty_connection_name() -> Result<(), ImportError> {
        let cases = [
            (r#"<connection bootstrap_servers="localhost:9092" />"#, true),
            (
                r#"<connection name="" bootstrap_servers="localhost:9092" />"#,
                false,
            ),
            (
                r#"<connection name="   " bootstrap_servers="localhost:9092" />"#,
                false,
            ),
        ];

        for (xml, missing) in cases {
            let mut reader = Reader::from_str(xml);
            let Event::Empty(e) = reader.read_event()? else {
                panic!("Expected an empty connection element");
            };
            let conn = parse_connection(&e, XmlVersion::Implicit1_0);
            if missing {
                assert!(matches!(conn, Err(ImportError::MissingAttribute("name"))));
            } else {
                assert!(matches!(conn, Err(ImportError::EmptyAttribute("name"))));
            }
        }
        Ok(())
    }

    #[test]
    fn reads_folder_attributes() -> Result<(), ImportError> {
        let mut reader = Reader::from_str(r#"<group id="2" name="Local &amp; Dev" />"#);
        let Event::Empty(e) = reader.read_event()? else {
            panic!("Expected an empty folder element");
        };
        let folder = parse_folder(&e, XmlVersion::Implicit1_0, Some("1".to_owned()))?;
        assert_eq!(folder.external_id, "2");
        assert_eq!(folder.name, "Local & Dev");
        assert_eq!(folder.parent_external_id.as_deref(), Some("1"));
        Ok(())
    }

    #[test]
    fn reads_nested_folders() -> Result<(), ImportError> {
        let xml = r#"
                <groups>
                    <group id="1" name="Development">
                        <group id="2" name="Local" />
                    </group>
                    <group id="3" name="Empty" />
                </groups>
            "#;
        let mut reader = Reader::from_str(xml);
        let mut folder_stack = Vec::<String>::new();
        let mut folders = Vec::<ImportedFolder>::new();
        loop {
            match reader.read_event()? {
                Event::Start(e) if e.name().as_ref() == "group" => {
                    let folder =
                        parse_folder(&e, XmlVersion::Implicit1_0, folder_stack.last().cloned())?;
                    folder_stack.push(folder.external_id.clone());
                    folders.push(folder);
                }
                Event::Empty(e) if e.name().as_ref() == "group" => {
                    let folder =
                        parse_folder(&e, XmlVersion::Implicit1_0, folder_stack.last().cloned())?;
                    folders.push(folder);
                }
                Event::End(e) if e.name().as_ref() == "group" => {
                    folder_stack.pop();
                }
                Event::Eof => break,
                _ => {}
            }
        }
        assert_eq!(folders.len(), 3);

        assert_eq!(folders[0].external_id, "1");
        assert_eq!(folders[0].parent_external_id, None);

        assert_eq!(folders[1].external_id, "2");
        assert_eq!(folders[1].parent_external_id.as_deref(), Some("1"));

        assert_eq!(folders[2].external_id, "3");
        assert_eq!(folders[2].parent_external_id, None);

        assert!(folder_stack.is_empty());
        Ok(())
    }

    #[test]
    fn reads_basic_import() -> Result<(), ImportError> {
        let xml = include_str!("../../tests/fixtures/offset_explorer_basic.xml");
        let res = parse_import(xml)?;
        assert_eq!(res.connections.len(), 1);
        assert_eq!(res.folders.len(), 3);

        assert_eq!(res.connections[0].name, "Local Demo");
        assert_eq!(res.connections[0].folder_external_id.as_deref(), Some("2"));

        assert_eq!(res.folders[1].external_id, "2");
        assert_eq!(res.folders[1].parent_external_id.as_deref(), Some("1"));
        assert_eq!(res.folders[2].name, "Empty");
        assert_eq!(res.folders[2].parent_external_id, None);
        Ok(())
    }

    #[test]
    fn rejects_unknown_xml_version() {
        let xml = r#"<?xml version="9.9"?><connections />"#;
        assert!(matches!(parse_import(xml), Err(ImportError::Xml(_))));
    }
}
