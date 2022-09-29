use std::fs;

use json::{parse, JsonValue};

#[derive(Debug)]
pub struct Parameters {
    pub translationUnits: Vec<String>,
    pub includeDirs: Vec<String>,
    pub includeSystemDirs: Vec<String>,
}

impl Parameters {
    pub const fn new() -> Self {
        Self {
            translationUnits: Vec::new(),
            includeDirs: Vec::new(),
            includeSystemDirs: Vec::new(),
        }
    }

    pub fn new_file(file: &str) -> Result<Self, String> {
        let contents = fs::read_to_string(file).map_err(|x| x.to_string())?;
        return Self::new().parse(contents);
    }

    fn parse(mut self, contents: String) -> Result<Self, String> {
        let parsing = parse(contents.as_str()).map_err(|x| x.to_string())?;
        if let JsonValue::Object(obj) = parsing {
            for (key, value) in obj.iter() {
                match key {
                    "translationUnits" => {
                        self.translationUnits = Self::parseStringArray(value, "translationUnits")?;
                    }
                    "includeDirs" => {
                        self.translationUnits = Self::parseStringArray(value, "includeDirs")?;
                    }
                    "includeSystemDirs" => {
                        self.translationUnits = Self::parseStringArray(value, "includeSystemDirs")?;
                    }
                    _ => {}
                }
            }
        } else {
            return Err("Invalid JSON Paramater: Missing object body".to_string());
        }
        Ok(self)
    }

    fn parseStringArray(value: &JsonValue, name: &str) -> Result<Vec<String>, String> {
        let mut res = vec![];
        if let JsonValue::Array(arr) = value {
            for val in arr {
                if let JsonValue::String(str) = val {
                    res.push(str.clone());
                } else if let JsonValue::Short(str) = val {
                    res.push(str.to_string());
                } else {
                    return Err(format!("Invalid value for {}: {:?}", name, val));
                }
            }
        } else {
            return Err(format!("Invalid value for {}: {:?}", name, value));
        }
        Ok(res)
    }
}
