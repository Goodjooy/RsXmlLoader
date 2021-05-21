use std::{collections::HashMap, fmt::Display, slice::Iter, str};

use regex::{Captures, Regex};

use crate::{
    xml_attr::AttrData, xml_inside::InsideData, XML_ATTR_DATA_PATTERN, XML_PLAIN_DATA_PATTERN,
    XML_STRUCT_CLOSE_PATTERN, XML_STRUCT_HEAD_PATTERN,
};

#[derive(Default)]
struct XmlStatus(bool);
pub struct XmlStruct {
    tag: String,
    //inside data
    insides_sign: Vec<String>,
    insides_data: HashMap<String, InsideData>,

    //contain string
    data: String,

    status: XmlStatus,
}

impl XmlStruct {
    pub fn new(tag: &str) -> XmlStruct {
        XmlStruct {
            tag: String::from(tag),
            insides_sign: Vec::new(),
            insides_data: HashMap::new(),

            data: String::new(),
            status: XmlStatus::default(),
        }
    }

    pub fn from_str(data: String) -> Option<(XmlStruct, String)> {
        let struct_head_pattern = Regex::new(XML_STRUCT_HEAD_PATTERN).unwrap();

        let mut remind_str = &data[..];

        match struct_head_pattern.captures(remind_str) {
            Some(match_data) => {
                let match_size = match_data.get(0).unwrap().as_str().len();

                let tag_name = match_data.get(1).unwrap().as_str();
                let attr_data = match_data.get(2).unwrap().as_str();
                let close_sign = match_data.get(3).unwrap().as_str();

                let mut xml = XmlStruct::new(tag_name);

                //load attr data
                if attr_data.len() != 0 {
                    xml.load_attr_data(attr_data);
                }
                remind_str = &remind_str[match_size..];
                //添加捕获索引
                match close_sign.len() != 0 {
                    true => {
                        xml.status.0 = true;
                    }
                    false => {
                        remind_str = xml.no_close_sign_handle(remind_str);
                    }
                }
                Some((xml, String::from(remind_str)))
            }
            None => None,
        }
    }
}

impl XmlStruct {
    pub fn get_tag(&self) -> &str {
        self.tag.as_str()
    }
    pub fn get_inside_iter_name(&self) -> Iter<String> {
        self.insides_sign.iter()
    }

    pub fn get_inside_data(&self, tag_name: &str) -> Option<&InsideData> {
        match self.insides_data.get(tag_name) {
            Some(data) => Some(data),
            None => None,
        }
    }
    pub fn get_plain_data(&self) -> Option<&String> {
        if self.data.is_empty() {
            None
        } else {
            Some(&self.data)
        }
    }
    pub fn is_close_tag(&self) -> bool {
        self.status.0
    }

    fn append_inside_data(&mut self, data: InsideData) -> Result<(), String> {
        match &data {
            InsideData::Attr(attr_data) => {
                if self.insides_sign.contains(attr_data.get_key_ref()) {
                    Err("inside data already exist".to_string())
                } else {
                    self.insides_sign.push(attr_data.get_key());
                    self.insides_data.insert(attr_data.get_key(), data);
                    Ok(())
                }
            }
            InsideData::Inside(inside_data) => {
                if self.insides_sign.contains(&inside_data.tag) {
                    Err("indside Data Tag Already Exist".to_string())
                } else {
                    self.insides_sign.push(inside_data.tag.clone());
                    self.insides_data.insert(inside_data.tag.clone(), data);

                    Ok(())
                }
            }
        }
    }

    fn set_plain_data(&mut self, data: &str) {
        self.data = String::from(data);
    }
    fn set_tag(&mut self, tag: &str) {
        self.tag = String::from(tag);
    }
}

impl XmlStruct {
    fn load_inside_data<'a>(&mut self, remind_str: &'a str) -> &'a str {
        let plain_pattern = Regex::new(XML_PLAIN_DATA_PATTERN).unwrap();

        match plain_pattern.captures(remind_str) {
            Some(match_data) => {
                let match_size = match_data.get(0).unwrap().as_str().len();
                let plain_data = match_data.get(1).unwrap().as_str();

                self.set_plain_data(plain_data);
                &remind_str[match_size..]
            }
            None => {
                let (inside_xml, r_str) = XmlStruct::from_str(String::from(remind_str))
                    .expect("Failure To Load Sub Xml Struct");

                self.append_inside_data(InsideData::Inside(inside_xml))
                    .unwrap();
                &remind_str[remind_str.len() - r_str.len()..]
            }
        }
    }

    fn check_close_tag<'a>(&mut self, match_data: Captures, remind_str: &'a str) -> &'a str {
        let match_len = match_data.get(0).unwrap().as_str().len();
        let tag_name = match_data.get(1).unwrap().as_str();

        match tag_name == self.tag.as_str() {
            true => self.status.0 = true,
            false => self.status.0 = false,
        }
        &remind_str[match_len..]
    }

    fn no_close_sign_handle<'a>(&mut self, r_str: &'a str) -> &'a str {
        let struct_close_pattern = Regex::new(XML_STRUCT_CLOSE_PATTERN).unwrap();

        let mut remind_str = &r_str[..];
        self.status.0 = false;
        //循环子结构
        while !self.status.0 {
            //是否为关闭标记
            match struct_close_pattern.captures(remind_str) {
                Some(match_data) => remind_str = self.check_close_tag(match_data, remind_str),
                None => remind_str = self.load_inside_data(remind_str),
            }
        }
        remind_str
    }

    fn load_attr_data(&mut self, attr_data: &str) {
        let xml_attr_pattern = Regex::new(XML_ATTR_DATA_PATTERN).unwrap();

        for attr in xml_attr_pattern.captures_iter(attr_data) {
            let key = attr.get(1).unwrap().as_str();
            let value = attr.get(2).unwrap().as_str();

            self.append_inside_data(InsideData::Attr(AttrData::new(key, value)))
                .unwrap();
        }
    }
}

impl Display for XmlStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}: \t{}\n", self.tag, self.data)?;
        for data in self.insides_sign.iter() {
            let value = self.insides_data.get(data).unwrap();
            write!(f, "\t{}\n", value)?;
        }

        write!(f, "\t]")
    }
}
