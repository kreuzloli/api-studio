//! 外部连接配置导入。

#[cfg(test)]
mod tests {
    use quick_xml::{events::Event, reader::Reader};

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
}
