use std::collections::HashMap;
use serde::{Deserialize};
use regex::Regex;
use anyhow::Result;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub identity: String,
    pub authorization: String,
    pub blade_auth: String,
    #[serde(with = "tuple_vec_map")]
    pub assignments: Vec<(String, String)>,
    pub sites: HashMap<String, String>,
}

pub struct AssignmentMap(Vec<(Regex, String)>);

impl AssignmentMap {
    pub fn new(assignments: &[(String, String)]) -> Result<AssignmentMap> {
        let mut vec = Vec::new();
        for (title, pattern) in assignments {
            vec.push((Regex::new(pattern)?, title.clone()));
        }
        Ok(AssignmentMap(vec))
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        for (regex, title) in &self.0 {
            if regex.is_match(name) {
                return Some(title);
            }
        }
        None
    }

    pub fn get_or_default<'a>(&'a self, name: &'a str) -> &'a str {
        self.get(name).unwrap_or(name)
    }
}
