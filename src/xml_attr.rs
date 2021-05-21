use std::fmt::Display;

#[derive(Debug, Hash, Default)]
pub struct AttrData {
    key: String,
    value: String,
}

impl AttrData {
    pub fn new(key: &str, value: &str) -> AttrData {
        AttrData {
            key: String::from(key),
            value: String::from(value),
        }
    }

    pub fn get_key(&self) -> String {
        self.key.clone()
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
    pub fn get_key_ref(&self) -> &String {
        &self.key
    }
    pub fn get_value_ref(&self) -> &String {
        &self.value
    }
}

impl Display for AttrData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.key, self.value)
    }
}

#[cfg(test)]
mod test {
    use crate::xml_attr::AttrData;

    #[test]
    fn new_attr() {
        let attr = AttrData::new("a", "11");

        assert_eq!(attr.get_key(), "a");
        assert_eq!(attr.get_value(), "11");
    }
}
