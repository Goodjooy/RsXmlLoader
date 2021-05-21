use std::{fmt::Display, slice::Iter};

use regex::Regex;

use crate::{xml::XmlStruct, xml_inside::InsideData, XML_ATTR_DATA_PATTERN, XML_META_PATTERN};

pub struct XmlRoot(XmlStruct, XmlMeta);

#[derive(Default)]
struct XmlMeta {
    version: String,
    encoding: String,
    stand_alone: bool,
}

impl XmlRoot {
    fn new() -> XmlRoot {
        XmlRoot(XmlStruct::new(""), XmlMeta::default())
    }

    fn load_meta_data(&mut self, data: String) -> String {
        let xml_meta_pattern = Regex::new(XML_META_PATTERN).unwrap();
        let xml_attr_pattern = Regex::new(XML_ATTR_DATA_PATTERN).unwrap();

        match xml_meta_pattern.captures(data.as_str()) {
            Some(match_data) => {
                let match_len = match_data.get(0).unwrap().as_str().len();
                let match_data = match_data.get(1).unwrap().as_str();

                for match_data in xml_attr_pattern.captures_iter(match_data) {
                    let key = match_data.get(1).unwrap().as_str();
                    let value = match_data.get(2).unwrap().as_str();

                    if key == "version" {
                        self.1.version = String::from(value);
                    } else if key == "encoding" {
                        self.1.encoding = String::from(value);
                    } else if key == "stand_alone" {
                        self.1.stand_alone = if value == "yes" { true } else { false };
                    }
                }
                data[match_len..].to_string()
            }
            None => data,
        }
    }

    fn load_xml_struct(&mut self, data: String) -> String {
        let xml = XmlStruct::from_str(data).expect("Failure Load Xml");
        self.0 = xml.0;
        xml.1
    }
}

impl XmlRoot {
    pub fn from_str(str: &str) -> Result<XmlRoot, String> {
        let mut xml = XmlRoot::new();

        let str = xml.load_meta_data(String::from(str));
        xml.load_xml_struct(str);

        Ok(xml)
    }
}

impl Display for XmlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "XML-DATA[version={}, encoding={}, stand alone={}]\nXML-STRUCT: {}",
            self.1.version, self.1.encoding, self.1.stand_alone, self.0
        )
    }
}

impl XmlRoot {
    pub fn get_tag(&self) -> &str {
        self.0.get_tag()
    }

    pub fn get_inside_data(&self, sign_name: &str) -> Option<&InsideData> {
        self.0.get_inside_data(sign_name)
    }

    pub fn get_inside_iter_name(&self) -> Iter<String> {
        self.0.get_inside_iter_name()
    }
    pub fn get_plain_data(&self) -> Option<&String> {
        self.0.get_plain_data()
    }

    pub fn is_close_tag(&self) -> bool {
        self.0.is_close_tag()
    }
}
