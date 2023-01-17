//! Parsing of the input config file.
use std::fs;

use json::{parse, JsonValue};

#[derive(Debug, Clone)]
/// The parsed config file.
pub struct Parameters {
    /// The path to the input files.
    pub translationUnits: Vec<String>,
    /// System Include paths.
    pub moduleHeaderUnits: Vec<String>,
    /// Include paths.
    pub includeDirs: Vec<String>,
    /// System Include paths.
    pub includeSystemDirs: Vec<String>,
    pub threadNum: Option<usize>,
}

impl Parameters {
    /// new Parameters.
    pub const fn new() -> Self {
        Self {
            translationUnits: Vec::new(),
            includeDirs: Vec::new(),
            includeSystemDirs: Vec::new(),
            moduleHeaderUnits: Vec::new(),
            threadNum: None,
        }
    }

    /// Parses the config file, and returns the results.
    pub fn new_file(file: &str) -> Result<Self, String> {
        let contents = fs::read_to_string(file).map_err(|x| x.to_string())?;
        Self::new().parse(&contents)
    }

    /// Parses the config file.
    fn parse(mut self, contents: &str) -> Result<Self, String> {
        let parsing = parse(contents).map_err(|x| x.to_string())?;
        if let JsonValue::Object(obj) = parsing {
            for (key, value) in obj.iter() {
                match key {
                    "translationUnits" => {
                        self.translationUnits = Self::parseStringArray(value, "translationUnits")?;
                    }
                    "includeDirs" => {
                        self.includeDirs = Self::parseStringArray(value, "includeDirs")?;
                    }
                    "includeSystemDirs" => {
                        self.includeSystemDirs =
                            Self::parseStringArray(value, "includeSystemDirs")?;
                    }
                    "moduleHeaderUnits" => {
                        self.moduleHeaderUnits =
                            Self::parseStringArray(value, "includeSystemDirs")?;
                    }
                    "threadNum" => {
                        if let JsonValue::Number(num) = value {
                            self.threadNum =
                                Some(num.as_fixed_point_u64(0).unwrap().try_into().unwrap());
                        } else {
                            return Err(
                                "Invalid JSON Paramater: threadNum must be a number".to_string()
                            );
                        }
                    }
                    _ => {}
                }
            }
        } else {
            return Err("Invalid JSON Paramater: Missing object body".to_string());
        }
        Ok(self)
    }

    /// Parse a vector of strings. uses the name for error reporting only.
    fn parseStringArray(value: &JsonValue, name: &str) -> Result<Vec<String>, String> {
        let mut res = vec![];
        if let JsonValue::Array(arr) = value {
            for val in arr {
                if let JsonValue::String(str) = val {
                    res.push(str.clone());
                } else if let JsonValue::Short(str) = val {
                    res.push(str.to_string());
                } else {
                    return Err(format!("Invalid value for {name}: {val:?}"));
                }
            }
        } else {
            return Err(format!("Invalid value for {name}: {value:?}"));
        }
        Ok(res)
    }
}
