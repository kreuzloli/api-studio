/// 区分 XML 解析失败、必填属性缺失和属性值为空。
#[derive(Debug)]
pub(crate) enum ImportError {
    Xml(quick_xml::Error),
    MissingAttribute(&'static str),
    EmptyAttribute(&'static str),
}

impl From<quick_xml::Error> for ImportError {
    fn from(err: quick_xml::Error) -> Self {
        Self::Xml(err)
    }
}
