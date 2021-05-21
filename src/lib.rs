mod xml;
mod xml_attr;
mod xml_inside;
mod xml_root;

pub const XML_META_PATTERN: &str = r#"^<\?xml([^<>]+)\?>"#;

pub const XML_STRUCT_HEAD_PATTERN: &str = r#"^\s*<([^/<>\s]+)(.*?)(/?)>\s*"#;
pub const XML_STRUCT_CLOSE_PATTERN: &str = r#"^</([^<>\s/]+)>"#;

pub const XML_ATTR_DATA_PATTERN: &str = r#"([^<>\s/]+)=["']([^<>\s]*)["']"#;
pub const XML_PLAIN_DATA_PATTERN: &str = r#"^([^<>]+)"#;

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::xml_root::XmlRoot;

    #[test]
    fn test_lv1_xml() {
        let xml = XmlRoot::from_str("<msg uid=\"abab\" url=\"112123\">emm</msg>")
            .expect("Error to Load Xml");

        println!("{}", xml);

        assert_eq!(xml.get_tag(), "msg");
        assert_eq!(xml.is_close_tag(), true);

        assert_eq!(xml.get_inside_iter_name().len(), 2);
    }
    #[test]
    fn test_lv2_xml() {
        let xml = XmlRoot::from_str("<msg uid=\"abab\" url=\"112123\"><in a=\"12\">bb</in></msg>")
            .expect("Error to Load Xml");

        println!("{}", xml);

        assert_eq!(xml.get_tag(), "msg");
        assert_eq!(xml.is_close_tag(), true);

        assert_eq!(xml.get_inside_iter_name().len(), 3);
    }
    #[test]
    fn load_from_file() {
        let xmldata = fs::read_to_string("./xmls/example.xml").expect("Failure to open File");
        let xml = XmlRoot::from_str(xmldata.as_str());
        println!("{}", xml.unwrap());
    }
}
