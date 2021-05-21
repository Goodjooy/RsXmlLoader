use std::fmt::Display;

use crate::xml::XmlStruct;
use crate::xml_attr::AttrData;

pub enum InsideData {
    Attr(AttrData),
    Inside(XmlStruct),
}

impl Display for InsideData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsideData::Attr(attr) => {
                write!(f, "{}", attr)
            }
            InsideData::Inside(xml) => {
                write!(f, "{}", xml)
            }
        }
    }
}
